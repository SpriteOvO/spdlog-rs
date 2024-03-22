use crate::{
    default_error_handler, default_thread_pool,
    formatter::Formatter,
    sink::{helper, OverflowPolicy, Sink, Sinks},
    sync::*,
    Error, ErrorHandler, LevelFilter, Record, RecordOwned, Result, ThreadPool,
};

/// A [combined sink], logging and flushing [asynchronously]
/// (thread-pool-based).
///
/// This sink sends `log` and `flush` operations to the inside thread pool for
/// asynchronous processing.
///
/// # Note
///
/// Errors that occur in `log` and `flush` will not be returned directly,
/// instead the error handler will be called.
///
/// # Examples
///
/// See [./examples] directory.
///
/// [combined sink]: index.html#combined-sink
/// [asynchronously]: index.html#asynchronous-combined-sink
/// [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/examples
// The names `AsyncSink` and `AsyncRuntimeSink` is reserved for future use.
pub struct AsyncPoolSink {
    level_filter: Atomic<LevelFilter>,
    overflow_policy: OverflowPolicy,
    thread_pool: Arc<ThreadPool>,
    backend: Arc<Backend>,
}

impl AsyncPoolSink {
    /// Constructs a builder of `AsyncPoolSink`.
    #[must_use]
    pub fn builder() -> AsyncPoolSinkBuilder {
        AsyncPoolSinkBuilder {
            level_filter: helper::SINK_DEFAULT_LEVEL_FILTER,
            overflow_policy: OverflowPolicy::Block,
            sinks: Sinks::new(),
            thread_pool: None,
            error_handler: None,
        }
    }

    /// Gets a reference to internal sinks in the combined sink.
    #[must_use]
    pub fn sinks(&self) -> &[Arc<dyn Sink>] {
        &self.backend.sinks
    }

    /// Sets a error handler.
    pub fn set_error_handler(&self, handler: Option<ErrorHandler>) {
        self.backend.error_handler.swap(handler, Ordering::Relaxed);
    }

    fn assign_task(&self, task: Task) -> Result<()> {
        self.thread_pool.assign_task(task, self.overflow_policy)
    }

    #[must_use]
    fn clone_backend(&self) -> Arc<Backend> {
        Arc::clone(&self.backend)
    }
}

impl Sink for AsyncPoolSink {
    fn log(&self, record: &Record) -> Result<()> {
        if self.should_log(record.level()) {
            self.assign_task(Task::Log {
                backend: self.clone_backend(),
                record: record.to_owned(),
            })?;
        }
        Ok(())
    }

    fn flush(&self) -> Result<()> {
        if crate::IS_TEARING_DOWN.load(Ordering::SeqCst) {
            // https://github.com/SpriteOvO/spdlog-rs/issues/64
            //
            // If the program is tearing down, this will be the final flush. `crossbeam`
            // uses thread-local internally, which is not supported in `atexit` callback.
            // This can be bypassed by flushing sinks directly on the current thread, but
            // before we do that we have to destroy the thread pool to ensure that any
            // pending log tasks are completed.
            self.thread_pool.destroy();
            self.backend.flush()
        } else {
            self.assign_task(Task::Flush {
                backend: self.clone_backend(),
            })
        }
    }

    /// For [`AsyncPoolSink`], the function performs the same call to all
    /// internal sinks.
    fn set_formatter(&self, formatter: Box<dyn Formatter>) {
        for sink in &self.backend.sinks {
            sink.set_formatter(formatter.clone_box())
        }
    }

    helper::common_impl! {
        @SinkCustom {
            level_filter: level_filter,
            formatter: None,
            error_handler: backend.error_handler,
        }
    }
}

/// The builder of [`AsyncPoolSink`].
pub struct AsyncPoolSinkBuilder {
    level_filter: LevelFilter,
    sinks: Sinks,
    overflow_policy: OverflowPolicy,
    thread_pool: Option<Arc<ThreadPool>>,
    error_handler: Option<ErrorHandler>,
}

impl AsyncPoolSinkBuilder {
    /// Add a [`Sink`].
    #[must_use]
    pub fn sink(mut self, sink: Arc<dyn Sink>) -> Self {
        self.sinks.push(sink);
        self
    }

    /// Add multiple [`Sink`]s.
    #[must_use]
    pub fn sinks<I>(mut self, sinks: I) -> Self
    where
        I: IntoIterator<Item = Arc<dyn Sink>>,
    {
        self.sinks.append(&mut sinks.into_iter().collect());
        self
    }

    /// Specifies a overflow policy.
    ///
    /// This parameter is **optional**, and defaults to
    /// [`OverflowPolicy::Block`].
    ///
    /// For more details, see the documentation of [`OverflowPolicy`].
    #[must_use]
    pub fn overflow_policy(mut self, overflow_policy: OverflowPolicy) -> Self {
        self.overflow_policy = overflow_policy;
        self
    }

    /// Specifies a custom thread pool.
    ///
    /// This parameter is **optional**, and defaults to the built-in thread
    /// pool.
    ///
    /// For more details, see the documentation of [`AsyncPoolSinkBuilder`].
    #[must_use]
    pub fn thread_pool(mut self, thread_pool: Arc<ThreadPool>) -> Self {
        self.thread_pool = Some(thread_pool);
        self
    }

    /// Builds a [`AsyncPoolSink`].
    pub fn build(self) -> Result<AsyncPoolSink> {
        let backend = Arc::new(Backend {
            sinks: self.sinks.clone(),
            error_handler: Atomic::new(self.error_handler),
        });

        let thread_pool = self.thread_pool.unwrap_or_else(default_thread_pool);

        Ok(AsyncPoolSink {
            level_filter: Atomic::new(self.level_filter),
            overflow_policy: self.overflow_policy,
            thread_pool,
            backend,
        })
    }

    helper::common_impl!(@SinkBuilderCustom {
        level_filter: level_filter,
        formatter: None,
        error_handler: error_handler,
    });
}

pub(crate) struct Backend {
    sinks: Sinks,
    error_handler: helper::SinkErrorHandler,
}

impl Backend {
    fn log(&self, record: &Record) -> Result<()> {
        let mut result = Ok(());
        for sink in &self.sinks {
            result = Error::push_result(result, sink.log(record));
        }
        result
    }

    fn flush(&self) -> Result<()> {
        let mut result = Ok(());
        for sink in &self.sinks {
            result = Error::push_result(result, sink.flush());
        }
        result
    }

    fn handle_error(&self, err: Error) {
        self.error_handler
            .load(Ordering::Relaxed)
            .unwrap_or(|err| default_error_handler("AsyncPoolSink", err))(err);
    }
}

pub(crate) enum Task {
    Log {
        backend: Arc<Backend>,
        record: RecordOwned,
    },
    Flush {
        backend: Arc<Backend>,
    },
}

impl Task {
    // calls this function in async threads
    pub(crate) fn exec(self) {
        match self {
            Task::Log { backend, record } => {
                if let Err(err) = backend.log(&record.as_ref()) {
                    backend.handle_error(err)
                }
            }
            Task::Flush { backend } => {
                if let Err(err) = backend.flush() {
                    backend.handle_error(err)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;
    use crate::{prelude::*, test_utils::*};

    #[test]
    fn default_thread_pool() {
        let counter_sink = Arc::new(CounterSink::new());
        let build_logger = || {
            build_test_logger(|b| {
                b.sink(Arc::new(
                    AsyncPoolSink::builder()
                        .sink(counter_sink.clone())
                        .build()
                        .unwrap(),
                ))
                .level_filter(LevelFilter::All)
                .flush_level_filter(LevelFilter::MoreSevereEqual(Level::Error))
            })
        };

        assert_eq!(counter_sink.log_count(), 0);
        assert_eq!(counter_sink.flush_count(), 0);

        {
            let logger = build_logger();

            info!(logger: logger, "");
            sleep(Duration::from_millis(50));
            assert_eq!(counter_sink.log_count(), 1);
            assert_eq!(counter_sink.flush_count(), 0);

            warn!(logger: logger, "");
            sleep(Duration::from_millis(50));
            assert_eq!(counter_sink.log_count(), 2);
            assert_eq!(counter_sink.flush_count(), 0);
        }

        {
            let logger = build_logger();

            error!(logger: logger, "");
            sleep(Duration::from_millis(50));
            assert_eq!(counter_sink.log_count(), 3);
            assert_eq!(counter_sink.flush_count(), 1);

            critical!(logger: logger, "");
            sleep(Duration::from_millis(50));
            assert_eq!(counter_sink.log_count(), 4);
            assert_eq!(counter_sink.flush_count(), 2);
        }
    }

    #[test]
    fn custom_thread_pool() {
        let counter_sink = Arc::new(CounterSink::new());
        let thread_pool = Arc::new(ThreadPool::builder().build().unwrap());
        let logger = build_test_logger(|b| {
            b.sink(Arc::new(
                AsyncPoolSink::builder()
                    .sink(counter_sink.clone())
                    .thread_pool(thread_pool)
                    .build()
                    .unwrap(),
            ))
            .level_filter(LevelFilter::All)
            .flush_level_filter(LevelFilter::MoreSevereEqual(Level::Error))
        });

        assert_eq!(counter_sink.log_count(), 0);
        assert_eq!(counter_sink.flush_count(), 0);

        info!(logger: logger, "");
        sleep(Duration::from_millis(50));
        assert_eq!(counter_sink.log_count(), 1);
        assert_eq!(counter_sink.flush_count(), 0);

        warn!(logger: logger, "");
        sleep(Duration::from_millis(50));
        assert_eq!(counter_sink.log_count(), 2);
        assert_eq!(counter_sink.flush_count(), 0);

        error!(logger: logger, "");
        sleep(Duration::from_millis(50));
        assert_eq!(counter_sink.log_count(), 3);
        assert_eq!(counter_sink.flush_count(), 1);
    }

    #[test]
    fn async_opeartions() {
        let counter_sink = Arc::new(CounterSink::with_delay(Some(Duration::from_secs(1))));
        // The default thread pool is not used here to avoid race when tests are run in
        // parallel.
        let thread_pool = Arc::new(ThreadPool::builder().build().unwrap());
        let logger = build_test_logger(|b| {
            b.sink(Arc::new(
                AsyncPoolSink::builder()
                    .sink(counter_sink.clone())
                    .thread_pool(thread_pool)
                    .build()
                    .unwrap(),
            ))
            .level_filter(LevelFilter::All)
            .flush_level_filter(LevelFilter::MoreSevereEqual(Level::Warn))
        });

        assert_eq!(counter_sink.log_count(), 0);
        assert_eq!(counter_sink.flush_count(), 0);

        info!(logger: logger, "meow");
        sleep(Duration::from_millis(500));
        assert_eq!(counter_sink.log_count(), 0);
        assert_eq!(counter_sink.flush_count(), 0);
        sleep(Duration::from_millis(750));
        assert_eq!(counter_sink.log_count(), 1);
        assert_eq!(counter_sink.flush_count(), 0);

        warn!(logger: logger, "nya");
        sleep(Duration::from_millis(250));
        assert_eq!(counter_sink.log_count(), 1);
        assert_eq!(counter_sink.flush_count(), 0);
        sleep(Duration::from_millis(1000));
        assert_eq!(counter_sink.log_count(), 2);
        assert_eq!(counter_sink.flush_count(), 0);
        sleep(Duration::from_millis(1250));
        assert_eq!(counter_sink.log_count(), 2);
        assert_eq!(counter_sink.flush_count(), 1);
    }
}

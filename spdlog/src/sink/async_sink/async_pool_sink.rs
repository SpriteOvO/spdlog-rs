use crate::{
    default_thread_pool,
    formatter::{Formatter, UnreachableFormatter},
    sink::{OverflowPolicy, Sink, SinkProp, SinkPropAccess, Sinks},
    sync::*,
    utils, Error, ErrorHandler, LevelFilter, Record, RecordOwned, Result, ThreadPool,
};

/// A [combined sink], logging and flushing asynchronously (thread-pool-based).
///
/// Expensive operations (such as `log` and `flush`) on asynchronous sinks will
/// be performed asynchronously on other threads.
///
/// Since there is no waiting, errors that occur while performing asynchronous
/// operations will not be returned to the upper level, and instead the error
/// handler of the sink will be called.
///
/// Users should only use asynchronous combined sinks to wrap actual sinks that
/// require a long time for operations (e.g., file sinks that are frequently
/// flushed, sinks involving networks), otherwise they will not get a
/// performance boost or even worse.
///
/// Since the thread pool has a capacity limit, the queue may be full in some
/// cases. When users encounter this situation, they have the following options:
///
///  - Adjust to a larger capacity via [`ThreadPoolBuilder::capacity`].
///
///  - Adjust the overflow policy via [`AsyncPoolSinkBuilder::overflow_policy`].
///
///  - Set up an error handler on asynchronous combined sinks via
///    [`AsyncPoolSinkBuilder::error_handler`]. The handler will be called when
///    a record is dropped or an operation has failed.
///
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
/// [`ThreadPoolBuilder::capacity`]: crate::ThreadPoolBuilder::capacity
/// [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/spdlog/examples
// The names `AsyncSink` and `AsyncRuntimeSink` is reserved for future use.
pub struct AsyncPoolSink {
    overflow_policy: OverflowPolicy,
    thread_pool: Arc<ThreadPool>,
    backend: Arc<Backend>,
}

impl AsyncPoolSink {
    /// Constructs a builder of `AsyncPoolSink` with default parameters:
    ///
    /// | Parameter         | Default Value                       |
    /// |-------------------|-------------------------------------|
    /// | [level_filter]    | [`LevelFilter::All`]                |
    /// | [error_handler]   | [`ErrorHandler::default()`]         |
    /// | [overflow_policy] | [`OverflowPolicy::Block`]           |
    /// | [thread_pool]     | internal shared default thread pool |
    ///
    /// [level_filter]: AsyncPoolSinkBuilder::level_filter
    /// [error_handler]: AsyncPoolSinkBuilder::error_handler
    /// [overflow_policy]: AsyncPoolSinkBuilder::overflow_policy
    /// [thread_pool]: AsyncPoolSinkBuilder::thread_pool
    #[must_use]
    pub fn builder() -> AsyncPoolSinkBuilder {
        let prop = SinkProp::default();
        // AsyncPoolSink does not have its own formatter, and we do not impl
        // `GetSinkProp` for it, so there should be no way to access the
        // formatter inside the `prop`.
        prop.set_formatter(UnreachableFormatter::new());

        AsyncPoolSinkBuilder {
            prop,
            overflow_policy: OverflowPolicy::Block,
            sinks: Sinks::new(),
            thread_pool: None,
        }
    }

    /// Gets a reference to internal sinks in the combined sink.
    #[must_use]
    pub fn sinks(&self) -> &[Arc<dyn Sink>] {
        &self.backend.sinks
    }

    fn assign_task(&self, task: Task) -> Result<()> {
        self.thread_pool.assign_task(task, self.overflow_policy)
    }

    #[must_use]
    fn clone_backend(&self) -> Arc<Backend> {
        Arc::clone(&self.backend)
    }
}

impl SinkPropAccess for AsyncPoolSink {
    fn level_filter(&self) -> LevelFilter {
        self.backend.prop.level_filter()
    }

    fn set_level_filter(&self, level_filter: LevelFilter) {
        self.backend.prop.set_level_filter(level_filter);
    }

    /// For [`AsyncPoolSink`], the function performs the same call to all
    /// internal sinks.
    fn set_formatter(&self, formatter: Box<dyn Formatter>) {
        utils::for_each_with!(self.backend.sinks, formatter, |(sink, formatter)| sink
            .set_formatter(formatter));
    }

    fn set_error_handler(&self, handler: ErrorHandler) {
        self.backend.prop.set_error_handler(handler);
    }
}

impl Sink for AsyncPoolSink {
    fn log(&self, record: &Record) -> Result<()> {
        self.assign_task(Task::Log {
            backend: self.clone_backend(),
            record: record.to_owned(),
        })
    }

    fn flush(&self) -> Result<()> {
        self.assign_task(Task::Flush {
            backend: self.clone_backend(),
        })
    }

    fn flush_on_exit(&self) -> Result<()> {
        // https://github.com/SpriteOvO/spdlog-rs/issues/64
        //
        // If the program is tearing down, this will be the final flush. `crossbeam`
        // uses thread-local internally, which is not supported in `atexit` callback.
        // This can be bypassed by flushing sinks directly on the current thread, but
        // before we do that we have to destroy the thread pool to ensure that any
        // pending log tasks are completed.
        self.thread_pool.destroy();
        self.backend.flush_on_exit()
    }
}

#[allow(missing_docs)]
pub struct AsyncPoolSinkBuilder {
    prop: SinkProp,
    sinks: Sinks,
    overflow_policy: OverflowPolicy,
    thread_pool: Option<Arc<ThreadPool>>,
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
    /// When the channel is full, an incoming operation is handled according to
    /// the specified policy.
    #[must_use]
    pub fn overflow_policy(mut self, overflow_policy: OverflowPolicy) -> Self {
        self.overflow_policy = overflow_policy;
        self
    }

    /// Specifies a custom thread pool.
    ///
    /// This parameter is **optional**, and defaults to the internal shared
    /// default thread pool.
    #[must_use]
    pub fn thread_pool(mut self, thread_pool: Arc<ThreadPool>) -> Self {
        self.thread_pool = Some(thread_pool);
        self
    }

    // Prop
    //

    /// Specifies a log level filter.
    ///
    /// This parameter is **optional**, and defaults to [`LevelFilter::All`].
    #[must_use]
    pub fn level_filter(self, level_filter: LevelFilter) -> Self {
        self.prop.set_level_filter(level_filter);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        note = "AsyncPoolSink does not have its own formatter, this method has no effect, it was added by accident and may be removed in the future",
        since = "0.5.2"
    )]
    #[must_use]
    pub fn formatter<F>(self, formatter: F) -> Self
    where
        F: Formatter + 'static,
    {
        self.prop.set_formatter(formatter);
        self
    }

    /// Specifies an error handler.
    ///
    /// This parameter is **optional**, and defaults to
    /// [`ErrorHandler::default()`].
    #[must_use]
    pub fn error_handler<F: Into<ErrorHandler>>(self, handler: F) -> Self {
        self.prop.set_error_handler(handler);
        self
    }

    /// Builds a [`AsyncPoolSink`].
    pub fn build(self) -> Result<AsyncPoolSink> {
        let backend = Arc::new(Backend {
            prop: self.prop,
            sinks: self.sinks.clone(),
        });

        let thread_pool = self.thread_pool.unwrap_or_else(default_thread_pool);

        Ok(AsyncPoolSink {
            overflow_policy: self.overflow_policy,
            thread_pool,
            backend,
        })
    }

    /// Builds a `Arc<AsyncPoolSink>`.
    ///
    /// This is a shorthand method for `.build().map(Arc::new)`.
    pub fn build_arc(self) -> Result<Arc<AsyncPoolSink>> {
        self.build().map(Arc::new)
    }
}

pub(crate) struct Backend {
    prop: SinkProp,
    sinks: Sinks,
}

impl Backend {
    fn log(&self, record: &Record) -> Result<()> {
        let mut result = Ok(());
        for sink in &self.sinks {
            result = Error::push_result(result, sink.log(record));
        }
        result
    }

    fn flush_with(&self, with: impl Fn(&dyn Sink) -> Result<()>) -> Result<()> {
        let mut result = Ok(());
        for sink in &self.sinks {
            result = Error::push_result(result, with(&**sink));
        }
        result
    }

    fn flush(&self) -> Result<()> {
        self.flush_with(|sink| sink.flush())
    }

    fn flush_on_exit(&self) -> Result<()> {
        self.flush_with(|sink| sink.flush_on_exit())
    }

    fn handle_error(&self, err: Error) {
        self.prop.call_error_handler_internal("AsyncPoolSink", err)
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
        let counter_sink = Arc::new(TestSink::new());
        let build_logger = || {
            build_test_logger(|b| {
                b.sink(
                    AsyncPoolSink::builder()
                        .sink(counter_sink.clone())
                        .build_arc()
                        .unwrap(),
                )
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
        let counter_sink = Arc::new(TestSink::new());
        let thread_pool = ThreadPool::builder().build_arc().unwrap();
        let logger = build_test_logger(|b| {
            b.sink(
                AsyncPoolSink::builder()
                    .sink(counter_sink.clone())
                    .thread_pool(thread_pool)
                    .build_arc()
                    .unwrap(),
            )
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
        let counter_sink = Arc::new(TestSink::with_delay(Some(Duration::from_secs(1))));
        // The default thread pool is not used here to avoid race when tests are run in
        // parallel.
        let thread_pool = ThreadPool::builder().build_arc().unwrap();
        let logger = build_test_logger(|b| {
            b.sink(
                AsyncPoolSink::builder()
                    .sink(counter_sink.clone())
                    .thread_pool(thread_pool)
                    .build_arc()
                    .unwrap(),
            )
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

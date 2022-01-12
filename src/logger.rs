//! Provides a logger structure.

use std::{
    sync::{atomic::Ordering, Arc, Mutex},
    time::Duration,
};

use atomic::Atomic;

use crate::{
    periodic_worker::PeriodicWorker,
    sink::{Sink, Sinks},
    Error, ErrorHandler, Level, LevelFilter, Record,
};

/// A logger structure.
///
/// A logger contains a combination of sinks, and sinks implement writing log
/// messages to actual targets.
///
/// Users usually log messages through log macros.
///
/// # Examples
///
/// ```
/// # use std::sync::Arc;
/// use std::time::Duration;
/// use spdlog::prelude::*;
///
/// # let custom_logger: Arc<Logger> = spdlog::default_logger();
/// let default_logger: Arc<Logger> = spdlog::default_logger();
/// default_logger.set_level_filter(LevelFilter::All);
/// default_logger.set_flush_period(Some(Duration::from_secs(10)));
/// info!("logging with default logger");
///
/// custom_logger.set_level_filter(LevelFilter::All);
/// custom_logger.set_flush_period(Some(Duration::from_secs(10)));
/// info!(logger: custom_logger, "logging with custom logger");
/// ```
///
/// For more examples, see [./examples] directory.
///
/// [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/examples
pub struct Logger {
    name: Option<String>,
    level_filter: Atomic<LevelFilter>,
    sinks: Sinks,
    flush_level_filter: Atomic<LevelFilter>,
    periodic_flusher: Mutex<Option<PeriodicWorker>>,
    error_handler: spin::RwLock<Option<ErrorHandler>>,
}

impl Logger {
    /// Constructs a `Logger` with empty sink.
    pub fn new() -> Logger {
        Logger {
            name: None,
            level_filter: Atomic::new(LevelFilter::MoreSevereEqual(Level::Info)),
            sinks: vec![],
            flush_level_filter: Atomic::new(LevelFilter::Off),
            periodic_flusher: Mutex::new(None),
            error_handler: spin::RwLock::new(None),
        }
    }

    /// Constructs a [`LoggerBuilder`].
    pub fn builder() -> LoggerBuilder {
        LoggerBuilder::new()
    }

    /// Gets the logger name.
    ///
    /// Returns `None` if the logger does not have a name.
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_ref())
    }

    /// Determines if a log message with the specified level would be
    /// logged.
    ///
    /// This allows callers to avoid expensive computation of log message
    /// arguments if the message would be discarded anyway.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::sync::Arc;
    /// use spdlog::prelude::*;
    ///
    /// let logger: Arc<Logger> = spdlog::default_logger();
    ///
    /// logger.set_level_filter(LevelFilter::MoreSevere(Level::Info));
    /// assert_eq!(logger.should_log(Level::Debug), false);
    /// assert_eq!(logger.should_log(Level::Info), false);
    /// assert_eq!(logger.should_log(Level::Warn), true);
    /// assert_eq!(logger.should_log(Level::Error), true);
    ///
    /// logger.set_level_filter(LevelFilter::All);
    /// assert_eq!(logger.should_log(Level::Debug), true);
    /// assert_eq!(logger.should_log(Level::Info), true);
    /// assert_eq!(logger.should_log(Level::Warn), true);
    /// assert_eq!(logger.should_log(Level::Error), true);
    /// ```
    pub fn should_log(&self, level: Level) -> bool {
        self.level_filter().compare(level)
    }

    /// Logs a record.
    ///
    /// Users usually do not use this function directly, use log macros instead.
    pub fn log(&self, record: &Record) {
        if !self.should_log(record.level()) {
            return;
        }
        self.sink_record(record);
    }

    /// Flushes any buffered records.
    ///
    /// Users can call this function to flush manually or use auto-flush
    /// policies. See also [`Logger::flush_level_filter`] and
    /// [`Logger::set_flush_period`].
    ///
    /// Note that it is expensive, calling it frequently will affect
    /// performance.
    pub fn flush(&self) {
        self.flush_sinks();
    }

    /// Gets the flush level filter.
    pub fn flush_level_filter(&self) -> LevelFilter {
        self.flush_level_filter.load(Ordering::Relaxed)
    }

    /// Sets a flush level filter.
    ///
    /// When logging a new record, flush the buffer if this filter condition is
    /// true.
    ///
    /// This auto-flush policy can work with [`Logger::set_flush_period`]
    /// together.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::sync::Arc;
    /// use spdlog::prelude::*;
    ///
    /// # let logger: Arc<Logger> = spdlog::default_logger();
    /// logger.set_flush_level_filter(LevelFilter::Off);
    /// trace!(logger: logger, "hello");
    /// trace!(logger: logger, "world");
    /// // Until here the buffer may not have been flushed (depending on sinks implementation)
    ///
    /// logger.set_flush_level_filter(LevelFilter::All);
    /// trace!(logger: logger, "hello"); // Logs and flushes the buffer once
    /// trace!(logger: logger, "world"); // Logs and flushes the buffer once
    /// ```
    pub fn set_flush_level_filter(&self, level_filter: LevelFilter) {
        self.flush_level_filter
            .store(level_filter, Ordering::Relaxed);
    }

    /// Gets the log filter level.
    pub fn level_filter(&self) -> LevelFilter {
        self.level_filter.load(Ordering::Relaxed)
    }

    /// Sets the log filter level.
    ///
    /// # Examples
    ///
    /// See [`Logger::should_log`].
    pub fn set_level_filter(&self, level_filter: LevelFilter) {
        self.level_filter.store(level_filter, Ordering::Relaxed);
    }

    /// Sets periodic flush.
    ///
    /// This function receives a `&Arc<Self>`. Calling it will spawn a new
    /// thread.
    ///
    /// This auto-flush policy can work with [`Logger::set_flush_level_filter`]
    /// together.
    ///
    /// # Panics
    ///
    ///  - Panics if `interval` is zero.
    ///
    ///  - Panics if this function is called with `Some` value and then clones
    ///    the `Logger` instead of the `Arc<Logger>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// # use std::sync::Arc;
    /// # use spdlog::prelude::*;
    ///
    /// # let logger: Arc<Logger> = spdlog::default_logger();
    /// // From now on, auto-flush the `logger` buffer every 10 seconds.
    /// logger.set_flush_period(Some(Duration::from_secs(10)));
    ///
    /// // Remove periodic auto-flush.
    /// logger.set_flush_period(None);
    /// ```
    pub fn set_flush_period(self: &Arc<Self>, interval: Option<Duration>) {
        let mut periodic_flusher = self.periodic_flusher.lock().unwrap();

        *periodic_flusher = None;

        if let Some(interval) = interval {
            let weak = Arc::downgrade(self);
            let callback = move || {
                let strong = weak.upgrade();
                if let Some(strong) = strong {
                    strong.flush_sinks();
                    true
                } else {
                    false // All `Arc`s are dropped, return `false` to quit the
                          // worker thread.
                }
            };
            *periodic_flusher = Some(PeriodicWorker::new(Box::new(callback), interval));
        }
    }

    /// Gets a reference to sinks in the logger.
    pub fn sinks(&self) -> &Sinks {
        &self.sinks
    }

    /// Gets a mutable reference to sinks in the logger.
    pub fn sinks_mut(&mut self) -> &mut Sinks {
        &mut self.sinks
    }

    /// Sets a error handler.
    ///
    /// If an error occurs while logging or flushing, this handler will be
    /// called. If no handler is set, the error will be output to the terminal
    /// and then ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use spdlog::prelude::*;
    ///
    /// spdlog::default_logger().set_error_handler(Some(|err: spdlog::Error| {
    ///     panic!("spdlog-rs error: {}", err)
    /// }));
    /// ```
    pub fn set_error_handler(&self, handler: Option<ErrorHandler>) {
        *self.error_handler.write() = handler;
    }

    fn sink_record(&self, record: &Record) {
        self.sinks.iter().for_each(|sink| {
            if sink.should_log(record.level()) {
                if let Err(err) = sink.log(record) {
                    self.handle_error(err);
                }
            }
        });

        if self.should_flush(record) {
            self.flush();
        }
    }

    fn flush_sinks(&self) {
        self.sinks.iter().for_each(|sink| {
            if let Err(err) = sink.flush() {
                self.handle_error(err);
            }
        });
    }

    fn handle_error(&self, err: Error) {
        if let Some(handler) = self.error_handler.read().as_ref() {
            handler(err)
        } else {
            crate::default_error_handler(
                format!(
                    "Logger ({})",
                    self.name.as_ref().map_or("*no name*", String::as_str)
                ),
                err,
            );
        }
    }

    fn should_flush(&self, record: &Record) -> bool {
        self.flush_level_filter().compare(record.level())
    }
}

impl Default for Logger {
    fn default() -> Logger {
        Logger::new()
    }
}

impl Clone for Logger {
    /// Clones the `Logger`.
    ///
    /// # Panics
    ///
    /// Panics if [`Logger::set_flush_period`] is called with `Some` value and
    /// then clones the `Logger` instead of the `Arc<Logger>`.
    fn clone(&self) -> Self {
        if self.periodic_flusher.lock().unwrap().is_some() {
            panic!(
                "you can't clone a `Logger` with a `flush_period` value, \
                 clone a `Arc<Logger>` instead."
            );
        }

        Logger {
            name: self.name.clone(),
            level_filter: Atomic::new(self.level_filter()),
            sinks: self.sinks.clone(),
            flush_level_filter: Atomic::new(self.flush_level_filter()),
            periodic_flusher: Mutex::new(None),
            error_handler: spin::RwLock::new(*self.error_handler.read()),
        }
    }
}

/// The builder of [`Logger`].
#[derive(Clone)]
pub struct LoggerBuilder {
    logger: Logger,
}

impl LoggerBuilder {
    /// Constructs a `LoggerBuilder`.
    ///
    /// The default value of [`Logger`] is the same as [`Logger::new`].
    pub fn new() -> Self {
        Self {
            logger: Logger::new(),
        }
    }

    /// Sets the name of the logger.
    pub fn name<S>(&mut self, name: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.logger.name = Some(name.into());
        self
    }

    /// Sets the log filter level.
    #[allow(unused_mut)]
    pub fn level_filter(&mut self, level_filter: LevelFilter) -> &mut Self {
        self.logger.set_level_filter(level_filter);
        self
    }

    /// Add a [`Sink`].
    pub fn sink(&mut self, sink: Arc<dyn Sink>) -> &mut Self {
        self.logger.sinks.push(sink);
        self
    }

    /// Add multiple [`Sink`]s.
    pub fn sinks<I>(&mut self, sinks: I) -> &mut Self
    where
        I: IntoIterator<Item = Arc<dyn Sink>>,
    {
        self.logger.sinks.append(&mut sinks.into_iter().collect());
        self
    }

    /// Sets the flush level filter.
    #[allow(unused_mut)]
    pub fn flush_level_filter(&mut self, level_filter: LevelFilter) -> &mut Self {
        self.logger.set_flush_level_filter(level_filter);
        self
    }

    /// Sets the error handler.
    #[allow(unused_mut)]
    pub fn error_handler(&mut self, handler: ErrorHandler) -> &mut Self {
        self.logger.set_error_handler(Some(handler));
        self
    }

    /// Builds a [`Logger`].
    pub fn build(&mut self) -> Logger {
        self.logger.clone()
    }
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{prelude::*, test_utils::*};

    use std::{thread, time::Duration};

    #[test]
    fn send_sync() {
        assert_send::<Logger>();
        assert_sync::<Logger>();
    }

    #[test]
    fn flush_level() {
        let test_sink = Arc::new(CounterSink::new());
        let test_logger = Logger::builder().sink(test_sink.clone()).build();

        trace!(logger: test_logger, "");
        error!(logger: test_logger, "");
        assert_eq!(test_sink.flush_count(), 0);
        test_sink.reset();

        test_logger.set_flush_level_filter(LevelFilter::MoreSevereEqual(Level::Warn));
        debug!(logger: test_logger, "");
        warn!(logger: test_logger, "");
        assert_eq!(test_sink.flush_count(), 1);
        test_sink.reset();

        test_logger.set_flush_level_filter(LevelFilter::Off);
        info!(logger: test_logger, "");
        trace!(logger: test_logger, "");
        assert_eq!(test_sink.flush_count(), 0);
        test_sink.reset();

        test_logger.set_flush_level_filter(LevelFilter::MoreSevereEqual(Level::Trace));
        info!(logger: test_logger, "");
        warn!(logger: test_logger, "");
        assert_eq!(test_sink.flush_count(), 2);
        test_sink.reset();
    }

    #[test]
    fn periodic_flush() {
        let test_sink = Arc::new(CounterSink::new());
        let test_logger = Arc::new(Logger::builder().sink(test_sink.clone()).build());

        test_logger.set_flush_period(Some(Duration::from_secs(1)));

        assert_eq!(test_sink.flush_count(), 0);

        thread::sleep(Duration::from_millis(1250));
        assert_eq!(test_sink.flush_count(), 1);

        thread::sleep(Duration::from_millis(1250));
        assert_eq!(test_sink.flush_count(), 2);

        test_logger.set_flush_period(None);

        thread::sleep(Duration::from_millis(1250));
        assert_eq!(test_sink.flush_count(), 2);

        test_logger.set_flush_period(Some(Duration::from_secs(1)));

        thread::sleep(Duration::from_millis(1250));
        assert_eq!(test_sink.flush_count(), 3);
    }
}

//! Provides a basic and default logger.

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{
    periodic_worker::PeriodicWorker,
    sink::{Sink, Sinks},
    Error, ErrorHandler, Level, LevelFilter, Record,
};

/// A logger structure.
pub struct Logger {
    name: Option<String>,
    level_filter: LevelFilter,
    sinks: Sinks,
    flush_level_filter: LevelFilter,
    periodic_flusher: Mutex<Option<PeriodicWorker>>,
    error_handler: Option<ErrorHandler>,
}

impl Logger {
    /// Constructs a empty `Logger`.
    pub fn new() -> Logger {
        Logger {
            name: None,
            level_filter: LevelFilter::MoreSevereEqual(Level::Info),
            sinks: vec![],
            flush_level_filter: LevelFilter::Off,
            periodic_flusher: Mutex::new(None),
            error_handler: None,
        }
    }

    /// Constructs a [`LoggerBuilder`].
    pub fn builder() -> LoggerBuilder {
        LoggerBuilder::new()
    }

    /// Gets the logger name.
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_ref())
    }

    /// Determines if a log message with the specified level would be
    /// logged.
    ///
    /// This allows callers to avoid expensive computation of log message
    /// arguments if the message would be discarded anyway.
    pub fn should_log(&self, level: Level) -> bool {
        self.level_filter.compare(level)
    }

    /// Logs the message.
    ///
    /// Note that `should_log` is *not* necessarily called before this method.
    /// Implementations of `log` should perform all necessary filtering
    /// internally.
    pub fn log(&self, record: &Record) {
        if !self.should_log(record.level()) {
            return;
        }
        self.sink_record(record);
    }

    /// Flushes any buffered records.
    pub fn flush(&self) {
        self.flush_sinks();
    }

    /// Getter of the flush level filter.
    pub fn flush_level_filter(&self) -> LevelFilter {
        self.flush_level_filter
    }

    /// Flushes any buffered records if the level filter condition is true.
    pub fn set_flush_level_filter(&mut self, level_filter: LevelFilter) {
        self.flush_level_filter = level_filter;
    }

    /// Getter of the log filter level.
    pub fn level_filter(&self) -> LevelFilter {
        self.level_filter
    }

    /// Setter of the log filter level.
    pub fn set_level_filter(&mut self, level_filter: LevelFilter) {
        self.level_filter = level_filter;
    }

    /// Sets periodic flush.
    ///
    /// # Panics
    ///
    /// Panics if `interval` is zero.
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
                    false // All `Arc`s are dropped, return `false` to quit the worker thread.
                }
            };
            *periodic_flusher = Some(PeriodicWorker::new(Box::new(callback), interval));
        }
    }

    /// Getter of the sinks.
    pub fn sinks(&self) -> &Sinks {
        &self.sinks
    }

    /// Getter of the sinks, returns `&mut`.
    pub fn sinks_mut(&mut self) -> &mut Sinks {
        &mut self.sinks
    }

    /// Sets a error handler.
    ///
    /// If an error occurs while logging, this handler will be called. If no
    /// handler is set, the error will be ignored.
    pub fn set_error_handler(&mut self, handler: Option<ErrorHandler>) {
        self.error_handler = handler;
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
        if let Some(handler) = &self.error_handler {
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
        self.flush_level_filter.compare(record.level())
    }
}

impl Default for Logger {
    fn default() -> Logger {
        Logger::new()
    }
}

/// The builder of [`Logger`].
pub struct LoggerBuilder {
    logger: Logger,
}

impl LoggerBuilder {
    /// Constructs a `LoggerBuilder`.
    ///
    /// The default value is the same as [`Logger::new()`].
    pub fn new() -> Self {
        Self {
            logger: Logger::new(),
        }
    }

    /// Sets the logger name.
    #[must_use]
    pub fn name<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.logger.name = Some(name.into());
        self
    }

    /// Sets the log filter level.
    #[must_use]
    pub fn level_filter(mut self, level_filter: LevelFilter) -> Self {
        self.logger.level_filter = level_filter;
        self
    }

    /// Add a [`Sink`].
    #[must_use]
    pub fn sink(mut self, sink: Arc<dyn Sink>) -> Self {
        self.logger.sinks.push(sink);
        self
    }

    /// Add multiple [`Sink`]s.
    #[must_use]
    pub fn sinks<I>(mut self, sinks: I) -> Self
    where
        I: IntoIterator<Item = Arc<dyn Sink>>,
    {
        self.logger.sinks.append(&mut sinks.into_iter().collect());
        self
    }

    /// Sets the flush level filter.
    #[must_use]
    pub fn flush_level_filter(mut self, level_filter: LevelFilter) -> Self {
        self.logger.flush_level_filter = level_filter;
        self
    }

    /// Sets the error handler.
    #[must_use]
    pub fn error_handler(mut self, handler: ErrorHandler) -> Self {
        self.logger.error_handler = Some(handler);
        self
    }

    /// Builds a [`Logger`].
    pub fn build(self) -> Logger {
        self.logger
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
        let mut test_logger = Logger::builder().sink(test_sink.clone()).build();

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

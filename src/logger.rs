//! Provides a basic and default logger.

use std::sync::{Arc, RwLock};

use crate::{
    sink::{Sink, Sinks},
    Error, ErrorHandler, Level, LevelFilter, Record,
};

/// A logger structure.
pub struct Logger {
    name: Option<String>,
    level: LevelFilter,
    sinks: Sinks,
    flush_level: LevelFilter,
    error_handler: Option<ErrorHandler>,
}

impl Logger {
    /// Constructs a empty `Logger`.
    pub fn new() -> Logger {
        Logger {
            name: None,
            level: LevelFilter::Info,
            sinks: vec![],
            flush_level: LevelFilter::Off,
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
        level <= self.level
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

    /// Getter of the flush level.
    pub fn flush_level(&self) -> LevelFilter {
        self.flush_level
    }

    /// Flushes any buffered records on the level or more severe.
    pub fn set_flush_level(&mut self, level: LevelFilter) {
        self.flush_level = level;
    }

    /// Getter of the log filter level.
    pub fn level(&self) -> LevelFilter {
        self.level
    }

    /// Setter of the log filter level.
    pub fn set_level(&mut self, level: LevelFilter) {
        self.level = level;
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
            let sink = sink.read().unwrap();
            if sink.should_log(record.level()) {
                if let Err(err) = sink.log(record) {
                    drop(sink);
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
            if let Err(err) = sink.read().unwrap().flush() {
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
        record.level() <= self.flush_level
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
    pub fn name<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.logger.name = Some(name.into());
        self
    }

    /// Sets the log filter level.
    pub fn level(mut self, level: LevelFilter) -> Self {
        self.logger.level = level;
        self
    }

    /// Add a [`Sink`].
    pub fn sink(mut self, sink: Arc<RwLock<dyn Sink>>) -> Self {
        self.logger.sinks.push(sink);
        self
    }

    /// Add multiple [`Sink`]s.
    pub fn sinks<I>(mut self, sinks: I) -> Self
    where
        I: IntoIterator<Item = Arc<RwLock<dyn Sink>>>,
    {
        self.logger.sinks.append(&mut sinks.into_iter().collect());
        self
    }

    /// Sets the flush level.
    pub fn flush_level(mut self, level: LevelFilter) -> Self {
        self.logger.flush_level = level;
        self
    }

    /// Sets the error handler.
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

    #[test]
    fn send_sync() {
        assert_send::<Logger>();
        assert_sync::<Logger>();
    }

    #[test]
    fn flush_level() {
        let test_sink = Arc::new(RwLock::new(TestSink::new()));
        let mut test_logger = Logger::builder().sink(test_sink.clone()).build();

        trace!(logger: test_logger, "");
        error!(logger: test_logger, "");
        assert_eq!(test_sink.read().unwrap().flush_counter(), 0);
        test_sink.read().unwrap().reset();

        test_logger.set_flush_level(LevelFilter::Warn);
        debug!(logger: test_logger, "");
        warn!(logger: test_logger, "");
        assert_eq!(test_sink.read().unwrap().flush_counter(), 1);
        test_sink.read().unwrap().reset();

        test_logger.set_flush_level(LevelFilter::Off);
        info!(logger: test_logger, "");
        trace!(logger: test_logger, "");
        assert_eq!(test_sink.read().unwrap().flush_counter(), 0);
        test_sink.read().unwrap().reset();

        test_logger.set_flush_level(LevelFilter::Trace);
        info!(logger: test_logger, "");
        warn!(logger: test_logger, "");
        assert_eq!(test_sink.read().unwrap().flush_counter(), 2);
        test_sink.read().unwrap().reset();
    }
}

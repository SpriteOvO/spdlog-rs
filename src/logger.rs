//! Provides a basic and default logger.

use std::sync::{Arc, Mutex};

use crate::{
    sink::{Sink, Sinks},
    ErrorHandler, Level, LevelFilter, Record,
};

/// A logger structure.
pub struct Logger {
    level: LevelFilter,
    sinks: Sinks,
    error_handler: Mutex<Option<ErrorHandler>>,
}

impl Logger {
    /// Constructs a `Logger`.
    pub fn new() -> Logger {
        Logger {
            level: LevelFilter::Info,
            sinks: vec![],
            error_handler: Mutex::new(None),
        }
    }

    /// Constructs a `Logger` with a [`Sink`].
    pub fn with_sink(sink: Arc<dyn Sink>) -> Logger {
        Logger {
            level: LevelFilter::Info,
            sinks: vec![sink],
            error_handler: Mutex::new(None),
        }
    }

    /// Constructs a `Logger` with multiple [`Sink`]s.
    pub fn with_sinks<I>(iter: I) -> Logger
    where
        I: IntoIterator<Item = Arc<dyn Sink>>,
    {
        Logger {
            level: LevelFilter::Info,
            sinks: iter.into_iter().collect(),
            error_handler: Mutex::new(None),
        }
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
    pub fn flush(&self) {}

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
    pub fn set_error_handler(&mut self, handler: ErrorHandler) {
        self.error_handler.lock().unwrap().replace(handler);
    }

    fn sink_record(&self, record: &Record) {
        self.sinks.iter().for_each(|sink| {
            if sink.should_log(record.level()) {
                if let Err(err) = sink.log(record) {
                    if let Some(handler) = self.error_handler.lock().unwrap().as_mut() {
                        handler(err)
                    }
                }
            }
        })
    }
}

impl Default for Logger {
    fn default() -> Logger {
        Logger::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn send_sync() {
        assert_send::<Logger>();
        assert_sync::<Logger>();
    }
}

//! Provides loggers for handling log messages and managing sinks.

pub mod basic_logger;

pub use basic_logger::BasicLogger;

use crate::{sink::Sinks, ErrorHandler, Level, LevelFilter, Record};

/// A trait for loggers.
pub trait Logger: Sync + Send {
    /// Determines if a log message with the specified level would be
    /// logged.
    ///
    /// This allows callers to avoid expensive computation of log message
    /// arguments if the message would be discarded anyway.
    fn enabled(&self, level: Level) -> bool;

    /// Logs the `Record`.
    ///
    /// Note that `enabled` is *not* necessarily called before this method.
    /// Implementations of `log` should perform all necessary filtering
    /// internally.
    fn log(&self, record: &Record);

    /// Flushes any buffered records.
    fn flush(&self);

    /// Getter of the log filter level.
    fn level(&self) -> LevelFilter;

    /// Setter of the log filter level.
    fn set_level(&mut self, level: LevelFilter);

    /// Getter of the sinks.
    fn sinks(&self) -> &Sinks;

    /// Getter of the sinks, returns `&mut`.
    fn sinks_mut(&mut self) -> &mut Sinks;

    /// Sink a given record.
    fn sink_record(&self, record: &Record);

    /// Sets a error handler.
    ///
    /// If an error occurs while logging, this handler will be called. If no
    /// handler is set, the error will be ignored.
    fn set_error_handler(&mut self, handler: ErrorHandler);
}

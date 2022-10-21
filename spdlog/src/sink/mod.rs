//! Provides sinks to flexibly output log messages to specified targets.
//!
//! # Sink
//!
//! Sinks are the objects that actually write logs to their targets. Each sink
//! should be responsible for only single target (e.g file, console, database),
//! and each sink has its own private instance of [`Formatter`] object.
//!
//! A sink has its own level filter that is not shared with the logger, and a
//! [`Logger`] can combine multiple [`Sink`]s.
//!
//! # Combined sink
//!
//! A combined sink is also a sink, but instead of having its own target and
//! formatter, it combines other sinks (as sub-sinks).
//!
//! Operations on a combined sink will be forwarded to its sub-sinks according
//! to the implementation.
//!
//! # Asynchronous combined sink
//!
//! Asynchronous combined sink is a type of combined sink. Expensive operations
//! (such as `log` and `flush`) on asynchronous sinks will be performed
//! asynchronously on other threads.
//!
//! Since there is no waiting, errors that occur while performing asynchronous
//! operations will not be returned to the upper level, and instead the error
//! handler of the sink will be called.
//!
//! [`Logger`]: crate::logger::Logger

#[cfg(feature = "multi-thread")]
pub(crate) mod async_sink;
mod file_sink;
mod helper;
#[cfg(any(
    all(target_os = "linux", feature = "native", feature = "libsystemd"),
    all(doc, not(doctest))
))]
mod journal_sink;
mod rotating_file_sink;
mod std_stream_sink;
#[cfg(any(all(windows, feature = "native"), all(doc, not(doctest))))]
mod win_debug_sink;
mod write_sink;

#[cfg(feature = "multi-thread")]
pub use async_sink::*;
pub use file_sink::*;
#[cfg(any(
    all(target_os = "linux", feature = "native", feature = "libsystemd"),
    all(doc, not(doctest))
))]
pub use journal_sink::*;
pub use rotating_file_sink::*;
pub use std_stream_sink::*;
#[cfg(any(all(windows, feature = "native"), all(doc, not(doctest))))]
pub use win_debug_sink::*;
pub use write_sink::*;

use crate::{formatter::Formatter, sync::*, ErrorHandler, Level, LevelFilter, Record, Result};

/// A trait for sinks.
pub trait Sink: Sync + Send {
    /// Determines if a log message with the specified level would be logged.
    fn should_log(&self, level: Level) -> bool {
        self.level_filter().compare(level)
    }

    /// Logs a record.
    ///
    /// Implementors should always call [`Sink::should_log`] internally to
    /// filter records.
    fn log(&self, record: &Record) -> Result<()>;

    /// Flushes any buffered records.
    fn flush(&self) -> Result<()>;

    /// Gets the log level filter.
    fn level_filter(&self) -> LevelFilter;

    /// Sets the log level filter.
    fn set_level_filter(&self, level_filter: LevelFilter);

    /// Sets the formatter.
    fn set_formatter(&self, formatter: Box<dyn Formatter>);

    /// Sets a error handler.
    ///
    /// Any errors that occur in `Sink` will be returned as directly as possible
    /// (e.g. returned to [`Logger`]), but some errors that are not likely to be
    /// returned directly will call this error handler. Most of these errors are
    /// uncommon.
    ///
    /// If no handler is set, errors will be print to `stderr` and then ignored.
    ///
    /// [`Logger`]: crate::logger::Logger
    fn set_error_handler(&self, handler: Option<ErrorHandler>);
}

/// A container for [`Sink`]s.
pub type Sinks = Vec<Arc<dyn Sink>>;

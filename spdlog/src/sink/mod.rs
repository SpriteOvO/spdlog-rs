//! Provides sinks to flexibly output log messages to specified targets.

mod file_sink;
#[cfg(all(target_os = "linux", feature = "libsystemd"))]
mod journal_sink;
mod rotating_file_sink;
mod std_stream_sink;
#[cfg(all(windows, feature = "winapi"))]
mod win_debug_sink;
mod write_sink;

pub use file_sink::*;
#[cfg(all(target_os = "linux", feature = "libsystemd"))]
pub use journal_sink::*;
pub use rotating_file_sink::*;
pub use std_stream_sink::*;
#[cfg(all(windows, feature = "winapi"))]
pub use win_debug_sink::*;
pub use write_sink::*;

use crate::{formatter::Formatter, sync::*, Level, LevelFilter, Record, Result};

/// A trait for sinks.
///
/// Sinks are the objects that actually write logs to their targets. Each sink
/// should be responsible for only single target (e.g file, console, database),
/// and each sink has its own private instance of [`Formatter`] object.
///
/// A [`Logger`] can combine multiple [`Sink`]s.
///
/// A sink has its own level filter that is not shared with the logger.
///
/// [`Logger`]: crate::logger::Logger
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

    /// Sets the log level filter, and returns the old formatter.
    fn swap_formatter(&self, formatter: Box<dyn Formatter>) -> Box<dyn Formatter>;

    /// Sets the formatter.
    fn set_formatter(&self, formatter: Box<dyn Formatter>) {
        self.swap_formatter(formatter);
    }
}

/// A container for [`Sink`]s.
pub type Sinks = Vec<Arc<dyn Sink>>;

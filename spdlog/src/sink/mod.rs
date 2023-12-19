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
//! Users should only use asynchronous combined sinks to wrap actual sinks that
//! require a long time for operations (e.g. involving UDP sends), otherwise
//! they will not get a performance boost or even worse.
//!
//! Since the thread pool has a capacity limit, the queue may be full in some
//! cases. When users encounter this situation, they have the following options:
//!
//!  - Adjust to a larger capacity via [`ThreadPoolBuilder::capacity`].
//!
//!  - Adjust the overflow policy via [`AsyncPoolSinkBuilder::overflow_policy`].
//!
//!  - Set up an error handler on asynchronous combined sinks via
//!    [`AsyncPoolSinkBuilder::error_handler`]. The handler will be called when
//!    a record is dropped or an operation has failed.
//!
//! [`Logger`]: crate::logger::Logger
//! [`ThreadPoolBuilder::capacity`]: crate::ThreadPoolBuilder::capacity

#[cfg(feature = "multi-thread")]
pub(crate) mod async_sink;
mod dedup_sink;
mod file_sink;
mod helper;
#[cfg(any(
    all(target_os = "linux", feature = "native", feature = "libsystemd"),
    all(doc, not(doctest))
))]
mod journald_sink;
mod rotating_file_sink;
mod std_stream_sink;
#[cfg(any(all(windows, feature = "native"), all(doc, not(doctest))))]
mod win_debug_sink;
mod write_sink;

#[cfg(feature = "multi-thread")]
pub use async_sink::*;
pub use dedup_sink::*;
pub use file_sink::*;
#[cfg(any(
    all(target_os = "linux", feature = "native", feature = "libsystemd"),
    all(doc, not(doctest))
))]
pub use journald_sink::*;
pub use rotating_file_sink::*;
pub use std_stream_sink::*;
#[cfg(any(all(windows, feature = "native"), all(doc, not(doctest))))]
pub use win_debug_sink::*;
pub use write_sink::*;

use crate::{formatter::Formatter, sync::*, ErrorHandler, Level, LevelFilter, Record, Result};

/// A trait for sinks.
pub trait Sink: Sync + Send {
    /// Determines if a log message with the specified level would be logged.
    #[must_use]
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
    #[must_use]
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

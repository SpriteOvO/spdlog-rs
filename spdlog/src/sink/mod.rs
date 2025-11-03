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
//! [`Logger`]: crate::logger::Logger

#[cfg(any(
    all(target_os = "android", feature = "native", feature = "android-ndk"),
    all(doc, not(doctest))
))]
mod android_sink;
#[cfg(feature = "multi-thread")]
pub(crate) mod async_sink;
mod dedup_sink;
mod file_sink;
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

use std::ops::Deref;

#[cfg(any(
    all(target_os = "android", feature = "native", feature = "android-ndk"),
    all(doc, not(doctest))
))]
pub use android_sink::*;
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

use crate::{
    formatter::{Formatter, FullFormatter},
    sync::*,
    Error, ErrorHandler, Level, LevelFilter, Record, Result,
};

pub(crate) const SINK_DEFAULT_LEVEL_FILTER: LevelFilter = LevelFilter::All;

/// Contains definitions of sink properties.
///
/// It provides a set of common properties for sink to define. If there is no
/// special need for properties, use it directly and then implement
/// [`GetSinkProp`] for your sink, a blanket implementation will be enabled,
/// which would eliminate a lot of boilerplate code.
///
/// If further customization of the properties is needed (e.g., using different
/// types, changing behavior), this struct is not needed. Instead, define
/// properties manually within your sink, and then implement [`SinkPropAccess`].
pub struct SinkProp {
    level_filter: Atomic<LevelFilter>,
    formatter: RwLockMappable<Box<dyn Formatter>>,
    error_handler: RwLock<ErrorHandler>,
}

impl Default for SinkProp {
    fn default() -> Self {
        Self {
            level_filter: Atomic::new(SINK_DEFAULT_LEVEL_FILTER),
            formatter: RwLockMappable::new(Box::new(FullFormatter::new())),
            error_handler: RwLock::new(ErrorHandler::default()),
        }
    }
}

impl SinkProp {
    /// Gets the log level filter.
    #[must_use]
    pub fn level_filter(&self) -> LevelFilter {
        self.level_filter.load(Ordering::Relaxed)
    }

    /// Sets the log level filter.
    pub fn set_level_filter(&self, level_filter: LevelFilter) {
        self.level_filter.store(level_filter, Ordering::Relaxed)
    }

    /// Gets the formatter.
    ///
    /// The returned value is a lock guard, so please avoid storing it in a
    /// variable with a longer lifetime.
    pub fn formatter<'a>(&'a self) -> impl Deref<Target = dyn Formatter> + 'a {
        RwLockMappableReadGuard::map(self.formatter.read(), |f| &**f)
    }

    /// Sets the formatter.
    pub fn set_formatter<F>(&self, formatter: F)
    where
        F: Formatter + 'static,
    {
        self.set_formatter_boxed(Box::new(formatter));
    }

    /// Sets the boxed formatter.
    pub fn set_formatter_boxed(&self, formatter: Box<dyn Formatter>) {
        *self.formatter.write() = formatter;
    }

    /// Calls the error handler with an error.
    pub fn call_error_handler(&self, err: Error) {
        self.error_handler.read_expect().call(err)
    }

    pub(crate) fn call_error_handler_internal(&self, from: impl AsRef<str>, err: Error) {
        self.error_handler.read_expect().call_internal(from, err)
    }

    /// Sets a error handler.
    ///
    /// Most errors that occur in `Sink` will be returned as directly as
    /// possible (e.g. returned to [`Logger`]), but some errors that cannot be
    /// returned immediately, this function will be called. For example,
    /// asynchronous errors.
    ///
    /// [`Logger`]: crate::logger::Logger
    pub fn set_error_handler<F: Into<ErrorHandler>>(&self, handler: F) {
        *self.error_handler.write_expect() = handler.into();
    }
}

/// Represents the getter for the [`SinkProp`] inside a sink.
///
/// This trait is not mandatory for a sink. It enables a blanket implementation,
/// where a sink that implements this trait will automatically get the
/// [`SinkPropAccess`] trait implemented, which eliminates a lot of boilerplate
/// code.
pub trait GetSinkProp {
    /// Gets the [`SinkProp`] from a sink.
    fn prop(&self) -> &SinkProp;
}

/// Represents getters for properties of a sink.
///
/// The use of a sink requires these properties, and this trait describes the
/// methods for getting them.
///
/// For the common case of custom sinks, users don't need to implement this
/// trait manually, they can just store a `SinkProp` in their sink struct and
/// implement trait [`GetSinkProp`], a blanket implementation will automatically
/// implement `SinkPropAccess` for the sink.
///
/// For more details on implementing custom sink, see [. /examples] directory.
///
/// [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/spdlog/examples
pub trait SinkPropAccess {
    /// Gets the log level filter.
    #[must_use]
    fn level_filter(&self) -> LevelFilter;

    /// Sets the log level filter.
    fn set_level_filter(&self, level_filter: LevelFilter);

    /// Sets the formatter.
    fn set_formatter(&self, formatter: Box<dyn Formatter>);

    /// Sets a error handler.
    ///
    /// Most errors that occur in `Sink` will be returned as directly as
    /// possible (e.g. returned to [`Logger`]), but some errors that cannot be
    /// returned immediately, this function will be called. For example,
    /// asynchronous errors.
    ///
    /// [`Logger`]: crate::logger::Logger
    fn set_error_handler(&self, handler: ErrorHandler);
}

impl<S: GetSinkProp> SinkPropAccess for S {
    fn level_filter(&self) -> LevelFilter {
        self.prop().level_filter()
    }

    fn set_level_filter(&self, level_filter: LevelFilter) {
        self.prop().set_level_filter(level_filter);
    }

    fn set_formatter(&self, formatter: Box<dyn Formatter>) {
        self.prop().set_formatter_boxed(formatter);
    }

    fn set_error_handler(&self, handler: ErrorHandler) {
        self.prop().set_error_handler(handler);
    }
}

/// Represents a sink
///
/// See [./examples] directory for how to implement a custom sink.
///
/// [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/spdlog/examples
pub trait Sink: SinkPropAccess + Sync + Send {
    /// Determines if a log message with the specified level would be logged.
    #[must_use]
    fn should_log(&self, level: Level) -> bool {
        self.level_filter().test(level)
    }

    /// Logs a record.
    fn log(&self, record: &Record) -> Result<()>;

    /// Flushes any buffered records.
    fn flush(&self) -> Result<()>;

    /// Flushes any buffered records at program exit.
    ///
    /// _spdlog-rs_ will perform a flush for sinks in the default logger when
    /// the program exits, and the flush will be called to this method
    /// `flush_on_exit` instead of `flush`. This is because the execution
    /// context may be in the [`atexit`] callback or in the panic handler when
    /// exiting. In such a context, some operations are restricted, e.g.
    /// Thread-local Storage (TLS) may not be available in `atexit` callbacks.
    ///
    /// This method calls directly to `flush` method by default. When users'
    /// `flush` method implementation is not usable in a program exit context,
    /// users should override the implementation of this method to provide an
    /// alternative flushing implementation. See the implementation of
    /// [`AsyncPoolSink::flush_on_exit`] as an example.
    ///
    /// For combined sinks, this method should always be overridden to propagate
    /// the information that "the program is exiting" to their sub-sinks. See
    /// the implementation of [`DedupSink::flush_on_exit`] as an example.
    ///
    /// [`atexit`]: https://en.cppreference.com/w/c/program/atexit
    fn flush_on_exit(&self) -> Result<()> {
        self.flush()
    }
}

/// Container type for [`Sink`]s.
pub type Sinks = Vec<Arc<dyn Sink>>;

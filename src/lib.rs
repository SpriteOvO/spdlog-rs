//! A fast and combinable Rust logging crate.
//!
//! It is inspired by the C++ logging library [spdlog], so if you are familiar
//! with C++ `spdlog`, you should be able to get started with this crate quite
//! easily. Of course, there are some differences, you can see [Significant
//! differences from C++ spdlog](#significant-differences-from-c-spdlog) below.
//!
//! # Getting started
//!
//! Add this to `Cargo.toml`:
//! ```toml
//! [dependencies]
//! spdlog-rs = "0.1"
//! ```
//!
//! `spdlog-rs` is out-of-the-box, it has a default logger, so users can output
//! logs to terminal by default without any configuration. For more details
//! about the default logger, please read the documentation of
//! [`default_logger`].
//!
//! The basic use of this crate is through these logging macros: [`trace!`],
//! [`debug!`], [`info!`], [`warn!`], [`error!`], [`critical!`] and [`log!`],
//! where [`critical!`] represents the most severe log messages and [`trace!`]
//! the most verbose. Each of these macros accept format strings similarly to
//! [`println!`]. All log macros and common types are already under [`prelude`]
//! module.
//!
//! [`Logger`] and [`Sink`] are the most important components of `spdlog-rs`.
//! Make sure to read their documentation. In short, a logger contains a
//! combination of sinks, and sinks implement writing log messages to actual
//! targets.
//!
//! ## Examples
//!
//! ```
//! use spdlog::prelude::*;
//!
//! info!("hello, {}", "world");
//! ```
//!
//! For more examples, see [./examples] directory.
//!
//! ## Help
//!
//! If you have any questions or need help while using this crate, feel free to
//! [open a discussion]. For feature requests or bug reports, please [open an
//! issue].
//!
//! # Compile time filters
//!
//! Log levels can be statically disabled at compile time via Cargo features.
//! Log invocations at disabled levels will be skipped and will not even be
//! present in the resulting binary. This level is configured separately for
//! release and debug builds. The features are:
//!
//!  - `level-off`
//!  - `level-error`
//!  - `level-warn`
//!  - `level-info`
//!  - `level-debug`
//!  - `level-trace`
//!  - `release-level-off`
//!  - `release-level-error`
//!  - `release-level-warn`
//!  - `release-level-info`
//!  - `release-level-debug`
//!  - `release-level-trace`
//!
//! These features control the value of the `STATIC_LEVEL_FILTER` constant. The
//! logging macros check this value before logging a message. By default, no
//! levels are disabled.
//!
//! For example, a crate can disable trace level logs in debug builds and trace,
//! debug, and info level logs in release builds with
//! `features = ["level-debug", "release-level-warn"]`.
//!
//! # Crate Feature Flags
//!
//! The following crate feature flags are available in addition to the filters.
//! They are configured in your `Cargo.toml`.
//!
//!  - `source-location` allows recording the source location of each log. When
//!    it is enabled the default formatter [`FullFormatter`] will always format
//!    the source location information. If you do not want the source location
//!    information to appear in your binary file, you may prefer not to enable
//!    it.
//!
//!  - `flexible-string` improves the performance of formatting records, however
//!    contains unsafe code. For more details, see the documentation of
//!    [`StringBuf`].
//!
//! # Significant differences from C++ spdlog
//!
//! The significant differences between `spdlog-rs` and C++ `spdlog`[^1]:
//!  - `spdlog-rs` does not have `registry`[^2]. You don't need to register for
//!    loggers.
//!
//!  - `spdlog-rs` does not have `backtrace`[^2].
//!
//!  - `spdlog-rs` currently does not have `pattern_formatter`. The solution for
//!    custom formatting log messages is to implement your own [`Formatter`]
//!    structure and then call [`Sink::set_formatter`].
//!
//!  - In `spdlog-rs`, [`LevelFilter`] is a more flexible and readable enum with
//!    logical conditions.
//!
//!  - In `spdlog-rs`, there is no "_st" sinks, all sinks are "_mt".
//!
//!  - `daily_file_sink` and `hourly_file_sink` in C++ `spdlog` are merged into
//!    [`RotatingFileSink`] in `spdlog-rs`. They correspond to rotation policies
//!    [`RotationPolicy::Daily`] and [`RotationPolicy::Hourly`].
//!
//!  - Some sinks in C++ `spdlog` are not yet implemented in `spdlog-rs`. (Yes,
//!    PRs are welcome)
//!
//!  - ...
//!
//! [^1]: At the time of writing this section, the latest version of C++ `spdlog` is v1.9.2.
//!
//! [^2]: C++ `spdlog` is also planned to remove it in v2.x.
//!
//! [spdlog]: https://github.com/gabime/spdlog
//! [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/examples
//! [open a discussion]: https://github.com/SpriteOvO/spdlog-rs/discussions/new
//! [open an issue]: https://github.com/SpriteOvO/spdlog-rs/issues/new/choose
//! [`FullFormatter`]: crate::formatter::FullFormatter
//! [`RotatingFileSink`]: crate::sink::RotatingFileSink
//! [`Formatter`]: crate::formatter::Formatter
//! [`RotationPolicy::Daily`]: crate::sink::RotationPolicy::Daily
//! [`RotationPolicy::Hourly`]: crate::sink::RotationPolicy::Hourly

#![warn(missing_docs)]

mod error;
pub mod formatter;
mod level;
mod log_macros;
mod logger;
mod periodic_worker;
mod record;
pub mod sink;
mod source_location;
#[doc(hidden)]
pub mod string_buf;
pub mod terminal_style;
#[cfg(test)]
mod test_utils;
mod utils;

pub use error::*;
pub use level::*;
pub use logger::*;
pub use record::*;
pub use source_location::*;
pub use string_buf::StringBuf;

/// Contains all log macros and common types.
pub mod prelude {
    pub use super::{critical, debug, error, info, log, trace, warn};
    pub use super::{Level, LevelFilter, Logger, LoggerBuilder};
}

use std::sync::Arc;

use arc_swap::ArcSwap;
use cfg_if::cfg_if;
use lazy_static::lazy_static;

use sink::{
    Sink, {StdStream, StdStreamSink},
};
use terminal_style::StyleMode;

/// The statically resolved log level filter.
///
/// See the crate level documentation for information on how to configure this.
///
/// This value is checked by the log macros, but not by [`Logger`]s and
/// [`Sink`]s. Code that manually calls functions on these should compare the
/// level against this value.
///
/// [`Logger`]: crate::logger::Logger
/// [`Sink`]: crate::sink::Sink
pub const STATIC_LEVEL_FILTER: LevelFilter = STATIC_LEVEL_FILTER_INNER;

cfg_if! {
    if #[cfg(all(not(debug_assertions), feature = "release-level-off"))] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::Off;
    } else if #[cfg(all(not(debug_assertions), feature = "release-level-critical"))] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Critical);
    } else if #[cfg(all(not(debug_assertions), feature = "release-level-error"))] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Error);
    } else if #[cfg(all(not(debug_assertions), feature = "release-level-warn"))] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Warn);
    } else if #[cfg(all(not(debug_assertions), feature = "release-level-info"))] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Info);
    } else if #[cfg(all(not(debug_assertions), feature = "release-level-debug"))] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Debug);
    } else if #[cfg(all(not(debug_assertions), feature = "release-level-trace"))] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Trace);
    } else if #[cfg(feature = "level-off")] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::Off;
    } else if #[cfg(feature = "level-critical")] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Critical);
    } else if #[cfg(feature = "level-error")] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Error);
    } else if #[cfg(feature = "level-warn")] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Warn);
    } else if #[cfg(feature = "level-info")] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Info);
    } else if #[cfg(feature = "level-debug")] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Debug);
    } else {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Trace);
    }
}

#[cfg(not(windows))]
pub(crate) const EOL: &str = "\n";
#[cfg(windows)]
pub(crate) const EOL: &str = "\r\n";

lazy_static! {
    static ref DEFAULT_LOGGER: ArcSwap<Logger> = {
        let stdout = StdStreamSink::new(StdStream::Stdout, StyleMode::Auto);
        stdout.set_level_filter(LevelFilter::MoreVerbose(Level::Warn));

        let stderr = StdStreamSink::new(StdStream::Stderr, StyleMode::Auto);
        stderr.set_level_filter(LevelFilter::MoreSevereEqual(Level::Warn));

        let sinks: [Arc<dyn Sink>; 2] = [Arc::new(stdout), Arc::new(stderr)];

        ArcSwap::from_pointee(Logger::builder().sinks(sinks).build())
    };
}

/// Returns an [`Arc`] default logger.
///
/// Default logger contains two [`StdStreamSink`]s, writing logs on `info` level
/// and more verbose levels to `stdout`, and writing logs on `warn` level and
/// more severe levels to `stderr`.
///
/// # Examples
///
/// ```
/// # use std::sync::Arc;
/// use spdlog::prelude::*;
///
/// let default_logger: Arc<Logger> = spdlog::default_logger();
///
/// default_logger.set_level_filter(LevelFilter::All);
///
/// info!("this log will be written to `stdout`");
/// debug!("this log will be written to `stdout`");
/// trace!("this log will be written to `stdout`");
///
/// warn!("this log will be written to `stderr`");
/// error!("this log will be written to `stderr`");
/// critical!("this log will be written to `stderr`");
/// ```
pub fn default_logger() -> Arc<Logger> {
    DEFAULT_LOGGER.load().clone()
}

/// Sets the given logger as the default logger, and returns the old default
/// logger.
///
/// # Examples
///
/// ```
/// # use std::sync::Arc;
/// use spdlog::prelude::*;
///
/// # let new_logger = spdlog::default_logger();
/// let old_logger: Arc<Logger> = spdlog::swap_default_logger(new_logger);
///
/// info!("this log will be handled by `new_logger`");
/// info!(logger: old_logger, "this log will be handled by `old_logger`");
/// ```
pub fn swap_default_logger(logger: Arc<Logger>) -> Arc<Logger> {
    DEFAULT_LOGGER.swap(logger)
}

/// Sets the given logger as the default logger.
///
/// # Examples
///
/// ```
/// # use std::sync::Arc;
/// use spdlog::prelude::*;
///
/// # let new_logger = spdlog::default_logger();
/// spdlog::set_default_logger(new_logger);
///
/// info!("this log will be handled by `new_logger`");
/// ```
pub fn set_default_logger(logger: Arc<Logger>) {
    swap_default_logger(logger);
}

fn default_error_handler(from: impl AsRef<str>, error: Error) {
    let date = chrono::Local::now()
        .format("%Y-%m-%d %H:%M:%S.%3f")
        .to_string();

    eprintln!(
        "[*** SPDLOG-RS UNHANDLED ERROR ***] [{}] [{}] {}",
        date,
        from.as_ref(),
        error
    );
}

// Used at log macros
#[doc(hidden)]
pub fn __log(logger: &Logger, level: Level, fmt_args: std::fmt::Arguments) {
    // use `Cow` to avoid allocation as much as we can
    let payload: std::borrow::Cow<str> = match fmt_args.as_str() {
        Some(literal_str) => literal_str.into(), // no format arguments, so it is a `&'static str`
        None => fmt_args.to_string().into(),
    };

    let mut builder = Record::builder(level, payload).source_location(source_location_current!());
    if let Some(logger_name) = logger.name() {
        builder = builder.logger_name(logger_name);
    }
    logger.log(&builder.build());
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_utils::*;

    #[test]
    fn test_default_logger() {
        let test_sink = Arc::new(CounterSink::new());

        let test_logger = Arc::new(test_logger_builder().sink(test_sink.clone()).build());
        let empty_logger = Arc::new(Logger::builder().build());

        set_default_logger(empty_logger.clone());
        info!("hello");
        error!("world");

        set_default_logger(test_logger);
        warn!("hello");
        error!("rust");

        set_default_logger(empty_logger);
        info!("hello");
        error!("spdlog");

        assert_eq!(test_sink.log_count(), 2);
        assert_eq!(
            test_sink.payloads(),
            vec!["hello".to_string(), "rust".to_string()]
        );
    }
}

//! A fast and flexible Rust logging library.
//!
//! Inspired by the C++ logging library [spdlog](https://github.com/gabime/spdlog).
//!
//! # Compile time filters
//!
//! Log levels can be statically disabled at compile time via Cargo features.
//! Log invocations at disabled levels will be skipped and will not even be
//! present in the resulting binary. This level is configured separately for
//! release and debug builds. The features are:
//!
//! * `level-off`
//! * `level-error`
//! * `level-warn`
//! * `level-info`
//! * `level-debug`
//! * `level-trace`
//! * `release-level-off`
//! * `release-level-error`
//! * `release-level-warn`
//! * `release-level-info`
//! * `release-level-debug`
//! * `release-level-trace`
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
//! * `source-location` allows recording the source location of each log, and it
//!   is performance cheap to enable it. If you do not want the source location
//!   information to appear in the binary file, you may prefer not to enable it.

#![warn(missing_docs)]

pub mod error;
pub mod formatter;
pub mod level;
mod log_macros;
pub mod logger;
mod periodic_worker;
pub mod record;
pub mod sink;
pub mod source_location;
pub mod string_buf;
pub mod terminal_style;
#[cfg(test)]
pub mod test_utils;
mod utils;

pub use error::{Error, ErrorHandler, Result};
pub use level::{Level, LevelFilter};
pub use logger::{Logger, LoggerBuilder};
pub use record::Record;
pub use source_location::SourceLocation;
pub use string_buf::StringBuf;

/// Contains available log macros and common types.
pub mod prelude {
    pub use super::{critical, debug, error, info, log, trace, warn};
    pub use super::{Level, LevelFilter, Logger, LoggerBuilder};
}

use std::sync::Arc;

use arc_swap::ArcSwap;
use cfg_if::cfg_if;
use lazy_static::lazy_static;

use sink::{
    std_out_stream_style_sink::{StdOutStream, StdOutStreamStyleSink},
    Sink,
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
        let stdout = StdOutStreamStyleSink::new(StdOutStream::Stdout, StyleMode::Auto);
        stdout.set_level_filter(LevelFilter::MoreVerbose(Level::Warn));

        let stderr = StdOutStreamStyleSink::new(StdOutStream::Stderr, StyleMode::Auto);
        stderr.set_level_filter(LevelFilter::MoreSevereEqual(Level::Warn));

        let sinks: [Arc<dyn Sink>; 2] = [Arc::new(stdout), Arc::new(stderr)];

        ArcSwap::from_pointee(Logger::builder().sinks(sinks).build())
    };
}

/// Returns a reference to the default logger.
pub fn default_logger() -> Arc<Logger> {
    DEFAULT_LOGGER.load().clone()
}

/// Swaps the default logger to the given logger.
pub fn swap_default_logger(logger: Arc<Logger>) -> Arc<Logger> {
    DEFAULT_LOGGER.swap(logger)
}

/// Sets the default logger to the given logger.
pub fn set_default_logger(logger: Arc<Logger>) {
    swap_default_logger(logger);
}

fn default_error_handler(from: impl AsRef<str>, error: Error) {
    let date = chrono::Local::now()
        .format("%Y-%m-%d %H:%M:%S.%3f")
        .to_string();

    eprintln!(
        "[*** SPDLOG UNHANDLED ERROR ***] [{}] [{}] {}",
        date,
        from.as_ref(),
        error
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_utils::*;

    #[test]
    fn test_default_logger() {
        let test_sink = Arc::new(CounterSink::new());

        let test_logger = Arc::new(test_logger_builder().sink(test_sink.clone()).build());
        let empty_logger = Arc::new(Logger::new());

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

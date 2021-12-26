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
//! * `max_level_off`
//! * `max_level_error`
//! * `max_level_warn`
//! * `max_level_info`
//! * `max_level_debug`
//! * `max_level_trace`
//! * `release_max_level_off`
//! * `release_max_level_error`
//! * `release_max_level_warn`
//! * `release_max_level_info`
//! * `release_max_level_debug`
//! * `release_max_level_trace`
//!
//! These features control the value of the `STATIC_MAX_LEVEL` constant. The
//! logging macros check this value before logging a message. By default, no
//! levels are disabled.
//!
//! For example, a crate can disable trace level logs in debug builds and trace,
//! debug, and info level logs in release builds with `features =
//! ["max_level_debug", "release_max_level_warn"]`.
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
pub mod record;
pub mod sink;
pub mod source_location;
pub mod str_buf;
pub mod terminal;
#[cfg(test)]
pub mod test_utils;

pub use error::{Error, ErrorHandler, Result};
pub use level::{Level, LevelFilter};
pub use record::Record;
pub use source_location::SourceLocation;
pub use str_buf::StrBuf;

use std::sync::{Arc, RwLock};

use cfg_if::cfg_if;
use lazy_static::lazy_static;

use sink::StdoutStyleSink;

/// The statically resolved maximum log level.
///
/// See the crate level documentation for information on how to configure this.
///
/// This value is checked by the log macros, but not by [`Logger`]s and
/// [`Sink`]s. Code that manually calls functions on these should compare the
/// level against this value.
///
/// [`Logger`]: crate::logger::Logger
/// [`Sink`]: crate::sink::Sink
pub const STATIC_MAX_LEVEL: LevelFilter = MAX_LEVEL_INNER;

cfg_if! {
    if #[cfg(all(not(debug_assertions), feature = "release_max_level_off"))] {
        const MAX_LEVEL_INNER: LevelFilter = LevelFilter::Off;
    } else if #[cfg(all(not(debug_assertions), feature = "release_max_level_error"))] {
        const MAX_LEVEL_INNER: LevelFilter = LevelFilter::Error;
    } else if #[cfg(all(not(debug_assertions), feature = "release_max_level_warn"))] {
        const MAX_LEVEL_INNER: LevelFilter = LevelFilter::Warn;
    } else if #[cfg(all(not(debug_assertions), feature = "release_max_level_info"))] {
        const MAX_LEVEL_INNER: LevelFilter = LevelFilter::Info;
    } else if #[cfg(all(not(debug_assertions), feature = "release_max_level_debug"))] {
        const MAX_LEVEL_INNER: LevelFilter = LevelFilter::Debug;
    } else if #[cfg(all(not(debug_assertions), feature = "release_max_level_trace"))] {
        const MAX_LEVEL_INNER: LevelFilter = LevelFilter::Trace;
    } else if #[cfg(feature = "max_level_off")] {
        const MAX_LEVEL_INNER: LevelFilter = LevelFilter::Off;
    } else if #[cfg(feature = "max_level_error")] {
        const MAX_LEVEL_INNER: LevelFilter = LevelFilter::Error;
    } else if #[cfg(feature = "max_level_warn")] {
        const MAX_LEVEL_INNER: LevelFilter = LevelFilter::Warn;
    } else if #[cfg(feature = "max_level_info")] {
        const MAX_LEVEL_INNER: LevelFilter = LevelFilter::Info;
    } else if #[cfg(feature = "max_level_debug")] {
        const MAX_LEVEL_INNER: LevelFilter = LevelFilter::Debug;
    } else {
        const MAX_LEVEL_INNER: LevelFilter = LevelFilter::Trace;
    }
}

lazy_static! {
    static ref DEFAULT_LOGGER: RwLock<Arc<dyn logger::Logger>> = RwLock::new(Arc::new(
        logger::BasicLogger::with_sink(Arc::new(StdoutStyleSink::default()))
    ));
}

/// Initializes the crate
///
/// Users should initialize early at runtime and should only initialize once.
pub fn init() {
    lazy_static::initialize(&DEFAULT_LOGGER);
}

/// Returns a reference to the default logger.
pub fn default_logger() -> Arc<dyn logger::Logger> {
    DEFAULT_LOGGER.read().unwrap().clone()
}

/// Sets the default logger to the given logger.
pub fn set_default_logger(logger: Arc<dyn logger::Logger>) {
    *DEFAULT_LOGGER.write().unwrap() = logger;
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_utils::{EmptyLogger, TestLogger};

    #[test]
    fn test_default_logger() {
        let test_logger = Arc::new(TestLogger::new());
        let empty_logger = Arc::new(EmptyLogger::new());

        set_default_logger(empty_logger.clone());
        info!("hello");
        error!("world");

        set_default_logger(test_logger.clone());
        warn!("hello");
        error!("rust");

        set_default_logger(empty_logger);
        info!("hello");
        error!("spdlog");

        assert_eq!(test_logger.log_counter(), 2);
        assert_eq!(
            test_logger.payloads(),
            vec!["hello".to_string(), "rust".to_string()]
        );
    }
}

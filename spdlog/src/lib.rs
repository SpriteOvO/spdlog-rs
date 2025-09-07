//! Fast, highly configurable Rust logging crate.
//!
//! It is inspired by the C++ logging library [spdlog], and we share most of the
//! same concepts. So if you are already familiar with C++ `spdlog`, you should
//! be able to get started with this crate quite easily. Of course, there are
//! some differences, you can see [Significant differences from C++
//! spdlog](#significant-differences-from-c-spdlog) below.
//!
//! # Table of contents
//!
//! - [Getting started](#getting-started)
//! - [Compile-time filters](#compile-time-filters)
//! - [Crate feature flags](#crate-feature-flags)
//! - [Supported Rust versions](#supported-rust-versions)
//! - Overview of features
//!   - [Configured via environment variable](init_env_level)
//!   - [Compile-time and runtime pattern formatter]
//!   - [Asynchronous support]
//!   - [Structured logging]
//!   - [Compatible with log crate](LogCrateProxy)
//!
//! [Compile-time and runtime pattern formatter]: formatter/index.html#compile-time-and-runtime-pattern-formatter
//! [Asynchronous support]: crate::sink::AsyncPoolSink
//! [Structured logging]: crate::kv
//!
//! # Getting started
//!
//! Add this to `Cargo.toml`:
//! ```toml
//! [dependencies]
//! spdlog-rs = "0.4"
//! ```
//!
//! `spdlog-rs` is highly configurable, and also works out-of-the-box for
//! lightweight projects. By default, logs will be output to `stdout` and
//! `stderr`.
//!
//! ```
//! use spdlog::prelude::*;
//!
//! // Non-severe logs (trace, debug) are ignored by default.
//! // If you wish to enable all logs, call
//! spdlog::default_logger().set_level_filter(spdlog::LevelFilter::All);
//!
//! info!("hello, world!");
//! error!("oops!");
//! debug!("3 + 2 = {}", 5);
//! ```
//!
//! Output:
//!
//! <pre>
//! [2022-11-02 09:23:12.263] [<font color="#0DBC79">info</font>] hello, world!
//! [2022-11-02 09:23:12.263] [<font color="#F35E5E">error</font>] oops!
//! [2022-11-02 09:23:12.263] [<font color="#11A8CD">debug</font>] 3 + 2 = 5
//! </pre>
//!
//! The basic use is through these logging macros: [`trace!`], [`debug!`],
//! [`info!`], [`warn!`], [`error!`], [`critical!`], where `critical!`
//! represents the most severe logs and `trace!` the most verbose. Each of these
//! macros accept format strings similarly to [`println!`]. All log macros
//! and common types are already under [`prelude`] module.
//!
//! ## Sink
//!
//! Many real programs want more than just displaying logs to the terminal.
//!
//! [`Sink`]s are the objects that actually write logs to their targets. If you
//! want logs to be written to files as well, [`FileSink`] is what you need.
//!
//! ```
//! # use std::sync::Arc;
//! use spdlog::{prelude::*, sink::FileSink};
//!
//! # fn main() -> spdlog::Result<()> {
//! let path = "path/to/somewhere.log";
//!
//! # let path = concat!(env!("OUT_DIR"), "/doctest-out/crate-1.txt");
//! let new_logger = spdlog::default_logger().fork_with(|new| {
//!     let file_sink = Arc::new(FileSink::builder().path(path).build()?);
//!     new.sinks_mut().push(file_sink);
//!     Ok(())
//! })?;
//! # let backup = spdlog::default_logger();
//! spdlog::set_default_logger(new_logger);
//!
//! info!("from now on, logs will be written to both stdout/stderr and the file");
//! # spdlog::set_default_logger(backup);
//! # Ok(()) }
//! ```
//!
//! Take a look at [`sink`] module for more interesting sinks, such as
//! [`RotatingFileSink`] that automatically rotates files by time point or file
//! size, and [`AsyncPoolSink`] that outputs logs asynchronously.
//!
//! ## Logger
//!
//! A complex program may consist of many separated components.
//!
//! [`Logger`] manages, controls and manipulates multiple sinks within it. In
//! addition to having the global [`default_logger`], more loggers are allowed
//! to be configured, stored and used independently.
//!
//! Logging macros provide an optional parameter `logger`. If it is specified,
//! logs will be processed by the specified logger instead of the global default
//! logger.
//!
//! And benefiting from the fact that a logger uses `Arc` to store sinks, a sink
//! can be set and used by more than one logger, and you can combine them as you
//! like.
//!
//! ```
//! use spdlog::prelude::*;
//! # use spdlog::Result;
//!
//! struct AppDatabase {
//!     logger: Logger,
//!     // Database info...
//! }
//!
//! impl AppDatabase {
//!     fn new() -> Result<Self> {
//!         let logger = Logger::builder()
//!             .name("database")
//!             // .sink( ... )
//!             // .sink( ... )
//!             // .level_filter( ... )
//!             // ...
//!             .build()?;
//!         Ok(Self { logger, /* Database info... */ })
//!     }
//!
//!     fn query<T>(&self) -> T {
//!         let data = /* Query from the database */
//!         # 114514;
//!         trace!(logger: self.logger, "queried data {}", data);
//!         data
//!         # ; unreachable!()
//!     }
//! }
//!
//! struct AppNetwork { /* ... */ }
//! struct AppAuth { /* ... */ }
//! struct AppBlahBlah { /* ... */ }
//! ```
//!
//! ## Learn more
//!
//! Directory [./examples] contains more advanced usage examples. You can learn
//! them along with their documentation.
//!
//! If you have any trouble while using this crate, please don't hesitate to
//! [open a discussion] for help. For feature requests or bug reports, please
//! [open an issue].
//!
//! # Compile-time filters
//!
//! Log levels can be statically disabled at compile time via Cargo features.
//! Log invocations at disabled levels will be skipped and will not even be
//! present in the resulting binary. This level is configured separately for
//! release and debug builds. The features are:
//!
//!  - `level-off`
//!  - `level-critical`
//!  - `level-error`
//!  - `level-warn`
//!  - `level-info`
//!  - `level-debug`
//!  - `level-trace`
//!  - `release-level-off`
//!  - `release-level-critical`
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
//! # Crate feature flags
//!
//! The following crate feature flags are available in addition to the filters.
//! They are configured in your `Cargo.toml`.
//!
//!  - `source-location` allows recording the source location of each log. When
//!    it is enabled, the source location will be present in [`Record`] and
//!    visible to [`Formatter`], and formatting patterns related to source
//!    location will be available. If you do not want the source location
//!    information to appear in your binary file, you may prefer not to enable
//!    it.
//!
//!  - `flexible-string` improves the performance of formatting records, however
//!    it contains unsafe code. For more details, see the documentation of
//!    [`StringBuf`].
//!
//!  - `log` enables the compatibility with [log crate].
//!
//!  - `native` enables platform-specific components, such as
//!    [`sink::WinDebugSink`] for Windows, [`sink::JournaldSink`] for Linux,
//!    etc. Note If the component requires additional system dependencies, then
//!    more granular features need to be enabled as well.
//!
//!  - `runtime-pattern` enables the ability to build patterns with runtime
//!    template string. See [`RuntimePattern`] for more details.
//!
//!  - `serde_json` enables [`formatter::JsonFormatter`].
//!
//! # Supported Rust versions
//!
//! <!--
//! When updating this, also update:
//! - .github/workflows/ci.yml
//! - Cargo.toml
//! - README.md
//! -->
//!
//! The current minimum supported Rust version is 1.61.
//!
//! `spdlog-rs` is built against the latest Rust stable release, it is not
//! guaranteed to build on Rust versions earlier than the minimum supported
//! version.
//!
//! `spdlog-rs` follows the compiler support policy that the latest stable
//! version and the 3 most recent minor versions before that are always
//! supported. For example, if the current latest Rust stable version is 1.61,
//! the minimum supported version will not be increased past 1.58. Increasing
//! the minimum supported version is not considered a semver breaking change as
//! long as it complies with this policy.
//!
//! # Significant differences from C++ spdlog
//!
//! The significant differences between `spdlog-rs` and C++ `spdlog`[^1]:
//!  - `spdlog-rs` does not have `registry`[^2]. You don't need to register for
//!    loggers.
//!
//!  - `spdlog-rs` does not have `backtrace`[^2].
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
//!  - `async_logger` in C++ `spdlog` is [`AsyncPoolSink`] in `spdlog-rs`. This
//!    allows it to be used with synchronous sinks.
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
//! [`FileSink`]: crate::sink::FileSink
//! [`RotatingFileSink`]: crate::sink::RotatingFileSink
//! [`AsyncPoolSink`]: crate::sink::AsyncPoolSink
//! [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/spdlog/examples
//! [open a discussion]: https://github.com/SpriteOvO/spdlog-rs/discussions/new
//! [open an issue]: https://github.com/SpriteOvO/spdlog-rs/issues/new/choose
//! [log crate]: https://crates.io/crates/log
//! [`Formatter`]: crate::formatter::Formatter
//! [`RuntimePattern`]: crate::formatter::RuntimePattern
//! [`RotationPolicy::Daily`]: crate::sink::RotationPolicy::Daily
//! [`RotationPolicy::Hourly`]: crate::sink::RotationPolicy::Hourly

#![allow(unexpected_cfgs)]
// Credits: https://blog.wnut.pw/2020/03/24/documentation-and-unstable-rustdoc-features/
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(missing_docs)]

// Used for referencing from proc-macros
// Credits: https://stackoverflow.com/a/57049687
extern crate self as spdlog;

mod env_level;
pub mod error;
pub mod formatter;
pub mod kv;
mod level;
#[cfg(feature = "log")]
mod log_crate_proxy;
mod log_macros;
mod logger;
mod periodic_worker;
pub mod re_export;
mod record;
pub mod sink;
mod source_location;
mod string_buf;
mod sync;
pub mod terminal_style;
#[cfg(test)]
mod test_utils;
#[cfg(feature = "multi-thread")]
mod thread_pool;
mod utils;

pub use error::{Error, ErrorHandler, Result};
pub use level::*;
#[cfg(feature = "log")]
pub use log_crate_proxy::*;
pub use logger::*;
pub use record::*;
pub use source_location::*;
#[doc(hidden)]
pub use spdlog_macros::normalize_forward as __normalize_forward;
pub use string_buf::StringBuf;
#[cfg(feature = "multi-thread")]
pub use thread_pool::*;

/// Contains all log macros and common types.
pub mod prelude {
    pub use super::{
        critical, debug, error, info, log, trace, warn, Level, LevelFilter, Logger, LoggerBuilder,
    };
}

use std::{
    borrow::Cow,
    env::{self, VarError},
    ffi::OsStr,
    fmt,
    io::{self, Write},
    panic,
    result::Result as StdResult,
};

use cfg_if::cfg_if;
use error::EnvLevelError;
use sink::{Sink, StdStreamSink};
use sync::*;

/// The statically resolved log level filter.
///
/// See the crate level documentation for information on how to configure this.
///
/// This value is checked by the log macros, but not by [`Logger`]s and
/// [`Sink`]s. Code that manually calls functions on these should test the level
/// against this value.
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
#[doc(hidden)]
pub const __EOL: &str = "\n";
#[cfg(windows)]
#[doc(hidden)]
pub const __EOL: &str = "\r\n";

static DEFAULT_LOGGER: OnceCell<ArcSwap<Logger>> = OnceCell::new();

#[must_use]
fn default_logger_ref() -> &'static ArcSwap<Logger> {
    DEFAULT_LOGGER.get_or_init(|| {
        let stdout = StdStreamSink::builder()
            .stdout()
            .level_filter(LevelFilter::MoreVerbose(Level::Warn))
            .build()
            .unwrap();

        let stderr = StdStreamSink::builder()
            .stderr()
            .level_filter(LevelFilter::MoreSevereEqual(Level::Warn))
            .build()
            .unwrap();

        let sinks: [Arc<dyn Sink>; 2] = [Arc::new(stdout), Arc::new(stderr)];

        let res = ArcSwap::from_pointee(Logger::builder().sinks(sinks).build_default().unwrap());

        flush_default_logger_at_exit();
        res
    })
}

/// Returns the global default logger.
///
/// This default logger will be used by logging macros, if the `logger`
/// parameter is not specified when logging macros are called.
///
/// If the default logger has not been replaced, the default:
///
///  - contains a sink [`StdStreamSink`], writing logs on [`Level::Info`] and
///    more verbose levels to `stdout`.
///
///  - contains a sink [`StdStreamSink`], writing logs on [`Level::Warn`] level
///    and more severe levels to `stderr`.
///
///  - level filter ignores logs on [`Level::Debug`] and more verbose levels.
///
///    However, if you want to enable logging for all levels:
///    ```
///    use spdlog::prelude::*;
///
///    spdlog::default_logger().set_level_filter(LevelFilter::All);
///    ```
///
/// Users can replace the default logger with [`set_default_logger`] or
/// [`swap_default_logger`].
///
/// # Examples
///
/// ```
/// # use std::sync::Arc;
/// use spdlog::prelude::*;
///
/// let default_logger = spdlog::default_logger();
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
#[must_use]
pub fn default_logger() -> Arc<Logger> {
    default_logger_ref().load().clone()
}

/// Sets the given logger as the new global default logger, and returns the old
/// one.
///
/// # Examples
///
/// ```
/// # use std::sync::Arc;
/// use spdlog::prelude::*;
///
/// let new_logger: Arc<Logger> = /* ... */
/// # spdlog::default_logger();
/// let old_logger = spdlog::swap_default_logger(new_logger);
///
/// info!("this log will be handled by `new_logger`");
/// info!(logger: old_logger, "this log will be handled by `old_logger`");
/// ```
pub fn swap_default_logger(logger: Arc<Logger>) -> Arc<Logger> {
    default_logger_ref().swap(logger)
}

/// Sets the given logger as the new global default logger.
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

/// Initializes environment variable level filters from environment variable
/// `SPDLOG_RS_LEVEL`.
///
/// Returns whether the level in the environment variable was applied if there
/// are no errors.
///
/// The default level filter of loggers built after calling this function will
/// be configured based on the value of environment variable `SPDLOG_RS_LEVEL`.
///
/// If you want to read from a custom environment variable, see
/// [`init_env_level_from`].
///
/// Users should call this function early, the level filter of loggers built
/// before calling this function will not be configured by environment variable.
///
/// ## Formats of the environment variable value
///
/// The levels contained in the environment variable mean
/// `LevelFilter::MoreSevereEqual(level)`.
///
/// ---
///
/// - Specifies the level filter for ***the default logger***.
///
///   Possible inputs: `off`, `trace`, `warn`, `all`, etc.
///
/// ---
///
/// - Specifies the level filter for ***unnamed loggers***.
///
///   Possible inputs: `=off`, `=info`, `=error`, `=all`, etc.
///
/// ---
///
/// - Specifies the level filter for ***loggers with the specified name***.
///
///   Possible inputs: `logger-name=info`, `network=warn`, `core=info`,
///   `gui=critical`, etc.
///
/// ---
///
/// - Specifies the level filter for ***all loggers except the default logger***
///   (respect the above rules first if they are matched).
///
///   Possible inputs: `*=error`, `*=off`, `*=critical`, etc.
///
/// ---
///
/// The levels are not case-sensitive, and these rules are combinable, separated
/// by commas.
///
/// For example, these are legal:
///
/// ---
///
/// - `ALL,*=ALL`
///
///   Specifies the level filter for all loggers as `LevelFilter::All`.
///
/// ---
///
/// - `off,*=ERROR`
///
///   Specifies the level filter for the default logger as `LevelFilter::Off`,
///   the rest of loggers as `LevelFilter::MoreSevereEqual(Level::Error)`.
///
/// ---
///
/// - `gui=warn,network=trace`
///
///   Specifies the level filter for loggers with name "gui" as
///   `LevelFilter::MoreSevereEqual(Level::Warn)`, loggers with name "network"
///   as `LevelFilter::MoreSevereEqual(Level::Trace)`.
///
/// ---
///
/// However, the same rule cannot be specified more than once.
///
/// # Examples
///
/// - Environment variable `SPDLOG_RS_LEVEL` is not present:
///
///   ```
///   use spdlog::prelude::*;
///
///   # fn main() -> Result<(), Box<dyn std::error::Error>> {
///   assert_eq!(spdlog::init_env_level()?, false);
///
///   assert_eq!(
///       spdlog::default_logger().level_filter(),
///       LevelFilter::MoreSevereEqual(Level::Info) // default level filter
///   );
///   assert_eq!(
///       Logger::builder().build()?.level_filter(), // unnamed logger
///       LevelFilter::MoreSevereEqual(Level::Info) // default level filter
///   );
///   assert_eq!(
///       Logger::builder().name("gui").build()?.level_filter(),
///       LevelFilter::MoreSevereEqual(Level::Info) // default level filter
///   );
///   assert_eq!(
///       Logger::builder().name("network").build()?.level_filter(),
///       LevelFilter::MoreSevereEqual(Level::Info) // default level filter
///   );
///   # Ok(()) }
///   ```
///
/// ---
///
/// - `SPDLOG_RS_LEVEL="TRACE,network=Warn,*=error"`:
///
///   ```
///   use spdlog::prelude::*;
///
///   # fn main() -> Result<(), Box<dyn std::error::Error>> {
///   # std::env::set_var("SPDLOG_RS_LEVEL", "TRACE,network=Warn,*=error");
///   assert_eq!(spdlog::init_env_level()?, true);
///
///   assert_eq!(
///       spdlog::default_logger().level_filter(),
///       LevelFilter::MoreSevereEqual(Level::Trace)
///   );
///   assert_eq!(
///       Logger::builder().build()?.level_filter(), // unnamed logger
///       LevelFilter::MoreSevereEqual(Level::Error)
///   );
///   assert_eq!(
///       Logger::builder().name("gui").build()?.level_filter(),
///       LevelFilter::MoreSevereEqual(Level::Error)
///   );
///   assert_eq!(
///       Logger::builder().name("network").build()?.level_filter(),
///       LevelFilter::MoreSevereEqual(Level::Warn)
///   );
///   # Ok(()) }
///   ```
///
/// ---
///
/// - `SPDLOG_RS_LEVEL="network=Warn,network=Warn"` will fail, as the same rule
///   is specified multiple times.
///
///   ```
///   # std::env::set_var("SPDLOG_RS_LEVEL", "network=Warn,network=Warn");
///   assert!(matches!(
///       spdlog::init_env_level(),
///       Err(spdlog::error::EnvLevelError::ParseEnvVar(_))
///   ));
///   ```
pub fn init_env_level() -> StdResult<bool, EnvLevelError> {
    init_env_level_from("SPDLOG_RS_LEVEL")
}

/// Initializes environment variable level filters from a specified environment
/// variable.
///
/// For more information, see [`init_env_level`].
///
/// # Examples
///
/// - `MY_APP_LOG_LEVEL="TRACE,network=Warn,*=error"`:
///
///   ```
///   use spdlog::prelude::*;
///
///   # fn main() -> Result<(), Box<dyn std::error::Error>> {
///   # std::env::set_var("MY_APP_LOG_LEVEL", "TRACE,network=Warn,*=error");
///   assert_eq!(spdlog::init_env_level_from("MY_APP_LOG_LEVEL")?, true);
///
///   assert_eq!(
///       spdlog::default_logger().level_filter(),
///       LevelFilter::MoreSevereEqual(Level::Trace)
///   );
///   assert_eq!(
///       Logger::builder().build()?.level_filter(), // unnamed logger
///       LevelFilter::MoreSevereEqual(Level::Error)
///   );
///   assert_eq!(
///       Logger::builder().name("gui").build()?.level_filter(),
///       LevelFilter::MoreSevereEqual(Level::Error)
///   );
///   assert_eq!(
///       Logger::builder().name("network").build()?.level_filter(),
///       LevelFilter::MoreSevereEqual(Level::Warn)
///   );
///   # Ok(()) }
///   ```
///
/// For more examples, see [`init_env_level`].
pub fn init_env_level_from<K: AsRef<OsStr>>(env_key: K) -> StdResult<bool, EnvLevelError> {
    let var = match env::var(env_key.as_ref()) {
        Err(VarError::NotPresent) => return Ok(false),
        Err(err) => return Err(EnvLevelError::FetchEnvVar(err)),
        Ok(var) => var,
    };
    env_level::from_str(&var)?;
    Ok(true)
}

/// Initializes the log crate proxy.
///
/// This function calls [`log::set_logger`] to set up a [`LogCrateProxy`] and
/// all logs from log crate will be forwarded to `spdlog-rs`'s logger.
///
/// Users should call this function only once, and then configure the proxy by
/// calling [`log_crate_proxy()`].
///
/// Note that the `log` crate uses a different log level filter and by default
/// it rejects all log messages. To log messages via the `log` crate, you have
/// to call [`log::set_max_level`] manually before logging. For more
/// information, please read the upstream documentation of
/// [`log::set_max_level`].
#[cfg(feature = "log")]
pub fn init_log_crate_proxy() -> StdResult<(), re_export::log::SetLoggerError> {
    log::set_logger(log_crate_proxy())
}

/// Returns the global instance of log crate proxy.
#[cfg(feature = "log")]
#[must_use]
pub fn log_crate_proxy() -> &'static LogCrateProxy {
    static PROXY: Lazy<LogCrateProxy> = Lazy::new(LogCrateProxy::new);
    &PROXY
}

static IS_TEARING_DOWN: AtomicBool = AtomicBool::new(false);

fn flush_default_logger_at_exit() {
    // Rust never calls `drop` for static variables.
    //
    // Setting up an exit handler gives us a chance to flush the default logger
    // once at the program exit, thus we don't lose the last logs.

    extern "C" fn handler() {
        IS_TEARING_DOWN.store(true, Ordering::SeqCst);
        if let Some(default_logger) = DEFAULT_LOGGER.get() {
            default_logger.load().flush()
        }
    }

    #[must_use]
    fn try_atexit() -> bool {
        use std::os::raw::c_int;

        extern "C" {
            fn atexit(cb: extern "C" fn()) -> c_int;
        }

        (unsafe { atexit(handler) }) == 0
    }

    fn hook_panic() {
        let previous_hook = panic::take_hook();

        panic::set_hook(Box::new(move |info| {
            handler();
            previous_hook(info);
        }));
    }

    if !try_atexit() {
        hook_panic() // at least
    }
}

fn default_error_handler(from: impl AsRef<str>, error: Error) {
    if let Error::Multiple(errs) = error {
        errs.into_iter()
            .for_each(|err| default_error_handler(from.as_ref(), err));
        return;
    }

    let date = chrono::Local::now()
        .format("%Y-%m-%d %H:%M:%S.%3f")
        .to_string();

    // https://github.com/SpriteOvO/spdlog-rs/discussions/87
    //
    // Don't use `eprintln!` here, as it may fail to write and then panic.
    let _ = writeln!(
        io::stderr(),
        "[*** SPDLOG-RS UNHANDLED ERROR ***] [{}] [{}] {}",
        date,
        from.as_ref(),
        error
    );
}

// Used at log macros
#[doc(hidden)]
pub fn __log(
    logger: &Logger,
    level: Level,
    srcloc: Option<SourceLocation>,
    kvs: &[(kv::Key, kv::Value)],
    fmt_args: fmt::Arguments,
) {
    // Use `Cow` to avoid allocation as much as we can
    let payload: Cow<str> = fmt_args
        .as_str()
        .map(Cow::Borrowed) // No format arguments, so it is a `&'static str`
        .unwrap_or_else(|| Cow::Owned(fmt_args.to_string()));
    let record = Record::new(level, payload, srcloc, logger.name(), kvs);
    logger.log(&record);
}

#[cfg(test)]
mod tests {
    use test_utils::*;

    use super::*;

    #[test]
    fn test_default_logger() {
        let test_sink = Arc::new(TestSink::new());

        let test_logger = Arc::new(build_test_logger(|b| b.sink(test_sink.clone())));
        let empty_logger = Arc::new(Logger::builder().build().unwrap());

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

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
//! spdlog-rs = "0.3"
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
//! [`Logger`] and [`sink`] are the most important components of `spdlog-rs`.
//! Make sure to read their documentation. In short, a logger contains a
//! combination of sinks, and sinks implement writing log messages to actual
//! targets.
//!
//! ## Examples
//!
//! ```
//! use spdlog::prelude::*;
//!
//! info!("hello world!");
//! warn!("3 + 2 = {}", 5);
//! error!("oops!");
//! ```
//!
//! Output:
//!
//! <pre>
//! [2022-11-02 09:23:12.263] [<font color="#11D116">info</font>] hello, world!
//! [2022-11-02 09:23:12.263] [<font color="#FDBC4B">warn</font>] 3 + 2 = 5
//! [2022-11-02 09:23:12.263] [<font color="#C0392B">error</font>] oops!
//! </pre>
//!
//! If you want to learn more advanced features such as *asynchronous sink*,
//! *compile-time pattern formatter*, etc., please see [./examples]
//! directory.
//!
//! ## Help
//!
//! If you have any questions or need help while using this crate, feel free to
//! [open a discussion]. For feature requests or bug reports, please [open an
//! issue].
//!
//! # Overview of features
//!
//! - [Compatible with log crate](#compatible-with-log-crate)
//! - [Asynchronous support](#asynchronous-support)
//! - [Configured via environment
//!   variable](#configured-via-environment-variable)
//! - [Compile-time and runtime pattern
//!   formatter](#compile-time-and-runtime-pattern-formatter)
//! - [Compile-time filters](#compile-time-filters)
//!
//! # Compatible with log crate
//!
//! This is optional and is controlled by crate feature `log`.
//!
//! The compatibility with [log crate] is mainly through a proxy layer
//! [`LogCrateProxy`]. Call [`init_log_crate_proxy`] function to enable the
//! proxy layer, and all logs from [log crate] will be handled by it. You can
//! use it to output [log crate] logs of upstream dependencies or to quickly
//! migrate from [log crate] for your projects.
//!
//! [`LogCrateProxy`] forwards all logs from [log crate] to [`default_logger`]
//! by default, you can call [`log_crate_proxy()`] to get a reference to this
//! proxy to configure it.
//!
//! See [./examples] directory for examples.
//!
//! # Asynchronous support
//!
//! See [Asynchronous combined sink].
//!
//! # Configured via environment variable
//!
//! Users can optionally configure the level filter of loggers via the
//! environment variable `SPDLOG_RS_LEVEL`.
//!
//! For more details, see the documentation of [`init_env_level`].
//!
//! # Compile-time and runtime pattern formatter
//!
//! spdlog-rs supports formatting your log records according to a pattern
//! string. There are 2 ways to construct a pattern:
//!
//! - Macro [`pattern!`]: Builds a pattern at compile-time.
//! - Macro [`runtime_pattern!`]: Builds a pattern at runtime.
//!
//! ```
//! use spdlog::formatter::{pattern, PatternFormatter};
//! #[cfg(feature = "runtime-pattern")]
//! use spdlog::formatter::runtime_pattern;
//! # use spdlog::sink::{Sink, WriteSink};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // This pattern is built at compile-time, the template accepts only a literal string.
//! let pattern = pattern!("[{date} {time}.{millisecond}] [{level}] {payload}{eol}");
//!
//! #[cfg(feature = "runtime-pattern")]
//! {
//!     // This pattern is built at runtime, the template accepts a runtime string.
//!     let input = "[{date} {time}.{millisecond}] [{level}] {payload}{eol}";
//!     let pattern = runtime_pattern!(input)?;
//! }
//!
//! // Use the compile-time or runtime pattern.
//! # let your_sink = WriteSink::builder().target(vec![]).build()?;
//! your_sink.set_formatter(Box::new(PatternFormatter::new(pattern)));
//! # Ok(()) }
//! ```
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
//! # Crate Feature Flags
//!
//! The following crate feature flags are available in addition to the filters.
//! They are configured in your `Cargo.toml`.
//!
//!  - `source-location` allows recording the source location of each log. When
//!    it is enabled the default formatter [`FullFormatter`] will always format
//!    the source location information, and some formatting patterns related to
//!    source location will be available. If you do not want the source location
//!    information to appear in your binary file, you may prefer not to enable
//!    it.
//!
//!  - `flexible-string` improves the performance of formatting records, however
//!    contains unsafe code. For more details, see the documentation of
//!    [`StringBuf`].
//!
//!  - `log` see [Compatible with log crate](#compatible-with-log-crate) above.
//!
//!  - `native` enables platform-specific components, such as
//!    [`sink::WinDebugSink`], [`sink::JournaldSink`], etc. Note If the
//!    component requires additional system dependencies, then more granular
//!    features need to be enabled as well. See the documentation of the
//!    component for these details.
//!
//!  - `runtime-pattern` enables the ability to build patterns with runtime
//!    template string. See [`RuntimePattern`] for more details.
//!
//! # Supported Rust Versions
//!
//! <!--
//! When updating this, also update:
//! - .github/workflows/ci.yml
//! - Cargo.toml
//! - README.md
//! -->
//!
//! The current minimum supported Rust version is 1.56.
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
//! [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/spdlog/examples
//! [open a discussion]: https://github.com/SpriteOvO/spdlog-rs/discussions/new
//! [open an issue]: https://github.com/SpriteOvO/spdlog-rs/issues/new/choose
//! [log crate]: https://crates.io/crates/log
//! [Asynchronous combined sink]: sink/index.html#asynchronous-combined-sink
//! [`pattern!`]: crate::formatter::pattern
//! [`runtime_pattern!`]: crate::formatter::runtime_pattern
//! [`RuntimePattern`]: crate::formatter::RuntimePattern
//! [`FullFormatter`]: crate::formatter::FullFormatter
//! [`RotatingFileSink`]: crate::sink::RotatingFileSink
//! [`Formatter`]: crate::formatter::Formatter
//! [`RotationPolicy::Daily`]: crate::sink::RotationPolicy::Daily
//! [`RotationPolicy::Hourly`]: crate::sink::RotationPolicy::Hourly
//! [`AsyncPoolSink`]: crate::sink::AsyncPoolSink

// Credits: https://blog.wnut.pw/2020/03/24/documentation-and-unstable-rustdoc-features/
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(missing_docs)]

mod env_level;
pub mod error;
pub mod formatter;
mod level;
#[cfg(feature = "log")]
mod log_crate_proxy;
mod log_macros;
mod logger;
mod periodic_worker;
mod record;
pub mod sink;
mod source_location;
#[doc(hidden)]
pub mod string_buf;
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
    env::{self, VarError},
    ffi::OsStr,
    panic,
    result::Result as StdResult,
};

use cfg_if::cfg_if;
use error::EnvLevelError;
use sink::{Sink, StdStream, StdStreamSink};
use sync::*;

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
            .std_stream(StdStream::Stdout)
            .level_filter(LevelFilter::MoreVerbose(Level::Warn))
            .build()
            .unwrap();

        let stderr = StdStreamSink::builder()
            .std_stream(StdStream::Stderr)
            .level_filter(LevelFilter::MoreSevereEqual(Level::Warn))
            .build()
            .unwrap();

        let sinks: [Arc<dyn Sink>; 2] = [Arc::new(stdout), Arc::new(stderr)];

        let res = ArcSwap::from_pointee(Logger::builder().sinks(sinks).build_default().unwrap());

        flush_default_logger_at_exit();
        res
    })
}

/// Returns an [`Arc`] default logger.
///
/// This default logger will be used by logging macros, if the `logger`
/// parameter is not specified when logging macros are called.
///
/// If the default logger has not been replaced, the default:
///
///  - Contains a sink [`StdStreamSink`], writing logs on [`Level::Info`] and
///    more verbose levels to `stdout`.
///
///  - Contains a sink [`StdStreamSink`], writing logs on [`Level::Warn`] level
///    and more severe levels to `stderr`.
///
///  - Level filter ignores logs on [`Level::Debug`] and more verbose levels.
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
#[must_use]
pub fn default_logger() -> Arc<Logger> {
    default_logger_ref().load().clone()
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
    default_logger_ref().swap(logger)
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

/// Initialize environment variable level filters from environment variable
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
/// `gui=critical`, etc.
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
/// the rest of loggers as `LevelFilter::MoreSevereEqual(Level::Error)`.
///
/// ---
///
/// - `gui=warn,network=trace`
///
///   Specifies the level filter for loggers with name "gui" as
/// `LevelFilter::MoreSevereEqual(Level::Warn)`, loggers with name "network" as
/// `LevelFilter::MoreSevereEqual(Level::Trace)`.
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

/// Initialize environment variable level filters from a specified environment
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

/// Initialize log crate proxy.
///
/// This function calls [`log::set_logger`] to set up a [`LogCrateProxy`] and
/// all logs from log crate will be forwarded to `spdlog-rs`'s logger.
///
/// Users should call this function only once. Get the proxy to configure by
/// calling [`log_crate_proxy()`].
///
/// Note that the `log` crate uses a different log level filter and by default
/// it rejects all log messages. To log messages via the `log` crate, you have
/// to call [`log::set_max_level`] manually before logging. For more
/// information, please read the documentation of [`log::set_max_level`].
///
/// For more details, please read documentation of [`log::set_logger`] and
/// [`LogCrateProxy`].
#[cfg(feature = "log")]
pub fn init_log_crate_proxy() -> StdResult<(), log_crate::SetLoggerError> {
    log::set_logger(log_crate_proxy())
}

/// Returns a [`LogCrateProxy`].
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

    eprintln!(
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
    fmt_args: std::fmt::Arguments,
) {
    // use `Cow` to avoid allocation as much as we can
    let payload: std::borrow::Cow<str> = match fmt_args.as_str() {
        Some(literal_str) => literal_str.into(), // no format arguments, so it is a `&'static str`
        None => fmt_args.to_string().into(),
    };

    let mut builder = Record::builder(level, payload).source_location(srcloc);
    if let Some(logger_name) = logger.name() {
        builder = builder.logger_name(logger_name);
    }
    logger.log(&builder.build());
}

#[cfg(test)]
mod tests {
    use test_utils::*;

    use super::*;

    #[test]
    fn test_default_logger() {
        let test_sink = Arc::new(CounterSink::new());

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

//! Provides formatters for sink formatting log records.
//!
//! # Formatter
//!
//! Each normal *Sink* owns a *Formatter*, which is used to format each log.
//!
//! The default formatter for most sinks is [`FullFormatter`], you can call
//! [`Sink::set_formatter`] to replace it with another formatter.
//!
//! The easiest way to make a custom formatter is to build a pattern, see
//! [Compile-time and runtime pattern
//! formatter](#compile-time-and-runtime-pattern-formatter) below. If pattern
//! isn't flexible enough for you, you need to implement [`Formatter`] trait for
//! your own formatter struct. See the implementation of [`FullFormatter`] and
//! [./examples] directory for examples.
//!
//! # Compile-time and runtime pattern formatter
//!
//! *spdlog-rs* supports formatting your log records according to a pattern
//! string. There are 2 ways to construct a pattern:
//!
//! - Macro [`pattern!`]: Builds a pattern at compile-time.
//! - Macro [`runtime_pattern!`]: Builds a pattern at runtime.
//!
//! ```
//! use spdlog::formatter::{pattern, PatternFormatter};
//! # use spdlog::sink::{Sink, WriteSink};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // This pattern is built at compile-time, the template accepts only a literal string.
//! let pattern = pattern!("[{date} {time}.{millisecond}] [{level}] {payload}{eol}");
//!
//! #[cfg(feature = "runtime-pattern")]
//! {
//!     use spdlog::formatter::runtime_pattern;
//!
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
//! [`Sink::set_formatter`]: crate::sink::Sink::set_formatter
//! [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/spdlog/examples

mod full_formatter;
#[cfg(any(
    all(target_os = "linux", feature = "native", feature = "libsystemd"),
    all(doc, not(doctest))
))]
mod journald_formatter;
#[cfg(feature = "json-formatter")]
mod json_formatter;
mod local_time_cacher;
mod pattern_formatter;

use std::ops::Range;

use dyn_clone::*;
pub use full_formatter::*;
#[cfg(any(
    all(target_os = "linux", feature = "native", feature = "libsystemd"),
    all(doc, not(doctest))
))]
pub(crate) use journald_formatter::*;
#[cfg(feature = "json-formatter")]
pub use json_formatter::*;
pub(crate) use local_time_cacher::*;
pub use pattern_formatter::*;

use crate::{Record, Result, StringBuf};

/// Represents a formatter that can be used for formatting logs.
///
/// # Examples
///
/// See the implementation of [`FullFormatter`] and [./examples] directory.
///
/// [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/spdlog/examples
pub trait Formatter: Send + Sync + DynClone {
    /// Formats a log record.
    fn format(&self, record: &Record, dest: &mut StringBuf) -> Result<FmtExtraInfo>;
}
clone_trait_object!(Formatter);

/// Extra information for formatted text.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct FmtExtraInfo {
    style_range: Option<Range<usize>>,
}

impl FmtExtraInfo {
    /// Constructs a `FmtExtraInfo`.
    #[must_use]
    pub fn new() -> FmtExtraInfo {
        FmtExtraInfo::default()
    }

    /// Gets a [`FmtExtraInfoBuilder`].
    #[must_use]
    pub fn builder() -> FmtExtraInfoBuilder {
        FmtExtraInfoBuilder::new()
    }

    /// A style range (in bytes) of the formatted text.
    ///
    /// If style is available in the sink, the text in the range will be
    /// rendered in the style corresponding to that log message level, otherwise
    /// it will be ignored.
    ///
    /// Its indexes are guaranteed by the setter to be the correct UTF-8
    /// boundary.
    #[must_use]
    pub fn style_range(&self) -> Option<Range<usize>> {
        self.style_range.clone() // This clone is cheap
    }
}

#[allow(missing_docs)]
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct FmtExtraInfoBuilder {
    info: FmtExtraInfo,
}

impl FmtExtraInfoBuilder {
    /// Constructs a `FmtExtraInfoBuilder`.
    ///
    /// The default value of [`FmtExtraInfo`] is the same as
    /// [`FmtExtraInfo::new`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets style range (in bytes) of the formatted text.
    ///
    /// Users must ensure that indexes are correctly UTF-8 boundary.
    #[must_use]
    pub fn style_range(mut self, range: Range<usize>) -> Self {
        self.info.style_range = Some(range);
        self
    }

    /// Builds a [`FmtExtraInfo`].
    #[must_use]
    pub fn build(self) -> FmtExtraInfo {
        self.info
    }
}

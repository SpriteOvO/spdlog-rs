//! Provides formatters for sink formatting log records.
//!
//! Usually use [`Sink::set_formatter`] to set the formatter of a sink.
//!
//! [`Sink::set_formatter`]: crate::sink::Sink::set_formatter

mod full_formatter;
mod pattern_formatter;
mod utils;

pub use full_formatter::*;
pub use pattern_formatter::*;

use std::ops::Range;

use crate::{Record, Result, StringBuf};

/// A trait for log records formatters.
///
/// # Examples
///
/// See the implementation of [`FullFormatter`] and [./examples] directory.
///
/// [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/spdlog/examples
pub trait Formatter: Send + Sync {
    /// Format a log record.
    fn format(&self, record: &Record, dest: &mut StringBuf) -> Result<FmtExtraInfo>;
}

/// Extra information for formatted text.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct FmtExtraInfo {
    style_range: Option<Range<usize>>,
}

impl FmtExtraInfo {
    /// Constructs a `FmtExtraInfo`.
    pub fn new() -> FmtExtraInfo {
        FmtExtraInfo::default()
    }

    /// Constructs a [`FmtExtraInfoBuilder`].
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
    pub fn style_range(&self) -> Option<Range<usize>> {
        self.style_range.clone() // This clone is cheap
    }
}

/// The builder of [`FmtExtraInfo`].
///
/// # Examples
///
/// See the implementation of [`FullFormatter`] and [./examples] directory.
///
/// [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/spdlog/examples
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct FmtExtraInfoBuilder {
    info: FmtExtraInfo,
}

impl FmtExtraInfoBuilder {
    /// Constructs a `FmtExtraInfoBuilder`.
    ///
    /// The default value of [`FmtExtraInfo`] is the same as
    /// [`FmtExtraInfo::new`].
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
    pub fn build(self) -> FmtExtraInfo {
        self.info
    }
}

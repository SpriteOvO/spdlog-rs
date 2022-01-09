//! Provides formatters for sink formatting log messages.

pub mod full_formatter;

pub use full_formatter::*;

use std::ops::Range;

use crate::{Record, Result, StringBuf};

/// A trait for log message formatters.
///
/// Used at [`Sink::set_formatter`].
///
/// [`Sink::set_formatter`]: crate::sink::Sink::set_formatter
pub trait Formatter: Send + Sync {
    /// Format a log message
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

    /// A style range of the formatted text.
    ///
    /// If style is available in the sink, the text in the range will be
    /// rendered in the style corresponding to that log message level, otherwise
    /// it will be ignored.
    ///
    /// It must be guaranteed to be the correct byte offset for the utf-8
    /// characters.
    pub fn style_range(&self) -> Option<Range<usize>> {
        self.style_range.clone() // This clone is cheap
    }
}

/// The builder of [`Logger`].
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct FmtExtraInfoBuilder {
    info: FmtExtraInfo,
}

impl FmtExtraInfoBuilder {
    /// Constructs a `FmtExtraInfoBuilder`.
    ///
    /// The default value is the same as [`FmtExtraInfo::default()`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets style range of the formatted text.
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

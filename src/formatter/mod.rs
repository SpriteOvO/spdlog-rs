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

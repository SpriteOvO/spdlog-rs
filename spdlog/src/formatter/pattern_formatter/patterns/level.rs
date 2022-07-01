use std::fmt::Write;

use crate::{
    formatter::pattern_formatter::{Pattern, PatternContext},
    Error, Record, StringBuf,
};

/// A pattern that writes the level of a log record into the output. Examples:
/// `critical`, `error`, `warn`.
///
/// This pattern corresponds to `{l}` or `{level}` in the pattern template
/// string.
#[derive(Copy, Clone, Debug, Default)]
pub struct Level;

impl Level {
    /// Create a new `Level` pattern.
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for Level {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(record.level().as_str())
            .map_err(Error::FormatRecord)?;
        Ok(())
    }
}

/// A pattern that writes the level in a shorter form of a log record into the
/// output. Examples: `C`, `E`, `W`.
///
/// This pattern corresponds to `{L}` or `{level-short}` in the pattern template
/// string.
#[derive(Copy, Clone, Debug, Default)]
pub struct ShortLevel;

impl ShortLevel {
    /// Create a new `ShortLevel` pattern.
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for ShortLevel {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(record.level().as_short_str())
            .map_err(Error::FormatRecord)
    }
}

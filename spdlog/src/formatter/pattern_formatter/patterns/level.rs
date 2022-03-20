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
#[derive(Copy, Clone, Debug)]
pub struct Level;

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
/// This pattern corresponds to `{L}` or `{short-level}` in the pattern template
/// string.
pub struct ShortLevel;

impl Pattern for ShortLevel {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        const SHORT_LEVEL_NAMES: &[&str] = &["C", "E", "W", "I", "D", "T"];
        dest.write_str(SHORT_LEVEL_NAMES[record.level() as u16 as usize])
            .map_err(Error::FormatRecord)?;
        Ok(())
    }
}

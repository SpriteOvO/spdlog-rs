use std::fmt::Write;

use crate::{
    formatter::pattern_formatter::{Pattern, PatternContext},
    Error, Record, StringBuf,
};

/// A pattern that writes the logger's name into the output. Example:
/// `my-logger`.
#[derive(Copy, Clone, Debug, Default)]
pub struct LoggerName;

impl LoggerName {
    /// Create a new `LoggerName` pattern.
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for LoggerName {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        if let Some(logger_name) = record.logger_name() {
            dest.write_str(logger_name).map_err(Error::FormatRecord)?;
        }
        Ok(())
    }
}

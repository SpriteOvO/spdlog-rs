use std::fmt::Write;

use crate::{
    formatter::pattern_formatter::{Pattern, PatternContext},
    Error, Record, StringBuf,
};

/// A pattern that writes the logger's name into the output. Example: `my-logger`.
///
/// This pattern corresponds to `{n}` or `{logger}` in the pattern template string.
#[derive(Copy, Clone, Debug)]
pub struct LoggerName;

impl Pattern for LoggerName {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(record.logger_name().unwrap_or(""))
            .map_err(|err| Error::FormatRecord(err))?;
        Ok(())
    }
}

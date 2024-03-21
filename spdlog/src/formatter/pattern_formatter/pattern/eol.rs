use std::fmt::Write;

use crate::{
    formatter::pattern_formatter::{Pattern, PatternContext},
    Error, Record, StringBuf,
};

/// A pattern that writes an EOL character into the output.
///
/// # Implementation
///
/// On non-Windows systems, this pattern writes a `\n` to the output.
///
/// On Windows, this pattern writes a `\r\n` to the output.
#[derive(Clone, Default)]
pub struct Eol;

impl Pattern for Eol {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(crate::__EOL).map_err(Error::FormatRecord)
    }
}

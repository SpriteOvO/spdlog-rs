use std::fmt::Write;

use crate::{
    formatter::pattern_formatter::{Pattern, PatternContext},
    Error, Record, StringBuf,
};

/// A pattern that writes the payload of a log record into output. Example: `log
/// message`.
#[derive(Clone, Default)]
pub struct Payload;

impl Pattern for Payload {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(record.payload())
            .map_err(Error::FormatRecord)
    }
}

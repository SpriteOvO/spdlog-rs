use crate::{
    formatter::pattern_formatter::{Pattern, PatternContext},
    Error, Record, StringBuf,
};

#[derive(Clone, Default)]
pub struct KV;

impl Pattern for KV {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        record
            .key_values()
            .write_to(dest, false)
            .map_err(Error::FormatRecord)
    }
}

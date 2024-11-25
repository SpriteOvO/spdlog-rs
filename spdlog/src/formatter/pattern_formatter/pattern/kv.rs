use std::fmt::{self, Write};

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
        (|| -> Result<(), fmt::Error> {
            let kvs = record.key_values();
            if !kvs.is_empty() {
                dest.write_str("{ ")?;
                let mut iter = kvs.peekable();
                while let Some((key, value)) = iter.next() {
                    dest.write_str(key.as_str())?;
                    dest.write_str("=")?;
                    write!(dest, "{}", value)?;
                    if iter.peek().is_some() {
                        dest.write_str(", ")?;
                    }
                }
                dest.write_str(" }")?;
            }
            Ok(())
        })()
        .map_err(Error::FormatRecord)
    }
}

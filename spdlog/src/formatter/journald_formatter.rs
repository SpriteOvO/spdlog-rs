// TODO: Remove this file, use `PatternFormatter` instead
//
// Need to keep waiting for conditional space and brackets to be supported in
// pattern template strings (optional fields require these, e.g. `logger_name`)

use std::fmt::{self, Write};

use cfg_if::cfg_if;

use crate::{
    formatter::{Formatter, FormatterContext},
    Error, Record, StringBuf, __EOL,
};

#[derive(Clone)]
pub(crate) struct JournaldFormatter {}

impl JournaldFormatter {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn format_impl(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut FormatterContext,
    ) -> Result<(), fmt::Error> {
        cfg_if! {
            if #[cfg(not(feature = "flexible-string"))] {
                dest.reserve(crate::string_buf::RESERVE_SIZE);
            }
        }

        dest.write_str("[")?;

        if let Some(logger_name) = record.logger_name() {
            dest.write_str(logger_name)?;
            dest.write_str("] [")?;
        }

        let style_range_begin = dest.len();

        dest.write_str(record.level().as_str())?;

        let style_range_end = dest.len();

        dest.write_str("] ")?;
        dest.write_str(record.payload())?;

        let kvs = record.key_values();
        if !kvs.is_empty() {
            dest.write_str(" { ")?;

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

        dest.write_str(__EOL)?;

        ctx.set_style_range(Some(style_range_begin..style_range_end));
        Ok(())
    }
}

impl Formatter for JournaldFormatter {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut FormatterContext,
    ) -> crate::Result<()> {
        self.format_impl(record, dest, ctx)
            .map_err(Error::FormatRecord)
    }
}

impl Default for JournaldFormatter {
    fn default() -> Self {
        Self::new()
    }
}

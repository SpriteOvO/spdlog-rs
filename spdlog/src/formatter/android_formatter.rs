// TODO: Remove this file, use `PatternFormatter` instead
//
// Need to keep waiting for conditional space and brackets to be supported in
// pattern template strings (optional fields require these, e.g. `logger_name`)

use std::fmt::{self, Write};

use crate::{
    formatter::{Formatter, FormatterContext},
    Error, Record, StringBuf,
};

#[derive(Clone)]
pub(crate) struct AndroidFormatter {}

impl AndroidFormatter {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn format_impl(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut FormatterContext,
    ) -> Result<(), fmt::Error> {
        #[cfg(not(feature = "flexible-string"))]
        dest.reserve(crate::string_buf::RESERVE_SIZE);

        if let Some(logger_name) = record.logger_name() {
            dest.write_str("[")?;
            dest.write_str(logger_name)?;
            dest.write_str("] ")?;
        }

        if let Some(srcloc) = record.source_location() {
            dest.write_str("[")?;
            dest.write_str(srcloc.module_path())?;
            dest.write_str(", ")?;
            dest.write_str(srcloc.file())?;
            dest.write_str(":")?;
            dest.write_str(&numtoa::BaseN::<10>::u32(srcloc.line()))?;
            dest.write_str("] ")?;
        }

        dest.write_str(record.payload())?;

        record.key_values().write_to(dest, true)?;
        Ok(())
    }
}

impl Formatter for AndroidFormatter {
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

impl Default for AndroidFormatter {
    fn default() -> Self {
        Self::new()
    }
}

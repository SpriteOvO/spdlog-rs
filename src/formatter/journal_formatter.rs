// TODO: Remove this file, use `PatternFormatter` instead

use std::fmt::{self, Write};

use crate::{
    formatter::{FmtExtraInfo, Formatter},
    Error, Record, StringBuf, EOL,
};

#[derive(Clone)]
pub(crate) struct JournalFormatter {}

impl JournalFormatter {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn format_impl(
        &self,
        record: &Record,
        dest: &mut StringBuf,
    ) -> Result<FmtExtraInfo, fmt::Error> {
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
        dest.write_str(EOL)?;

        Ok(FmtExtraInfo {
            style_range: Some(style_range_begin..style_range_end),
        })
    }
}

impl Formatter for JournalFormatter {
    fn format(&self, record: &Record, dest: &mut StringBuf) -> crate::Result<FmtExtraInfo> {
        self.format_impl(record, dest).map_err(Error::FormatRecord)
    }

    fn clone_box(&self) -> Box<dyn Formatter> {
        Box::new(self.clone())
    }
}

impl Default for JournalFormatter {
    fn default() -> Self {
        Self::new()
    }
}

//! Provides a full info formatter.

use std::{
    fmt::{self, Write},
    marker::PhantomData,
};

use cfg_if::cfg_if;

use crate::{
    formatter::{FmtExtraInfo, Formatter, LOCAL_TIME_CACHER},
    Error, Record, StringBuf, EOL,
};

/// A full info log records formatter.
///
/// It is the default formatter for sinks.
///
/// Log messages formatted by it look like:
///
///  - Default:
///
///    `[2021-12-23 01:23:45.067] [info] log message`
///
///  - If the logger has a name:
///
///    `[2021-12-23 01:23:45.067] [logger-name] [info] log message`
///
///  - If crate feature `source-location` is enabled:
///
///    `[2021-12-23 01:23:45.067] [info] [mod::path, src/main.rs:2] log message`
#[derive(Clone)]
pub struct FullFormatter {
    _phantom: PhantomData<()>,
    with_eol: bool,
}

impl FullFormatter {
    /// Constructs a `FullFormatter`.
    pub fn new() -> FullFormatter {
        FullFormatter {
            _phantom: PhantomData,
            with_eol: true,
        }
    }

    pub(crate) fn without_eol() -> Self {
        Self {
            _phantom: PhantomData,
            with_eol: false,
        }
    }

    fn format_impl(
        &self,
        record: &Record,
        dest: &mut StringBuf,
    ) -> Result<FmtExtraInfo, fmt::Error> {
        cfg_if! {
            if #[cfg(not(feature = "flexible-string"))] {
                dest.reserve(crate::string_buf::RESERVE_SIZE);
            }
        }

        {
            let mut local_time_cacher = LOCAL_TIME_CACHER.lock();
            let time = local_time_cacher.get(record.time());
            dest.write_str("[")?;
            dest.write_str(&time.full_second_str())?;
            dest.write_str(".")?;
            write!(dest, "{:03}", time.millisecond())?;
            dest.write_str("] [")?;
        }

        if let Some(logger_name) = record.logger_name() {
            dest.write_str(logger_name)?;
            dest.write_str("] [")?;
        }

        let style_range_begin = dest.len();

        dest.write_str(record.level().as_str())?;

        let style_range_end = dest.len();

        if let Some(srcloc) = record.source_location() {
            dest.write_str("] [")?;
            dest.write_str(srcloc.module_path())?;
            dest.write_str(", ")?;
            dest.write_str(srcloc.file())?;
            dest.write_str(":")?;
            write!(dest, "{}", srcloc.line())?;
        }

        dest.write_str("] ")?;
        dest.write_str(record.payload())?;

        if self.with_eol {
            dest.write_str(EOL)?;
        }

        Ok(FmtExtraInfo {
            style_range: Some(style_range_begin..style_range_end),
        })
    }
}

impl Formatter for FullFormatter {
    fn format(&self, record: &Record, dest: &mut StringBuf) -> crate::Result<FmtExtraInfo> {
        self.format_impl(record, dest).map_err(Error::FormatRecord)
    }

    fn clone_box(&self) -> Box<dyn Formatter> {
        Box::new(self.clone())
    }
}

impl Default for FullFormatter {
    fn default() -> FullFormatter {
        FullFormatter::new()
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;

    use super::*;
    use crate::{Level, EOL};

    #[test]
    fn format() {
        let record = Record::new(Level::Warn, "test log content");
        let mut buf = StringBuf::new();
        let extra_info = FullFormatter::new().format(&record, &mut buf).unwrap();

        let local_time: DateTime<Local> = record.time().into();
        assert_eq!(
            format!(
                "[{}] [warn] test log content{}",
                local_time.format("%Y-%m-%d %H:%M:%S.%3f"),
                EOL
            ),
            buf
        );
        assert_eq!(Some(27..31), extra_info.style_range());
    }
}

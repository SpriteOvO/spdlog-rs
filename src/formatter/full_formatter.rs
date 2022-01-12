//! Provides a full info formatter.

use std::{
    fmt::{self, Write},
    time::SystemTime,
};

use chrono::prelude::*;

use crate::{
    formatter::{FmtExtraInfo, Formatter},
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
///    `[2021-12-23 01:23:45.067] [info] [main.rs:2] log message`
pub struct FullFormatter {
    local_time_cacher: spin::Mutex<LocalTimeCacher>,
}

impl FullFormatter {
    /// Constructs a `FullFormatter`.
    pub fn new() -> FullFormatter {
        FullFormatter {
            local_time_cacher: spin::Mutex::new(LocalTimeCacher::new()),
        }
    }

    fn format_impl(
        &self,
        record: &Record,
        dest: &mut StringBuf,
    ) -> Result<FmtExtraInfo, fmt::Error> {
        {
            let mut local_time_cacher = self.local_time_cacher.lock();
            let time = local_time_cacher.get(record.time());
            dest.write_str("[")?;
            dest.write_str(time.0)?;
            dest.write_str(".")?;
            write!(dest, "{:03}", time.1)?;
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
            dest.write_str(srcloc.file_name())?;
            dest.write_str(":")?;
            write!(dest, "{}", srcloc.line())?;
        }

        dest.write_str("] ")?;
        dest.write_str(record.payload())?;
        dest.write_str(EOL)?;

        Ok(FmtExtraInfo {
            style_range: Some(style_range_begin..style_range_end),
        })
    }
}

impl Formatter for FullFormatter {
    fn format(&self, record: &Record, dest: &mut StringBuf) -> crate::Result<FmtExtraInfo> {
        self.format_impl(record, dest).map_err(Error::FormatRecord)
    }
}

impl Default for FullFormatter {
    fn default() -> FullFormatter {
        FullFormatter::new()
    }
}

#[derive(Clone, Default)]
struct LocalTimeCacher {
    last_secs: i64,
    local_time_str: Option<String>,
}

impl LocalTimeCacher {
    fn new() -> LocalTimeCacher {
        LocalTimeCacher::default()
    }

    // Returns (local_time_in_sec, millisecond)
    fn get(&mut self, system_time: SystemTime) -> (&str, u32) {
        let utc_time: DateTime<Utc> = system_time.into();
        let millisecond = utc_time.nanosecond() % 1_000_000_000 / 1_000_000;
        (self.update(utc_time), millisecond)
    }

    fn update(&mut self, utc_time: DateTime<Utc>) -> &str {
        let secs = utc_time.timestamp();

        if self.local_time_str.is_none() || self.last_secs != secs {
            let local_time: DateTime<Local> = utc_time.into();
            self.local_time_str = Some(format!(
                // `local_time.format("%Y-%m-%d %H:%M:%S")` is slower than this way
                "{}-{:02}-{:02} {:02}:{:02}:{:02}",
                local_time.year(),
                local_time.month(),
                local_time.day(),
                local_time.hour(),
                local_time.minute(),
                local_time.second(),
            ));
            self.last_secs = secs;
        }

        self.local_time_str.as_ref().unwrap().as_str()
    }
}

#[cfg(test)]
mod tests {

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

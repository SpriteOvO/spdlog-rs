//! Provides a basic and default log message formatter.

use std::fmt::{self, Write};

use chrono::prelude::*;

use crate::{
    formatter::{FmtExtraInfo, Formatter},
    Error, Record, StringBuf,
};

/// A basic and default log message formatter.
///
/// The log message formatted by it looks like this:
/// `[2021-12-23 01:23:45.067] [info] log message`.
pub struct BasicFormatter {
    local_time_cacher: spin::Mutex<LocalTimeCacher>,
}

impl BasicFormatter {
    /// Constructs a [`BasicFormatter`].
    pub fn new() -> BasicFormatter {
        BasicFormatter {
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

        Ok(FmtExtraInfo {
            style_range: Some(style_range_begin..style_range_end),
        })
    }
}

impl Formatter for BasicFormatter {
    fn format(&self, record: &Record, dest: &mut StringBuf) -> crate::Result<FmtExtraInfo> {
        self.format_impl(record, dest).map_err(Error::FormatRecord)
    }
}

impl Default for BasicFormatter {
    fn default() -> BasicFormatter {
        BasicFormatter::new()
    }
}

#[derive(Clone, Default)]
struct LocalTimeCacher {
    cache: Option<LocalTimeCache>,
}

impl LocalTimeCacher {
    fn new() -> LocalTimeCacher {
        LocalTimeCacher::default()
    }

    // Returns (local_time_in_sec, millisecond)
    fn get(&mut self, utc_time: &DateTime<Utc>) -> (&str, u32) {
        match &mut self.cache {
            None => self.cache = Some(LocalTimeCache::new(utc_time)),
            Some(cache) => {
                let secs = utc_time.timestamp();
                if cache.last_secs != secs {
                    *cache = LocalTimeCache::new(utc_time);
                }
            }
        }

        let millisecond = utc_time.nanosecond() % 1_000_000_000 / 1_000_000;

        (
            self.cache.as_ref().unwrap().local_time_str.as_str(),
            millisecond,
        )
    }
}

#[derive(Clone)]
struct LocalTimeCache {
    last_secs: i64,
    local_time_str: String,
}

impl LocalTimeCache {
    fn new(utc_time: &DateTime<Utc>) -> Self {
        let time: DateTime<Local> = (*utc_time).into();
        Self {
            last_secs: time.timestamp(),
            local_time_str: format!(
                // `time.format("%Y-%m-%d %H:%M:%S")` is slower than this way
                "{}-{:02}-{:02} {:02}:{:02}:{:02}",
                time.year(),
                time.month(),
                time.day(),
                time.hour(),
                time.minute(),
                time.second(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::Level;

    #[test]
    fn format() {
        let record = Record::new(Level::Warn, "test log content");
        let mut buf = StringBuf::new();
        let extra_info = BasicFormatter::new().format(&record, &mut buf).unwrap();

        assert_eq!(
            format!(
                "[{}] [warn] test log content",
                Into::<DateTime::<Local>>::into(record.time().clone())
                    .format("%Y-%m-%d %H:%M:%S.%3f")
            ),
            buf
        );
        assert_eq!(Some(27..31), extra_info.style_range());
    }
}

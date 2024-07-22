use std::fmt::{self, Write};

use cfg_if::cfg_if;
use serde::{ser::SerializeStruct, Serialize};

use crate::{
    formatter::{FmtExtraInfo, Formatter, LOCAL_TIME_CACHER},
    Error, Record, StringBuf, __EOL,
};

struct JsonRecord<'a>(&'a Record<'a>);

impl<'a> Serialize for JsonRecord<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let src_loc = self.0.source_location();

        let mut record =
            serializer.serialize_struct("JsonRecord", if src_loc.is_none() { 4 } else { 5 })?;

        record.serialize_field("level", &self.0.level())?;
        {
            let mut local_time_cacher = LOCAL_TIME_CACHER.lock();
            record.serialize_field::<str>(
                "time",
                local_time_cacher
                    .get(self.0.time())
                    .full_second_str()
                    .as_ref(),
            )?;
        }
        record.serialize_field("tid", &self.0.tid())?;
        record.serialize_field("payload", self.0.payload())?;

        if let Some(src_loc) = src_loc {
            record.serialize_field("source_location", src_loc)?;
        }

        record.end()
    }
}

impl<'a> From<&'a Record<'a>> for JsonRecord<'a> {
    fn from(value: &'a Record<'a>) -> Self {
        JsonRecord(value)
    }
}

enum JsonFormatterError {
    Fmt(fmt::Error),
    Serialization(serde_json::Error),
}

impl From<fmt::Error> for JsonFormatterError {
    fn from(value: fmt::Error) -> Self {
        JsonFormatterError::Fmt(value)
    }
}

impl From<serde_json::Error> for JsonFormatterError {
    fn from(value: serde_json::Error) -> Self {
        JsonFormatterError::Serialization(value)
    }
}

impl From<JsonFormatterError> for crate::Error {
    fn from(value: JsonFormatterError) -> Self {
        match value {
            JsonFormatterError::Fmt(e) => Error::FormatRecord(e),
            JsonFormatterError::Serialization(e) => Error::Serialization(e.into()),
        }
    }
}

/// JSON logs formatter
///
/// Output format:
///
///   ```json
///   {"level":"info","time":"2024-01-01
/// 12:00:00","tid":123456,"payload":"test"}
///
///   // with source location
///   {"level":"info","time":"2024-01-01
/// 12:00:00","tid":123456,"payload":"test","source_location":{"module_path":"
/// module","file":"file.rs","line":42}}
///   ```
#[derive(Clone)]
pub struct JsonFormatter;

impl JsonFormatter {
    /// Create a `JsonFormatter`
    pub fn new() -> JsonFormatter {
        JsonFormatter
    }

    fn format_impl(
        &self,
        record: &Record,
        dest: &mut StringBuf,
    ) -> Result<FmtExtraInfo, JsonFormatterError> {
        cfg_if! {
            if #[cfg(not(feature = "flexible-string"))] {
                dest.reserve(crate::string_buf::RESERVE_SIZE);
            }
        }

        let json_record: JsonRecord = record.into();

        dest.write_str(&serde_json::to_string(&json_record)?)?;

        dest.write_str(__EOL)?;

        Ok(FmtExtraInfo { style_range: None })
    }
}

impl Formatter for JsonFormatter {
    fn format(&self, record: &Record, dest: &mut StringBuf) -> crate::Result<FmtExtraInfo> {
        self.format_impl(record, dest).map_err(Into::into)
    }
}

impl Default for JsonFormatter {
    fn default() -> Self {
        JsonFormatter::new()
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;

    use super::*;
    use crate::{Level, SourceLocation, __EOL};

    #[test]
    fn should_format_json() {
        let mut dest = StringBuf::new();
        let formatter = JsonFormatter::new();
        let record = Record::builder(Level::Info, "payload").build();
        let extra_info = formatter.format(&record, &mut dest).unwrap();

        let local_time: DateTime<Local> = record.time().into();

        assert_eq!(extra_info.style_range, None);
        assert_eq!(
            dest.to_string(),
            format!(
                r#"{{"level":"Info","time":"{}","tid":{},"payload":"{}"}}{}"#,
                local_time.format("%Y-%m-%d %H:%M:%S"),
                record.tid(),
                "payload",
                __EOL
            )
        );
    }

    #[test]
    fn should_format_json_with_src_loc() {
        let mut dest = StringBuf::new();
        let formatter = JsonFormatter::new();
        let record = Record::builder(Level::Info, "payload")
            .source_location(Some(SourceLocation::__new("module", "file.rs", 1, 2)))
            .build();
        let extra_info = formatter.format(&record, &mut dest).unwrap();

        let local_time: DateTime<Local> = record.time().into();

        assert_eq!(extra_info.style_range, None);
        assert_eq!(
            dest.to_string(),
            format!(
                r#"{{"level":"Info","time":"{}","tid":{},"payload":"{}","source_location":{{"module_path":"module","file":"file.rs","line":1,"column":2}}}}{}"#,
                local_time.format("%Y-%m-%d %H:%M:%S"),
                record.tid(),
                "payload",
                __EOL
            )
        );
    }
}

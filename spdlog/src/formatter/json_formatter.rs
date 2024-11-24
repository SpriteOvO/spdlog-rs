use std::{
    fmt::{self, Write},
    marker::PhantomData,
    time::SystemTime,
};

use cfg_if::cfg_if;
use serde::{
    ser::{SerializeMap, SerializeStruct},
    Serialize, Serializer,
};

use crate::{
    formatter::{Formatter, FormatterContext},
    Error, Record, StringBuf, __EOL,
};

fn opt_to_num<T>(opt: Option<T>) -> usize {
    opt.map_or(0, |_| 1)
}

struct JsonRecord<'a, 'b>(&'a Record<'b>);

impl Serialize for JsonRecord<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let fields_len =
            4 + opt_to_num(self.0.logger_name()) + opt_to_num(self.0.source_location());
        let mut record = serializer.serialize_struct("JsonRecord", fields_len)?;

        record.serialize_field("level", &self.0.level())?;
        record.serialize_field(
            "timestamp",
            &self
                .0
                .time()
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                // https://github.com/SpriteOvO/spdlog-rs/pull/69#discussion_r1694063293
                .and_then(|dur| u64::try_from(dur.as_millis()).ok())
                .expect("invalid timestamp"),
        )?;
        record.serialize_field("payload", self.0.payload())?;

        if !self.0.key_values().is_empty() {
            struct JsonKV<'a, 'b>(&'a Record<'b>);

            impl Serialize for JsonKV<'_, '_> {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    let kv = self.0.key_values();
                    let mut map = serializer.serialize_map(Some(kv.len()))?;
                    for (key, value) in kv {
                        map.serialize_entry(key.as_str(), &value)?;
                    }
                    map.end()
                }
            }

            record.serialize_field("kv", &JsonKV(self.0))?;
        }

        if let Some(logger_name) = self.0.logger_name() {
            record.serialize_field("logger", logger_name)?;
        }
        record.serialize_field("tid", &self.0.tid())?;
        if let Some(src_loc) = self.0.source_location() {
            record.serialize_field("source", src_loc)?;
        }

        record.end()
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
            JsonFormatterError::Serialization(e) => Error::SerializeRecord(e.into()),
        }
    }
}

#[rustfmt::skip]
/// JSON logs formatter.
/// 
/// Each log will be serialized into a single line of JSON object with the following schema.
/// 
/// ## Schema
/// 
/// | Field       | Type         | Description                                                                                                                    |
/// |-------------|--------------|--------------------------------------------------------------------------------------------------------------------------------|
/// | `level`     | String       | The level of the log. Same as the return of [`Level::as_str`].                                                                 |
/// | `timestamp` | Integer(u64) | The timestamp when the log was generated, in milliseconds since January 1, 1970 00:00:00 UTC.                                  |
/// | `payload`   | String       | The contents of the log.                                                                                                       |
/// | `kv`        | Object/Null  | The key-values of the log. Null if kv is not specified.                                                                        |
/// | `logger`    | String/Null  | The name of the logger. Null if the logger has no name.                                                                        |
/// | `tid`       | Integer(u64) | The thread ID when the log was generated.                                                                                      |
/// | `source`    | Object/Null  | The source location of the log. See [`SourceLocation`] for its schema. Null if crate feature `source-location` is not enabled. |
/// 
/// <div class="warning">
/// 
/// - If the type of a field is Null, the field will not be present or be `null`.
/// 
/// - The order of the fields is not guaranteed.
/// 
/// </div>
/// 
/// ---
/// 
/// ## Examples
///
///  - Default:
/// 
///    ```json
///    {"level":"info","timestamp":1722817424798,"payload":"hello, world!","tid":3472525}
///    {"level":"error","timestamp":1722817424798,"payload":"something went wrong","tid":3472525}
///    ```
/// 
///  - If the logger has a name:
/// 
///    ```json
///    {"level":"info","timestamp":1722817541459,"payload":"hello, world!","logger":"app-component","tid":3478045}
///    {"level":"error","timestamp":1722817541459,"payload":"something went wrong","logger":"app-component","tid":3478045}
///    ```
/// 
///  - If key-values are present:
/// 
///    ```json
///    {"level":"info","timestamp":1722817541459,"payload":"hello, world!","kv":{"k1":123,"k2":"cool"},"tid":3478045}
///    {"level":"error","timestamp":1722817541459,"payload":"something went wrong","kv":{"k1":123,"k2":"cool"},"tid":3478045}
///    ```
/// 
///  - If crate feature `source-location` is enabled:
/// 
///    ```json
///    {"level":"info","timestamp":1722817572709,"payload":"hello, world!","tid":3479856,"source":{"module_path":"my_app::say_hi","file":"src/say_hi.rs","line":4,"column":5}}
///    {"level":"error","timestamp":1722817572709,"payload":"something went wrong","tid":3479856,"source":{"module_path":"my_app::say_hi","file":"src/say_hi.rs","line":5,"column":5}}
///    ```
/// 
/// [`Level::as_str`]: crate::Level::as_str
/// [`SourceLocation`]: crate::SourceLocation
#[derive(Clone)]
pub struct JsonFormatter(PhantomData<()>);

impl JsonFormatter {
    /// Constructs a `JsonFormatter`.
    #[must_use]
    pub fn new() -> JsonFormatter {
        JsonFormatter(PhantomData)
    }

    fn format_impl(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut FormatterContext,
    ) -> Result<(), JsonFormatterError> {
        cfg_if! {
            if #[cfg(not(feature = "flexible-string"))] {
                dest.reserve(crate::string_buf::RESERVE_SIZE);
            }
        }

        // TODO: https://github.com/serde-rs/json/issues/863
        //
        // The performance can be significantly optimized here if the issue can be
        // solved.
        dest.write_str(&serde_json::to_string(&JsonRecord(record))?)?;

        dest.write_str(__EOL)?;

        Ok(())
    }
}

impl Formatter for JsonFormatter {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut FormatterContext,
    ) -> crate::Result<()> {
        self.format_impl(record, dest, ctx).map_err(Into::into)
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
    use crate::{kv, Level, SourceLocation, __EOL};

    #[test]
    fn should_format_json() {
        let mut dest = StringBuf::new();
        let formatter = JsonFormatter::new();
        let record = Record::new(Level::Info, "payload", None, None, &[]);
        let mut ctx = FormatterContext::new();
        formatter.format(&record, &mut dest, &mut ctx).unwrap();

        let local_time: DateTime<Local> = record.time().into();

        assert_eq!(ctx.style_range(), None);
        assert_eq!(
            dest.to_string(),
            format!(
                r#"{{"level":"info","timestamp":{},"payload":"{}","tid":{}}}{}"#,
                local_time.timestamp_millis(),
                "payload",
                record.tid(),
                __EOL
            )
        );
    }

    #[test]
    fn should_format_json_with_logger_name() {
        let mut dest = StringBuf::new();
        let formatter = JsonFormatter::new();
        let record = Record::new(Level::Info, "payload", None, Some("my-component"), &[]);
        let mut ctx = FormatterContext::new();
        formatter.format(&record, &mut dest, &mut ctx).unwrap();

        let local_time: DateTime<Local> = record.time().into();

        assert_eq!(ctx.style_range(), None);
        assert_eq!(
            dest.to_string(),
            format!(
                r#"{{"level":"info","timestamp":{},"payload":"{}","logger":"my-component","tid":{}}}{}"#,
                local_time.timestamp_millis(),
                "payload",
                record.tid(),
                __EOL
            )
        );
    }

    #[test]
    fn should_format_json_with_src_loc() {
        let mut dest = StringBuf::new();
        let formatter = JsonFormatter::new();
        let record = Record::new(
            Level::Info,
            "payload",
            Some(SourceLocation::__new("module", "file.rs", 1, 2)),
            None,
            &[],
        );
        let mut ctx = FormatterContext::new();
        formatter.format(&record, &mut dest, &mut ctx).unwrap();

        let local_time: DateTime<Local> = record.time().into();

        assert_eq!(ctx.style_range(), None);
        assert_eq!(
            dest.to_string(),
            format!(
                r#"{{"level":"info","timestamp":{},"payload":"{}","tid":{},"source":{{"module_path":"module","file":"file.rs","line":1,"column":2}}}}{}"#,
                local_time.timestamp_millis(),
                "payload",
                record.tid(),
                __EOL
            )
        );
    }

    #[test]
    fn should_format_json_with_kv() {
        let mut dest = StringBuf::new();
        let formatter = JsonFormatter::new();
        let kvs = [
            (kv::Key::__from_static_str("k1"), kv::Value::from(114)),
            (kv::Key::__from_static_str("k2"), kv::Value::from("514")),
        ];
        let record = Record::new(Level::Info, "payload", None, None, &kvs);
        let mut ctx = FormatterContext::new();
        formatter.format(&record, &mut dest, &mut ctx).unwrap();

        let local_time: DateTime<Local> = record.time().into();

        assert_eq!(ctx.style_range(), None);
        assert_eq!(
            dest.to_string(),
            format!(
                r#"{{"level":"info","timestamp":{},"payload":"{}","kv":{{"k1":114,"k2":"514"}},"tid":{}}}{}"#,
                local_time.timestamp_millis(),
                "payload",
                record.tid(),
                __EOL
            )
        );
    }
}

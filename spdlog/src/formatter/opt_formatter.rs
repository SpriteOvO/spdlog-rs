use std::fmt::{self, Write};

use crate::{
    formatter::{fmt_with_time, Formatter, FormatterContext, TimeDate},
    Error, Record, StringBuf, __EOL,
};

/// A formatter that fields can be opt-in / opt-out.
///
/// It enables all fields by default, formatting results are identical to
/// [`FullFormatter`] (in this case, use the latter for optimal performance).
///
/// By disabling fields, corresponding information can be removed from formatted
/// results.
///
/// [`FullFormatter`]: crate::formatter::FullFormatter
///
/// ## Examples
///
/// ```
/// use spdlog::formatter::OptFormatter;
/// # use spdlog::info;
#[doc = include_str!(concat!(env!("OUT_DIR"), "/test_utils/common_for_doc_test.rs"))]
/// #
///
/// let formatter = OptFormatter::builder()
///     .time(false)
///     .source_location(false)
///     .build();
/// // ... Setting up sinks with the formatter
/// # let (doctest, sink) = test_utils::echo_logger_from_formatter(formatter, None);
/// info!(logger: doctest, "Interesting log message");
/// # assert_eq!(
/// #     sink.clone_string().replace("\r", ""),
/// /* Output */ "[info] Interesting log message\n"
/// # );
///
/// let formatter = OptFormatter::builder()
///     .time(false)
///     .level(false)
///     .source_location(false)
///     .build();
/// // ... Setting up sinks with the formatter
/// # let (doctest, sink) = test_utils::echo_logger_from_formatter(formatter, None);
/// info!(logger: doctest, "Interesting log message");
/// # assert_eq!(
/// #     sink.clone_string().replace("\r", ""),
/// /* Output */ "Interesting log message\n"
/// # );
/// ```
#[derive(Clone)]
pub struct OptFormatter {
    options: FormattingOptions,
}

impl OptFormatter {
    /// Gets a builder of `OptFormatter` with default parameters:
    ///
    /// | Parameter         | Default Value |
    /// |-------------------|---------------|
    /// | [time]            | `true`        |
    /// | [logger_name]     | `true`        |
    /// | [level]           | `true`        |
    /// | [source_location] | `true`        |
    /// | [kv]              | `true`        |
    /// | [eol]             | `true`        |
    ///
    /// [time]: OptFormatterBuilder::time
    /// [logger_name]: OptFormatterBuilder::logger_name
    /// [level]: OptFormatterBuilder::level
    /// [source_location]: OptFormatterBuilder::source_location
    /// [kv]: OptFormatterBuilder::kv
    /// [eol]: OptFormatterBuilder::eol
    #[must_use]
    pub fn builder() -> OptFormatterBuilder {
        OptFormatterBuilder(FormattingOptions {
            time: true,
            logger_name: true,
            level: true,
            source_location: true,
            kv: true,
            eol: true,
        })
    }

    fn format_impl(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut FormatterContext,
    ) -> Result<(), fmt::Error> {
        #[cfg(not(feature = "flexible-string"))]
        dest.reserve(crate::string_buf::RESERVE_SIZE);

        let mut spacer = AutoSpacer::new();

        spacer.write_if(self.options.time, dest, |dest| {
            fmt_with_time(
                ctx,
                record,
                |mut time: TimeDate| -> Result<(), fmt::Error> {
                    dest.write_str("[")?;
                    dest.write_str(time.full_second_str())?;
                    dest.write_str(".")?;
                    write!(dest, "{:03}", time.millisecond())?;
                    dest.write_str("]")?;
                    Ok(())
                },
            )
        })?;
        spacer.write_if_opt(
            self.options.logger_name,
            record.logger_name(),
            dest,
            |dest, logger_name| {
                dest.write_str("[")?;
                dest.write_str(logger_name)?;
                dest.write_str("]")
            },
        )?;
        let mut style_range = None;
        spacer.write_if(self.options.level, dest, |dest| {
            dest.write_str("[")?;
            let style_range_begin = dest.len();
            dest.write_str(record.level().as_str())?;
            let style_range_end = dest.len();
            dest.write_str("]")?;
            style_range = Some(style_range_begin..style_range_end);
            Ok(())
        })?;
        spacer.write_if_opt(
            self.options.source_location,
            record.source_location(),
            dest,
            |dest, srcloc| {
                dest.write_str("[")?;
                dest.write_str(srcloc.module_path())?;
                dest.write_str(", ")?;
                dest.write_str(srcloc.file())?;
                dest.write_str(":")?;
                write!(dest, "{}", srcloc.line())?;
                dest.write_str("]")
            },
        )?;
        spacer.write_always(dest, |dest| dest.write_str(record.payload()))?;

        let key_values = record.key_values();
        spacer.write_if(self.options.kv && !key_values.is_empty(), dest, |dest| {
            dest.write_str("{ ")?;
            key_values.write_to(dest, false)?;
            dest.write_str(" }")
        })?;

        if self.options.eol {
            dest.write_str(__EOL)?;
        }

        ctx.set_style_range(style_range);
        Ok(())
    }
}

impl Formatter for OptFormatter {
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

#[expect(missing_docs)]
pub struct OptFormatterBuilder(FormattingOptions);

impl OptFormatterBuilder {
    /// Specify whether to enable time field.
    ///
    /// Example of this field: `[2022-11-02 09:23:12.263]`
    #[must_use]
    pub fn time(&mut self, value: bool) -> &mut Self {
        self.0.time = value;
        self
    }

    /// Specify whether to enable logger name field.
    ///
    /// Example of this field: `[logger-name]`
    #[must_use]
    pub fn logger_name(&mut self, value: bool) -> &mut Self {
        self.0.logger_name = value;
        self
    }

    /// Specify whether to enable level field.
    ///
    /// Note that disabling this field will also remove the style from the
    /// formatted result.
    ///
    /// Example of this field: <code>[<font color="#0DBC79">info</font>]</code>
    #[must_use]
    pub fn level(&mut self, value: bool) -> &mut Self {
        self.0.level = value;
        self
    }

    /// Specify whether to enable source location field.
    ///
    /// Example of this field: `[mod::path, src/main.rs:4]`
    #[must_use]
    pub fn source_location(&mut self, value: bool) -> &mut Self {
        self.0.source_location = value;
        self
    }

    /// Specify whether to enable kv field.
    ///
    /// Example of this field: `{ key1=value1 key2=value2 }`
    #[must_use]
    pub fn kv(&mut self, value: bool) -> &mut Self {
        self.0.kv = value;
        self
    }

    /// Specify whether to enable eol field.
    ///
    /// Example of this field: `\n` or `\r\n` on Windows.
    #[must_use]
    pub fn eol(&mut self, value: bool) -> &mut Self {
        self.0.eol = value;
        self
    }

    /// Builds a `OptFormatter`.
    #[must_use]
    pub fn build(&mut self) -> OptFormatter {
        OptFormatter {
            options: self.0.clone(),
        }
    }
}

#[derive(Clone)]
struct FormattingOptions {
    time: bool,
    logger_name: bool,
    level: bool,
    source_location: bool,
    kv: bool,
    eol: bool,
}

struct AutoSpacer(bool);

impl AutoSpacer {
    fn new() -> Self {
        Self(false)
    }

    fn write_always(
        &mut self,
        dest: &mut StringBuf,
        f: impl FnOnce(&mut StringBuf) -> fmt::Result,
    ) -> fmt::Result {
        if self.0 {
            dest.write_str(" ")?;
        } else {
            self.0 = true;
        }
        f(dest)?;
        Ok(())
    }

    fn write_if(
        &mut self,
        conf: bool,
        dest: &mut StringBuf,
        f: impl FnOnce(&mut StringBuf) -> fmt::Result,
    ) -> fmt::Result {
        if conf {
            if self.0 {
                dest.write_str(" ")?;
            } else {
                self.0 = true;
            }
            f(dest)?;
        }
        Ok(())
    }

    fn write_if_opt<O>(
        &mut self,
        conf: bool,
        option: Option<O>,
        dest: &mut StringBuf,
        f: impl FnOnce(&mut StringBuf, O) -> fmt::Result,
    ) -> fmt::Result {
        if conf {
            if let Some(option) = option {
                if self.0 {
                    dest.write_str(" ")?;
                } else {
                    self.0 = true;
                }
                f(dest, option)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{formatter::FullFormatter, kv, Level, RecordOwned};

    fn record() -> RecordOwned {
        let kvs = [
            (kv::Key::__from_static_str("k1"), kv::Value::from(114)),
            (kv::Key::__from_static_str("k2"), kv::Value::from("514")),
        ];
        Record::new(Level::Warn, "test log content", None, Some("logger"), &kvs).to_owned()
    }

    #[test]
    fn default_should_same_with_full() {
        let record = record();

        let opt = {
            let mut buf = StringBuf::new();
            let mut ctx = FormatterContext::new();
            OptFormatter::builder()
                .build()
                .format(&record.as_ref(), &mut buf, &mut ctx)
                .unwrap();
            (buf, ctx)
        };
        let full = {
            let mut buf = StringBuf::new();
            let mut ctx = FormatterContext::new();
            FullFormatter::new()
                .format(&record.as_ref(), &mut buf, &mut ctx)
                .unwrap();
            (buf, ctx)
        };

        assert_eq!(opt.0, full.0);
        assert_eq!(opt.1.style_range(), full.1.style_range());
    }

    #[test]
    fn no_time() {
        let record = record();
        let mut buf = StringBuf::new();
        let mut ctx = FormatterContext::new();
        OptFormatter::builder()
            .time(false)
            .build()
            .format(&record.as_ref(), &mut buf, &mut ctx)
            .unwrap();

        assert_eq!(
            buf,
            format!("[logger] [warn] test log content {{ k1=114 k2=514 }}{__EOL}")
        );
        assert_eq!(ctx.style_range(), Some(10..14));
    }

    #[test]
    fn no_time_logger_name() {
        let record = record();
        let mut buf = StringBuf::new();
        let mut ctx = FormatterContext::new();
        OptFormatter::builder()
            .time(false)
            .logger_name(false)
            .build()
            .format(&record.as_ref(), &mut buf, &mut ctx)
            .unwrap();

        assert_eq!(
            buf,
            format!("[warn] test log content {{ k1=114 k2=514 }}{__EOL}")
        );
        assert_eq!(ctx.style_range(), Some(1..5));
    }

    #[test]
    fn no_time_logger_name_level() {
        let record = record();
        let mut buf = StringBuf::new();
        let mut ctx = FormatterContext::new();
        OptFormatter::builder()
            .time(false)
            .logger_name(false)
            .level(false)
            .build()
            .format(&record.as_ref(), &mut buf, &mut ctx)
            .unwrap();

        assert_eq!(buf, format!("test log content {{ k1=114 k2=514 }}{__EOL}"));
        assert!(ctx.style_range().is_none());
    }

    #[test]
    fn no_time_logger_name_level_kv() {
        let record = record();
        let mut buf = StringBuf::new();
        let mut ctx = FormatterContext::new();
        OptFormatter::builder()
            .time(false)
            .logger_name(false)
            .level(false)
            .kv(false)
            .build()
            .format(&record.as_ref(), &mut buf, &mut ctx)
            .unwrap();

        assert_eq!(buf, format!("test log content{__EOL}"));
        assert!(ctx.style_range().is_none());
    }

    #[test]
    fn no_time_eol() {
        let record = record();
        let mut buf = StringBuf::new();
        let mut ctx = FormatterContext::new();
        OptFormatter::builder()
            .time(false)
            .eol(false)
            .build()
            .format(&record.as_ref(), &mut buf, &mut ctx)
            .unwrap();

        assert_eq!(buf, "[logger] [warn] test log content { k1=114 k2=514 }");
        assert_eq!(ctx.style_range(), Some(10..14));
    }
}

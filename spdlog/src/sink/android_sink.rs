use std::{ffi::CString, io, ptr::null, result::Result as StdResult};

use libc::EPERM;

use crate::{
    formatter::{AndroidFormatter, Formatter, FormatterContext},
    prelude::*,
    sink::{GetSinkProp, Sink, SinkProp},
    sync::*,
    Error, ErrorHandler, Record, Result, StringBuf,
};

#[cfg(not(doc))]
mod ffi {
    use android_log_sys::{LogPriority, __android_log_write, c_int};

    use super::*;

    pub(super) struct AndroidLevelsMapping([LogPriority; Level::count()]);

    impl AndroidLevelsMapping {
        #[must_use]
        pub(super) const fn new() -> Self {
            Self([
                LogPriority::FATAL,   // spdlog::Critical
                LogPriority::ERROR,   // spdlog::Error
                LogPriority::WARN,    // spdlog::Warn
                LogPriority::INFO,    // spdlog::Info
                LogPriority::DEBUG,   // spdlog::Debug
                LogPriority::VERBOSE, // spdlog::Trace
            ])
        }

        #[must_use]
        pub(super) fn level(&self, level: Level) -> LogPriority {
            self.0[level as usize]
        }
    }

    pub(super) fn android_log_write(
        priority: LogPriority,
        tag: Option<&str>,
        text: &str,
    ) -> StdResult<(), io::Error> {
        let tag = tag
            .map(CString::new)
            .transpose()
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        let text =
            CString::new(text).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

        let tag_ptr = tag.as_deref().map(|tag| tag.as_ptr()).unwrap_or_else(null);
        let text_ptr = text.as_ptr();

        let result = unsafe { __android_log_write(priority as c_int, tag_ptr, text_ptr) };

        // Explicitly drop to ensure that they have not been moved to cause dangling
        // pointers.
        drop((tag, text));

        // Although the documentation [^1] says that:
        //   1 if the message was written to the log, or -EPERM if it was not;
        //
        // It doesn't point out that the behavior differs between versions. The above
        // behavior is available since Android 11 (API 30). Before that, the behavior of
        // the return value was not clarified in the documentation, but referring to the
        // implementation, for a successful log write, the number of bytes written is
        // actually returned. This behavior is changed in this commit [^2].
        //
        // For compatible with more versions, we do not use `result == 1` as the success
        // condition, but `result >= 0` instead.
        //
        // [^1]: https://developer.android.com/ndk/reference/group/logging#group___logging_1ga32a7173b092ec978b50490bd12ee523b
        // [^2]: https://android.googlesource.com/platform/system/logging/+/c17613c4582d4f6eecb3965bb96584f25762b827%5E%21/
        //
        // ---
        //
        // For the condition `result == -EPERM`, see
        // https://github.com/gabime/spdlog/commit/01b3724c484eebb42d83fa21aa8d71a57b2b8fb6
        if result >= 0 || /* !__android_log_is_loggable */ result == -EPERM {
            Ok(())
        } else {
            Err(io::Error::from_raw_os_error(-result))
        }
    }
}

/// Represents how to choose a tag for Android logs.
///
/// # Log Level Mapping
///
/// | spdlog-rs  | Android NDK |
/// |------------|-------------|
/// | `Critical` | `FATAL`     |
/// | `Error`    | `ERROR`     |
/// | `Warn`     | `WARN`      |
/// | `Info`     | `INFO`      |
/// | `Debug`    | `DEBUG`     |
/// | `Trace`    | `VERBOSE`   |
///
/// # Note
///
/// It requires linking to Android NDK `liblog`.
pub enum AndroidLogTag {
    /// The default tag determined by Android NDK.
    Default,
    /// The name of the `spdlog-rs` logger that generated the log.
    LoggerName,
    /// A custom string.
    Custom(String),
}

#[allow(missing_docs)]
pub struct AndroidSinkBuilder {
    prop: SinkProp,
    tag: AndroidLogTag,
}

impl AndroidSinkBuilder {
    /// Specifies how to choose a tag for Android logs.
    ///
    /// This parameter is **optional**.
    #[must_use]
    pub fn tag(mut self, tag: AndroidLogTag) -> Self {
        self.tag = tag;
        self
    }

    // Prop
    //

    /// Specifies a log level filter.
    ///
    /// This parameter is **optional**.
    #[must_use]
    pub fn level_filter(self, level_filter: LevelFilter) -> Self {
        self.prop.set_level_filter(level_filter);
        self
    }

    /// Specifies a formatter.
    ///
    /// This parameter is **optional**.
    #[must_use]
    pub fn formatter<F>(self, formatter: F) -> Self
    where
        F: Formatter + 'static,
    {
        self.prop.set_formatter(formatter);
        self
    }

    /// Specifies an error handler.
    ///
    /// This parameter is **optional**.
    #[must_use]
    pub fn error_handler<F: Into<ErrorHandler>>(self, handler: F) -> Self {
        self.prop.set_error_handler(handler);
        self
    }

    //

    /// Constructs a `AndroidSink`.
    pub fn build(self) -> Result<AndroidSink> {
        Ok(AndroidSink {
            prop: self.prop,
            tag: self.tag,
        })
    }

    /// Builds a `Arc<AndroidSink>`.
    ///
    /// This is a shorthand method for `.build().map(Arc::new)`.
    pub fn build_arc(self) -> Result<Arc<AndroidSink>> {
        self.build().map(Arc::new)
    }
}

/// A sink with Android NDK API `__android_log_write` as the target.
pub struct AndroidSink {
    prop: SinkProp,
    tag: AndroidLogTag,
}

impl AndroidSink {
    #[cfg(not(doc))]
    const LEVELS_MAPPING: ffi::AndroidLevelsMapping = ffi::AndroidLevelsMapping::new();

    /// Gets a builder of `AndroidSink` with default parameters:
    ///
    /// | Parameter       | Default Value               |
    /// |-----------------|-----------------------------|
    /// | [level_filter]  | `All`                       |
    /// | [formatter]     | `AndroidFormatter`          |
    /// | [error_handler] | [`ErrorHandler::default()`] |
    /// |                 |                             |
    /// | [tag]           | `AndroidSink::Default`      |
    ///
    /// [level_filter]: AndroidSinkBuilder::level_filter
    /// [formatter]: AndroidSinkBuilder::formatter
    /// [error_handler]: AndroidSinkBuilder::error_handler
    /// [`ErrorHandler::default()`]: crate::error::ErrorHandler::default()
    /// [tag]: AndroidSinkBuilder::tag
    #[must_use]
    pub fn builder() -> AndroidSinkBuilder {
        let prop = SinkProp::default();
        prop.set_formatter(AndroidFormatter::new());

        AndroidSinkBuilder {
            prop,
            tag: AndroidLogTag::Default,
        }
    }

    /// Gets how to choose a tag for Android logs.
    #[must_use]
    pub fn tag(&self) -> &AndroidLogTag {
        &self.tag
    }

    /// Sets how to choose a tag for Android logs.
    pub fn set_tag(&mut self, tag: AndroidLogTag) {
        self.tag = tag;
    }
}

impl GetSinkProp for AndroidSink {
    fn prop(&self) -> &SinkProp {
        &self.prop
    }
}

impl Sink for AndroidSink {
    fn log(&self, record: &Record) -> Result<()> {
        let mut string_buf = StringBuf::new();
        let mut ctx = FormatterContext::new();
        self.prop
            .formatter()
            .format(record, &mut string_buf, &mut ctx)?;

        let priority = Self::LEVELS_MAPPING.level(record.level());
        let tag = match &self.tag {
            AndroidLogTag::Default => None,
            AndroidLogTag::LoggerName => record.logger_name(),
            AndroidLogTag::Custom(tag) => Some(tag.as_str()),
        };
        ffi::android_log_write(priority, tag, &string_buf).map_err(Error::WriteRecord)
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }
}

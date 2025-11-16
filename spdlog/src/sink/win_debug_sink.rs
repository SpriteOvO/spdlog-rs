use std::{ffi::OsStr, iter::once};

use crate::{
    formatter::{Formatter, FormatterContext},
    sink::{GetSinkProp, Sink, SinkProp},
    sync::*,
    ErrorHandler, LevelFilter, Record, Result, StringBuf,
};

/// A sink with a win32 API `OutputDebugStringW` as the target.
pub struct WinDebugSink {
    prop: SinkProp,
}

impl WinDebugSink {
    /// Gets a builder of `WinDebugSink` with default parameters:
    ///
    /// | Parameter       | Default Value               |
    /// |-----------------|-----------------------------|
    /// | [level_filter]  | [`LevelFilter::All`]        |
    /// | [formatter]     | [`FullFormatter`]           |
    /// | [error_handler] | [`ErrorHandler::default()`] |
    ///
    /// [level_filter]: WinDebugSinkBuilder::level_filter
    /// [formatter]: WinDebugSinkBuilder::formatter
    /// [`FullFormatter`]: crate::formatter::FullFormatter
    /// [error_handler]: WinDebugSinkBuilder::error_handler
    #[must_use]
    pub fn builder() -> WinDebugSinkBuilder {
        WinDebugSinkBuilder {
            prop: SinkProp::default(),
        }
    }

    /// Constructs a `WinDebugSink`.
    #[allow(clippy::new_without_default)]
    #[deprecated(
        since = "0.3.0",
        note = "it may be removed in the future, use `WinDebugSink::builder()` instead"
    )]
    #[must_use]
    pub fn new() -> WinDebugSink {
        WinDebugSink::builder().build().unwrap()
    }
}

impl GetSinkProp for WinDebugSink {
    fn prop(&self) -> &SinkProp {
        &self.prop
    }
}

impl Sink for WinDebugSink {
    fn log(&self, record: &Record) -> Result<()> {
        #[cfg(windows)] // https://github.com/rust-lang/rust/issues/97976
        use std::os::windows::ffi::OsStrExt;

        let mut string_buf = StringBuf::new();
        let mut ctx = FormatterContext::new();
        self.prop
            .formatter()
            .format(record, &mut string_buf, &mut ctx)?;

        let wide: Vec<u16> = OsStr::new(&string_buf)
            .encode_wide()
            .chain(once(0))
            .collect();
        let wide = wide.as_ptr();

        unsafe { winapi::um::debugapi::OutputDebugStringW(wide) }

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }
}

#[allow(missing_docs)]
pub struct WinDebugSinkBuilder {
    prop: SinkProp,
}

impl WinDebugSinkBuilder {
    // Prop
    //

    /// Specifies a log level filter.
    ///
    /// This parameter is **optional**, and defaults to [`LevelFilter::All`].
    #[must_use]
    pub fn level_filter(self, level_filter: LevelFilter) -> Self {
        self.prop.set_level_filter(level_filter);
        self
    }

    /// Specifies a formatter.
    ///
    /// This parameter is **optional**, and defaults to [`FullFormatter`].
    ///
    /// [`FullFormatter`]: crate::formatter::FullFormatter
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
    /// This parameter is **optional**, and defaults to
    /// [`ErrorHandler::default()`].
    #[must_use]
    pub fn error_handler<F: Into<ErrorHandler>>(self, handler: F) -> Self {
        self.prop.set_error_handler(handler);
        self
    }

    //

    /// Builds a [`WinDebugSink`].
    pub fn build(self) -> Result<WinDebugSink> {
        let sink = WinDebugSink { prop: self.prop };
        Ok(sink)
    }

    /// Builds a `Arc<WinDebugSink>`.
    ///
    /// This is a shorthand method for `.build().map(Arc::new)`.
    pub fn build_arc(self) -> Result<Arc<WinDebugSink>> {
        self.build().map(Arc::new)
    }
}

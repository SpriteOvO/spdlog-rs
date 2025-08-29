use std::{ffi::OsStr, iter::once};

use crate::{
    formatter::{Formatter, FormatterContext},
    sink::{GetSinkProp, Sink, SinkProp},
    ErrorHandler, LevelFilter, Record, Result, StringBuf,
};

/// A sink with a win32 API `OutputDebugStringW` as the target.
pub struct WinDebugSink {
    prop: SinkProp,
}

impl WinDebugSink {
    /// Gets a builder of `WinDebugSink` with default parameters:
    ///
    /// | Parameter       | Default Value           |
    /// |-----------------|-------------------------|
    /// | [level_filter]  | `All`                   |
    /// | [formatter]     | `FullFormatter`         |
    /// | [error_handler] | [default error handler] |
    ///
    /// [level_filter]: WinDebugSinkBuilder::level_filter
    /// [formatter]: WinDebugSinkBuilder::formatter
    /// [error_handler]: WinDebugSinkBuilder::error_handler
    /// [default error handler]: error/index.html#default-error-handler
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
    pub fn error_handler(self, handler: ErrorHandler) -> Self {
        self.prop.set_error_handler(Some(handler));
        self
    }

    //

    /// Builds a [`WinDebugSink`].
    pub fn build(self) -> Result<WinDebugSink> {
        let sink = WinDebugSink { prop: self.prop };
        Ok(sink)
    }
}

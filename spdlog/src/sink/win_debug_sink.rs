use std::{ffi::OsStr, iter::once};

use crate::{
    formatter::FormatterContext,
    sink::{helper, Sink},
    Record, Result, StringBuf,
};

/// A sink with a win32 API `OutputDebugStringW` as the target.
pub struct WinDebugSink {
    common_impl: helper::CommonImpl,
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
            common_builder_impl: helper::CommonBuilderImpl::new(),
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

impl Sink for WinDebugSink {
    fn log(&self, record: &Record) -> Result<()> {
        #[cfg(windows)] // https://github.com/rust-lang/rust/issues/97976
        use std::os::windows::ffi::OsStrExt;

        let mut string_buf = StringBuf::new();
        let mut ctx = FormatterContext::new();
        self.common_impl
            .formatter
            .read()
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

    helper::common_impl!(@Sink: common_impl);
}

#[allow(missing_docs)]
pub struct WinDebugSinkBuilder {
    common_builder_impl: helper::CommonBuilderImpl,
}

impl WinDebugSinkBuilder {
    helper::common_impl!(@SinkBuilder: common_builder_impl);

    /// Builds a [`WinDebugSink`].
    pub fn build(self) -> Result<WinDebugSink> {
        let sink = WinDebugSink {
            common_impl: helper::CommonImpl::from_builder(self.common_builder_impl),
        };
        Ok(sink)
    }
}

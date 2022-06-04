use std::{ffi::OsStr, iter::once, os::windows::ffi::OsStrExt};

use winapi::um::debugapi::OutputDebugStringW;

use crate::{
    sink::{helper, Sink},
    Record, Result, StringBuf,
};

/// A sink with a win32 API `OutputDebugStringW` as the target.
pub struct WinDebugSink {
    common_impl: helper::CommonImpl,
}

impl WinDebugSink {
    /// Constructs a builder of `WinDebugSink`.
    pub fn builder() -> WinDebugSinkBuilder {
        WinDebugSinkBuilder {
            common_builder_impl: helper::CommonBuilderImpl::new(),
        }
    }

    /// Constructs a `WinDebugSink`.
    #[allow(clippy::new_without_default)]
    #[deprecated(note = "it may be removed in the future, use `WinDebugSink::builder()` instead")]
    pub fn new() -> WinDebugSink {
        WinDebugSink::builder().build().unwrap()
    }
}

impl Sink for WinDebugSink {
    fn log(&self, record: &Record) -> Result<()> {
        if !self.should_log(record.level()) {
            return Ok(());
        }

        let mut string_buf = StringBuf::new();
        self.common_impl
            .formatter
            .read()
            .format(record, &mut string_buf)?;

        let wide: Vec<u16> = OsStr::new(&string_buf)
            .encode_wide()
            .chain(once(0))
            .collect();
        let wide = wide.as_ptr();

        unsafe { OutputDebugStringW(wide) }

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }

    helper::common_impl!(@Sink: common_impl);
}

/// The builder of [`WinDebugSink`].
///
/// # Examples
///
/// - Building a [`WinDebugSink`].
///
///   ```
///   use spdlog::{prelude::*, sink::WinDebugSink};
///  
///   let sink: spdlog::Result<WinDebugSink> = WinDebugSink::builder()
///       .level_filter(LevelFilter::MoreSevere(Level::Info)) // optional
///       .build();
///   ```
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

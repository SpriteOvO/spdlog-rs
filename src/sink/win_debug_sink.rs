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
    /// Constructs a `WinDebugSink`.
    pub fn new() -> WinDebugSink {
        WinDebugSink {
            common_impl: helper::CommonImpl::new(),
        }
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

impl Default for WinDebugSink {
    fn default() -> Self {
        Self::new()
    }
}

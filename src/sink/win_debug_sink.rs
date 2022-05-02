use std::{ffi::OsStr, iter::once, os::windows::ffi::OsStrExt};

use winapi::um::debugapi::OutputDebugStringW;

use crate::{
    formatter::{Formatter, FullFormatter},
    sink::Sink,
    sync::*,
    LevelFilter, Record, Result, StringBuf,
};

/// A sink with a win32 API `OutputDebugStringW` as the target.
pub struct WinDebugSink {
    level_filter: Atomic<LevelFilter>,
    formatter: SpinRwLock<Box<dyn Formatter>>,
}

impl WinDebugSink {
    /// Constructs a `WinDebugSink`.
    pub fn new() -> WinDebugSink {
        WinDebugSink {
            level_filter: Atomic::new(LevelFilter::All),
            formatter: SpinRwLock::new(Box::new(FullFormatter::new())),
        }
    }
}

impl Sink for WinDebugSink {
    fn log(&self, record: &Record) -> Result<()> {
        if !self.should_log(record.level()) {
            return Ok(());
        }

        let mut string_buf = StringBuf::new();
        self.formatter.read().format(record, &mut string_buf)?;

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

    fn level_filter(&self) -> LevelFilter {
        self.level_filter.load(Ordering::Relaxed)
    }

    fn set_level_filter(&self, level_filter: LevelFilter) {
        self.level_filter.store(level_filter, Ordering::Relaxed);
    }

    fn set_formatter(&self, formatter: Box<dyn Formatter>) {
        *self.formatter.write() = formatter;
    }
}

impl Default for WinDebugSink {
    fn default() -> Self {
        Self::new()
    }
}

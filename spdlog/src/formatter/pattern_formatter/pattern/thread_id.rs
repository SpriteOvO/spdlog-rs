use std::fmt::Write;

use crate::{
    formatter::pattern_formatter::{Pattern, PatternContext},
    Error, Record, StringBuf,
};

/// A pattern that writes the current thread's ID into the output. Example:
/// `3132`.
///
/// # Implementation
///
/// On Linux, this pattern writes the return value of `gettid` to the output.
///
/// On macOS, this pattern writes the return value of `pthread_self` to the
/// output.
///
/// On Windows, this pattern writes the return value of `GetCurrentThreadId` to
/// the output.
#[derive(Copy, Clone, Debug, Default)]
pub struct ThreadId;

impl ThreadId {
    /// Create a new `ThreadId` pattern.
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for ThreadId {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let tid = get_current_thread_id();
        write!(dest, "{}", tid).map_err(Error::FormatRecord)
    }
}

#[cfg(target_os = "linux")]
fn get_current_thread_id() -> u64 {
    let tid = unsafe { libc::gettid() };
    tid as u64
}

#[cfg(target_os = "macos")]
fn get_current_thread_id() -> u64 {
    let tid = unsafe { libc::pthread_self() };
    tid as u64
}

#[cfg(target_os = "windows")]
fn get_current_thread_id() -> u64 {
    let tid = unsafe { winapi::um::processthreadsapi::GetCurrentThreadId() };
    tid as u64
}

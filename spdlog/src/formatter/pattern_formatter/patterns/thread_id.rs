use std::fmt::Write;

use crate::{
    formatter::pattern_formatter::{Pattern, PatternContext},
    Error, Record, StringBuf,
};

/// A pattern that writes the current thread's ID into the output. Example:
/// `3132`.
///
/// This pattern corresponds to `{t}` or `{tid}` in the pattern template string.
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
#[derive(Copy, Clone, Debug)]
pub struct ThreadId;

impl Pattern for ThreadId {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let id = get_current_thread_id();
        dest.write_fmt(format_args!("{}", id))
            .map_err(Error::FormatRecord)?;
        Ok(())
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

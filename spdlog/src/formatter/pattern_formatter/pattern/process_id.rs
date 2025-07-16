use std::fmt::Write;

use crate::{
    formatter::pattern_formatter::{Pattern, PatternContext},
    Error, Record, StringBuf,
};

/// A pattern that writes the current thread's ID into the output. Example:
/// `3824`.
///
/// # Implementation
///
/// On unix-like systems such as Linux and macOS, this pattern writes the return
/// value of `getpid` to the output.
///
/// On Windows, this pattern writes the return value of `GetCurrentProcessId` to
/// the output.
#[derive(Clone, Default)]
pub struct ProcessId;

impl Pattern for ProcessId {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let pid = get_current_process_id();
        write!(dest, "{pid}").map_err(Error::FormatRecord)
    }
}

// TODO: We can cache the PID someway to improve the performance, but remember
// to test the case of process forking.

#[cfg(target_family = "unix")]
#[must_use]
fn get_current_process_id() -> u64 {
    let pid = unsafe { libc::getpid() };
    pid as u64
}

#[cfg(target_os = "windows")]
#[must_use]
fn get_current_process_id() -> u64 {
    let pid = unsafe { winapi::um::processthreadsapi::GetCurrentProcessId() };
    pid as u64
}

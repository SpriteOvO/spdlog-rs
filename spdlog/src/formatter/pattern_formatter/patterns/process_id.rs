use std::fmt::Write;

use crate::{
    formatter::pattern_formatter::{Pattern, PatternContext},
    Error, Record, StringBuf,
};

/// A pattern that writes the current thread's ID into the output. Example:
/// `3824`.
///
/// This pattern corresponds to `{P}` or `{pid}` in the pattern template string.
///
/// # Implementation
///
/// On unix-like systems such as Linux and macOS, this pattern writes the return
/// value of `getpid` to the output.
///
/// On Windows, this pattern writes the return value of `GetCurrentProcessId` to
/// the output.
#[derive(Copy, Clone, Debug, Default)]
pub struct ProcessId;

impl ProcessId {
    /// Create a new `ProcessId` pattern.
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for ProcessId {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let pid = get_current_process_id();
        dest.write_fmt(format_args!("{}", pid))
            .map_err(Error::FormatRecord)?;
        Ok(())
    }
}

#[cfg(target_family = "unix")]
fn get_current_process_id() -> u64 {
    let pid = unsafe { libc::getpid() };
    pid as u64
}

#[cfg(target_os = "windows")]
fn get_current_process_id() -> u64 {
    let pid = unsafe { winapi::um::processthreadsapi::GetCurrentProcessId() };
    pid as u64
}

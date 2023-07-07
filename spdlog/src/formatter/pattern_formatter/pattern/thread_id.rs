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
#[derive(Clone, Default)]
pub struct ThreadId;

impl Pattern for ThreadId {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        write!(dest, "{}", record.tid()).map_err(Error::FormatRecord)
    }
}

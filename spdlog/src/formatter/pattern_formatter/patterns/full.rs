use crate::{
    formatter::{
        pattern_formatter::{Pattern, PatternContext},
        Formatter, FullFormatter,
    },
    Record, StringBuf,
};

/// A pattern that writes the fully formatted text of log records into the
/// output.
///
/// This pattern writes the same formatted text as [`FullFormatter`].
///
/// This pattern corresponds to `{+}` or `{full}` in the pattern template
/// string.
#[derive(Default)]
pub struct Full {
    full_formatter: FullFormatter,
}

impl Full {
    /// Create a new `FullPattern`.
    pub fn new() -> Self {
        Self {
            full_formatter: FullFormatter::new(),
        }
    }
}

impl Pattern for Full {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        self.full_formatter.format(record, dest)?;
        Ok(())
    }
}

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
#[derive(Clone)]
pub struct Full {
    full_formatter: FullFormatter,
}

impl Default for Full {
    fn default() -> Self {
        Full {
            full_formatter: FullFormatter::without_eol(),
        }
    }
}

impl Pattern for Full {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let extra_info = self.full_formatter.format(record, dest)?;
        if let Some(style_range) = extra_info.style_range {
            // Before we support multiple style ranges, if there is already a style range
            // set, we don't override it.
            if ctx.fmt_info_builder.info.style_range.is_none() {
                ctx.set_style_range(style_range)
            }
        }
        Ok(())
    }
}

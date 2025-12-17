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
            full_formatter: FullFormatter::builder().eol(false).build(),
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
        let saved_style_range = ctx.fmt_ctx.style_range.clone();

        self.full_formatter.format(record, dest, ctx.fmt_ctx)?;

        // TODO: Before we support multiple style ranges, if there is already a style
        // range set, we don't override it.
        if let Some(saved_style_range) = saved_style_range {
            ctx.fmt_ctx.set_style_range(Some(saved_style_range));
        }
        Ok(())
    }
}

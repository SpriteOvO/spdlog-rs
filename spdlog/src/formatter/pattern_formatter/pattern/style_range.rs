use crate::{
    formatter::pattern_formatter::{Pattern, PatternContext},
    Record, StringBuf,
};

/// A pattern that wraps another pattern and apply style ranges to the content
/// formatted by the wrapped pattern.
#[derive(Clone, Debug, Default)]
pub struct StyleRange<P> {
    inner: P,
}

impl<P> StyleRange<P>
where
    P: Pattern,
{
    /// Create a new `StyleRange` pattern that wraps the given inner pattern.
    pub fn new(inner: P) -> Self {
        Self { inner }
    }
}

impl<P> Pattern for StyleRange<P>
where
    P: Pattern,
{
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let style_range_start = dest.len();

        self.inner.format(record, dest, ctx)?;

        let style_range_end = dest.len();
        ctx.set_style_range(style_range_start..style_range_end);

        Ok(())
    }
}

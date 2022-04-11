use crate::{
    formatter::pattern_formatter::{Pattern, PatternContext},
    Record, StringBuf,
};

/// A pattern that wraps another pattern and apply color ranges to the content
/// formatted by the wrapped pattern.
///
/// This pattern corresponds to `{^..$}` in the pattern template string.
#[derive(Clone, Debug, Default)]
pub struct ColorRange<P> {
    inner: P,
}

impl<P> ColorRange<P> {
    /// Create a new `ColorRange` pattern that wraps the given inner pattern.
    pub fn new(inner: P) -> Self {
        Self { inner }
    }
}

impl<P> Pattern for ColorRange<P>
where
    P: Pattern,
{
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let color_range_start = dest.len();

        self.inner.format(record, dest, ctx)?;

        let color_range_end = dest.len();
        ctx.set_style_range(color_range_start..color_range_end);

        Ok(())
    }
}

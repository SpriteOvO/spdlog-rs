use std::marker::PhantomData;

use crate::{
    formatter::{Formatter, FormatterContext},
    Record, StringBuf,
};

#[derive(Clone)]
pub(crate) struct UnreachableFormatter(PhantomData<()>);

impl UnreachableFormatter {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}

impl Formatter for UnreachableFormatter {
    fn format(
        &self,
        _record: &Record,
        _dest: &mut StringBuf,
        _ctx: &mut FormatterContext,
    ) -> crate::Result<()> {
        unreachable!()
    }
}

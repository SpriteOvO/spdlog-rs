//! This module provides a pattern-string based formatter.
//!
//! The [`PatternFormatter`] struct defines a formatter that formats log
//! messages against a specific text pattern.
//!
//! Patterns are represented by the [`Pattern`] trait. You can create your own
//! pattern by implementing the [`Pattern`] trait.

pub mod patterns;

use std::{fmt::Write, ops::Range, sync::Arc};

pub use spdlog_macros::pattern;

use crate::{
    formatter::{FmtExtraInfo, FmtExtraInfoBuilder, Formatter},
    Error, Record, StringBuf,
};

/// A formatter that formats log records against a specified text pattern.
pub struct PatternFormatter<P> {
    pattern: P,
}

impl<P> PatternFormatter<P> {
    /// Create a new `PatternFormatter` object with the given pattern.
    ///
    /// Manually craft a pattern object `pattern` can be tedious and
    /// error-prone. It's recommended to use the `pattern!` macro to create
    /// a pattern object from a pattern string.
    pub fn new(pattern: P) -> Self {
        Self { pattern }
    }
}

impl<P> Formatter for PatternFormatter<P>
where
    P: Pattern,
{
    fn format(&self, record: &Record, dest: &mut StringBuf) -> crate::Result<FmtExtraInfo> {
        let mut ctx = PatternContext::new(FmtExtraInfoBuilder::default());
        self.pattern.format(record, dest, &mut ctx)?;
        Ok(ctx.fmt_info_builder.build())
    }

    fn clone_box(&self) -> Box<dyn Formatter> {
        unimplemented!() // TODO
    }
}

/// Provide context for patterns.
#[derive(Clone, Debug)]
pub struct PatternContext {
    fmt_info_builder: FmtExtraInfoBuilder,
}

impl PatternContext {
    /// Create a new `PatternContext` object.
    fn new(fmt_info_builder: FmtExtraInfoBuilder) -> Self {
        Self { fmt_info_builder }
    }

    /// Set the style range of the log message written by the patterns.
    ///
    /// This function is reserved for use by the color range pattern. Other
    /// built-in patterns should not use this function. User-defined
    /// patterns cannot use this function due to type privacy.
    fn set_style_range(&mut self, style_range: Range<usize>) {
        let builder = std::mem::take(&mut self.fmt_info_builder);
        self.fmt_info_builder = builder.style_range(style_range);
    }
}

/// A pattern.
///
/// A pattern is like a formatter, except that multiple patterns can be combined
/// in various ways to create a new pattern.
pub trait Pattern: Send + Sync {
    /// Format this pattern against the given log record and write the formatted
    /// message into the output buffer.
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()>;
}

impl Pattern for String {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        <&str as Pattern>::format(&&**self, record, dest, ctx)
    }
}

impl<'a> Pattern for &'a str {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(*self).map_err(Error::FormatRecord)?;
        Ok(())
    }
}

impl<'a, T> Pattern for &'a T
where
    T: ?Sized + Pattern,
{
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        <T as Pattern>::format(*self, record, dest, ctx)
    }
}

impl<'a, T> Pattern for &'a mut T
where
    T: ?Sized + Pattern,
{
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        <T as Pattern>::format(*self, record, dest, ctx)
    }
}

impl<T> Pattern for Box<T>
where
    T: ?Sized + Pattern,
{
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        <T as Pattern>::format(&**self, record, dest, ctx)
    }
}

impl<T> Pattern for Arc<T>
where
    T: ?Sized + Pattern,
{
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        <T as Pattern>::format(&**self, record, dest, ctx)
    }
}

impl<T> Pattern for [T]
where
    T: Pattern,
{
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        for p in self {
            <T as Pattern>::format(p, record, dest, ctx)?;
        }
        Ok(())
    }
}

impl<T, const N: usize> Pattern for [T; N]
where
    T: Pattern,
{
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        <[T] as Pattern>::format(self, record, dest, ctx)
    }
}

impl<T> Pattern for Vec<T>
where
    T: Pattern,
{
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        <[T] as Pattern>::format(self, record, dest, ctx)
    }
}

macro_rules! last {
    ( $a:tt, ) => { $a };
    ( $a:tt, $($rest:tt,)+ ) => { last!($($rest,)+) };
}

macro_rules! tuple_pattern {
    (
        $(
            $Tuple:ident {
                $(
                    ($idx:tt) -> $T:ident
                )+
            }
        )+
    ) => {
        $(
            impl<$($T,)+> Pattern for ($($T,)+)
            where
                $($T : Pattern,)+
                last!($($T,)+) : ?Sized,
            {
                fn format(&self, record: &Record, dest: &mut StringBuf, ctx: &mut PatternContext) -> crate::Result<()> {
                    $(
                        <$T as Pattern>::format(&self.$idx, record, dest, ctx)?;
                    )+
                    Ok(())
                }
            }
        )+
    };
}

impl Pattern for () {
    fn format(
        &self,
        _record: &Record,
        _dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        Ok(())
    }
}

tuple_pattern! {
    Tuple1 {
        (0) -> T0
    }
    Tuple2 {
        (0) -> T0
        (1) -> T1
    }
    Tuple3 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
    }
    Tuple4 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
    }
    Tuple5 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
    }
    Tuple6 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
    }
    Tuple7 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
    }
    Tuple8 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
    }
    Tuple9 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
    }
    Tuple10 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
    }
    Tuple11 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
    }
    Tuple12 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
    }
    Tuple13 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
    }
    Tuple14 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
    }
    Tuple15 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
        (14) -> T14
    }
    Tuple16 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
        (14) -> T14
        (15) -> T15
    }
    Tuple17 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
        (14) -> T14
        (15) -> T15
        (16) -> T16
    }
    Tuple18 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
        (14) -> T14
        (15) -> T15
        (16) -> T16
        (17) -> T17
    }
    Tuple19 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
        (14) -> T14
        (15) -> T15
        (16) -> T16
        (17) -> T17
        (18) -> T18
    }
    Tuple20 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
        (14) -> T14
        (15) -> T15
        (16) -> T16
        (17) -> T17
        (18) -> T18
        (19) -> T19
    }
    Tuple21 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
        (14) -> T14
        (15) -> T15
        (16) -> T16
        (17) -> T17
        (18) -> T18
        (19) -> T19
        (20) -> T20
    }
    Tuple22 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
        (14) -> T14
        (15) -> T15
        (16) -> T16
        (17) -> T17
        (18) -> T18
        (19) -> T19
        (20) -> T20
        (21) -> T21
    }
    Tuple23 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
        (14) -> T14
        (15) -> T15
        (16) -> T16
        (17) -> T17
        (18) -> T18
        (19) -> T19
        (20) -> T20
        (21) -> T21
        (22) -> T22
    }
    Tuple24 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
        (14) -> T14
        (15) -> T15
        (16) -> T16
        (17) -> T17
        (18) -> T18
        (19) -> T19
        (20) -> T20
        (21) -> T21
        (22) -> T22
        (23) -> T23
    }
    Tuple25 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
        (14) -> T14
        (15) -> T15
        (16) -> T16
        (17) -> T17
        (18) -> T18
        (19) -> T19
        (20) -> T20
        (21) -> T21
        (22) -> T22
        (23) -> T23
        (24) -> T24
    }
    Tuple26 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
        (14) -> T14
        (15) -> T15
        (16) -> T16
        (17) -> T17
        (18) -> T18
        (19) -> T19
        (20) -> T20
        (21) -> T21
        (22) -> T22
        (23) -> T23
        (24) -> T24
        (25) -> T25
    }
    Tuple27 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
        (14) -> T14
        (15) -> T15
        (16) -> T16
        (17) -> T17
        (18) -> T18
        (19) -> T19
        (20) -> T20
        (21) -> T21
        (22) -> T22
        (23) -> T23
        (24) -> T24
        (25) -> T25
        (26) -> T26
    }
    Tuple28 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
        (14) -> T14
        (15) -> T15
        (16) -> T16
        (17) -> T17
        (18) -> T18
        (19) -> T19
        (20) -> T20
        (21) -> T21
        (22) -> T22
        (23) -> T23
        (24) -> T24
        (25) -> T25
        (26) -> T26
        (27) -> T27
    }
    Tuple29 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
        (14) -> T14
        (15) -> T15
        (16) -> T16
        (17) -> T17
        (18) -> T18
        (19) -> T19
        (20) -> T20
        (21) -> T21
        (22) -> T22
        (23) -> T23
        (24) -> T24
        (25) -> T25
        (26) -> T26
        (27) -> T27
        (28) -> T28
    }
    Tuple30 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
        (14) -> T14
        (15) -> T15
        (16) -> T16
        (17) -> T17
        (18) -> T18
        (19) -> T19
        (20) -> T20
        (21) -> T21
        (22) -> T22
        (23) -> T23
        (24) -> T24
        (25) -> T25
        (26) -> T26
        (27) -> T27
        (28) -> T28
        (29) -> T29
    }
    Tuple31 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
        (14) -> T14
        (15) -> T15
        (16) -> T16
        (17) -> T17
        (18) -> T18
        (19) -> T19
        (20) -> T20
        (21) -> T21
        (22) -> T22
        (23) -> T23
        (24) -> T24
        (25) -> T25
        (26) -> T26
        (27) -> T27
        (28) -> T28
        (29) -> T29
        (30) -> T30
    }
    Tuple32 {
        (0) -> T0
        (1) -> T1
        (2) -> T2
        (3) -> T3
        (4) -> T4
        (5) -> T5
        (6) -> T6
        (7) -> T7
        (8) -> T8
        (9) -> T9
        (10) -> T10
        (11) -> T11
        (12) -> T12
        (13) -> T13
        (14) -> T14
        (15) -> T15
        (16) -> T16
        (17) -> T17
        (18) -> T18
        (19) -> T19
        (20) -> T20
        (21) -> T21
        (22) -> T22
        (23) -> T23
        (24) -> T24
        (25) -> T25
        (26) -> T26
        (27) -> T27
        (28) -> T28
        (29) -> T29
        (30) -> T30
        (31) -> T31
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::Level;
    use crate::SourceLocation;

    // We use `get_mock_record` and `test_pattern` in tests/pattern.rs so let's make
    // them pub in test builds.

    pub fn get_mock_record() -> Record<'static> {
        Record::builder(Level::Info, "record_payload")
            .logger_name("logger_name")
            .source_location(Some(SourceLocation::__new("module", "file", 10, 20)))
            .build()
    }

    pub fn test_pattern<P, T>(pattern: P, formatted: T, style_range: Option<Range<usize>>)
    where
        P: Pattern,
        T: AsRef<str>,
    {
        let record = get_mock_record();
        let mut output = StringBuf::new();
        let mut ctx = PatternContext::new(FmtExtraInfoBuilder::default());

        let format_result = pattern.format(&record, &mut output, &mut ctx);
        assert!(format_result.is_ok());

        assert_eq!(output.as_str(), formatted.as_ref());

        let fmt_info = ctx.fmt_info_builder.build();
        assert_eq!(fmt_info.style_range(), style_range);
    }

    #[test]
    fn test_string_as_pattern() {
        test_pattern(String::from("literal"), "literal", None);
    }

    #[test]
    fn test_str_as_pattern() {
        test_pattern("literal", "literal", None);
    }

    #[test]
    fn test_pattern_ref_as_pattern() {
        test_pattern(&String::from("literal"), "literal", None);
    }

    #[test]
    fn test_pattern_mut_as_pattern() {
        test_pattern(&mut String::from("literal"), "literal", None);
    }

    #[test]
    fn test_box_as_pattern() {
        test_pattern(Box::new(String::from("literal")), "literal", None);
    }

    #[test]
    fn test_arc_as_pattern() {
        test_pattern(Arc::new(String::from("literal")), "literal", None);
    }

    #[test]
    fn test_slice_as_pattern() {
        let pat: &[String] = &[String::from("literal1"), String::from("literal2")];
        test_pattern(pat, "literal1literal2", None);
    }

    #[test]
    fn test_empty_slice_as_pattern() {
        let pat: &[String] = &[];
        test_pattern(pat, "", None);
    }

    #[test]
    fn test_array_as_pattern() {
        let pat: [String; 3] = [
            String::from("literal1"),
            String::from("literal2"),
            String::from("literal3"),
        ];
        test_pattern(pat, "literal1literal2literal3", None);
    }

    #[test]
    fn test_empty_array_as_pattern() {
        let pat: [String; 0] = [];
        test_pattern(pat, "", None);
    }

    #[test]
    fn test_vec_as_pattern() {
        let pat = vec![
            String::from("literal1"),
            String::from("literal2"),
            String::from("literal3"),
        ];
        test_pattern(pat, "literal1literal2literal3", None);
    }

    #[test]
    fn test_empty_vec_as_pattern() {
        let pat: Vec<String> = vec![];
        test_pattern(pat, "", None);
    }

    #[test]
    fn test_tuple_as_pattern() {
        let pat = (
            String::from("literal1"),
            "literal2",
            String::from("literal3"),
        );
        test_pattern(pat, "literal1literal2literal3", None);
    }

    #[test]
    fn test_unit_as_pattern() {
        test_pattern((), "", None);
    }
}

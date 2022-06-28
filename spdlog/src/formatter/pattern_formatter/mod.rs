//! This module provides a pattern-string based formatter.
//!
//! The [`PatternFormatter`] struct defines a formatter that formats log
//! messages against a specific text pattern.
//!
//! Patterns are represented by the [`Pattern`] trait. You can create your own
//! pattern by implementing the [`Pattern`] trait.
//!
//! You can also build a pattern with the [`pattern`][pattern-macro] macro.
//!
//! [pattern-macro]: crate::pattern

pub mod patterns;

use std::{fmt::Write, ops::Range, sync::Arc};

use crate::{
    formatter::{FmtExtraInfo, FmtExtraInfoBuilder, Formatter},
    Error, Record, StringBuf,
};

#[allow(missing_docs)]
pub mod macros {
    pub use ::spdlog_macros::pattern as pattern_impl;
}

/// Build a pattern from a compile-time pattern template string.
///
/// # Basic Usage
///
/// In its simplest form, `pattern` receives a **literal** pattern string and
/// converts it into a zero-cost pattern:
///
/// ```
/// use spdlog::pattern;
/// use spdlog::formatter::PatternFormatter;
///
/// let pat = pattern!("pattern string");
/// let formatter = PatternFormatter::new(pat);
/// ```
///
/// # Using spdlog Built-in Patterns
///
/// A pattern that always outputs a fixed string is boring and useless.
/// Luckily, the pattern template string can contain placeholders that
/// represents built-in patterns. For example, to include the log level and
/// payload in the pattern, we can simply use `{l}` and `{v}` in the pattern
/// template string:
///
/// ```
/// # use spdlog::pattern;
/// # use spdlog::formatter::PatternFormatter;
/// #
/// use spdlog::info;
///
/// let pat = pattern!("[{l}] {v}");
/// let formatter = PatternFormatter::new(pat);
///
/// info!("Interesting log message");
/// // Logs: [info] Interesting log message
/// ```
///
/// Here, `{l}` and `{v}` are "placeholders" that will be replaced by the
/// output of the corresponding built-in patterns when formatting log records.
/// You can also use `{level}` and `{payload}`, if you prefer:
///
/// ```
/// # use spdlog::{info, pattern};
/// # use spdlog::formatter::PatternFormatter;
/// #
/// let pat = pattern!("[{level}] {payload}");
/// let formatter = PatternFormatter::new(pat);
///
/// info!("Interesting log message");
/// // Logs: [info] Interesting log message
/// ```
///
/// What if you want to output a literal `{` or `}` character? Simply use `{{`
/// and `}}`:
///
/// ```
/// # use spdlog::{info, pattern};
/// # use spdlog::formatter::PatternFormatter;
/// #
/// let pat = pattern!("[{{level}}] {payload}");
/// let formatter = PatternFormatter::new(pat);
///
/// info!("Interesting log message");
/// // Logs: [{level}] Interesting log message
/// ```
///
/// You can find a full list of all built-in patterns and their corresponding
/// placeholders at the end of this doc page.
///
/// # Using Style Range
///
/// A specific portion of a formatted log message can be specified as "style
/// range". Formatted text in the style range will be rendered in a different
/// style by supported sinks. You can use `{^...$}` to mark the style range
/// in the pattern template string:
///
/// ```
/// # use spdlog::{info, pattern};
/// # use spdlog::formatter::PatternFormatter;
/// #
/// let pat = pattern!("{^[{level}]$} {payload}");
/// let formatter = PatternFormatter::new(pat);
///
/// info!("Interesting log message");
/// // Logs: [info] Interesting log message
/// //       ^^^^^^ <- style range
/// ```
///
/// # Using Your Own Patterns
///
/// Yes, you can refer your own implementation of [`Pattern`] in the pattern
/// template string! Let's say you have a struct that implements the
/// [`Pattern`] trait:
///
/// ```
/// use std::fmt::Write;
/// use spdlog::{Record, StringBuf};
/// use spdlog::formatter::{Pattern, PatternContext};
///
/// #[derive(Default)]
/// struct MyPattern;
///
/// impl Pattern for MyPattern {
///     fn format(
///         &self,
///         record: &Record,
///         dest: &mut StringBuf,
///         _ctx: &mut PatternContext,
///     ) -> spdlog::Result<()> {
///         write!(dest, "My own pattern").unwrap();
///         Ok(())
///     }
/// }
/// ```
///
/// To refer `MyPattern` in the pattern template string, you need to use the
/// extended syntax to associate `MyPattern` with a name so that `pattern!`
/// can resolve it:
///
/// ```
/// # use std::fmt::Write;
/// # use spdlog::{info, pattern, Record, StringBuf};
/// # use spdlog::formatter::{Pattern, PatternContext, PatternFormatter};
/// #
/// # #[derive(Default)]
/// # struct MyPattern;
/// #
/// # impl Pattern for MyPattern {
/// #     fn format(
/// #         &self,
/// #         record: &Record,
/// #         dest: &mut StringBuf,
/// #         _ctx: &mut PatternContext,
/// #     ) -> spdlog::Result<()> {
/// #         write!(dest, "My own pattern").unwrap();
/// #         Ok(())
/// #     }
/// # }
/// #
/// let pat = pattern!("[{level}] {payload} - {mypat}",
///     {"mypat"} => MyPattern::default,
/// );
/// let formatter = PatternFormatter::new(pat);
///
/// info!("Interesting log message");
/// // Logs: [info] Interesting log message - My own pattern
/// ```
///
/// Note the special `{"name"} => id` syntax given to the `pattern` macro.
/// `name` is the name of your own pattern; placeholder `{name}` in the
/// template string will be replaced by the output of your own pattern. `name`
/// cannot contain `{` or `}`.`id` is a [path] that identifies a **function**
/// that can be called with **no arguments**. Instances of your own pattern
/// will be created by calling this function with no arguments.
///
/// [path]: https://doc.rust-lang.org/stable/reference/paths.html
///
/// ## Custom Pattern Creation
///
/// Each placeholder results in a new pattern instance. For example, consider a
/// custom pattern that writes a unique ID to the output:
///
/// ```
/// # use std::fmt::Write;
/// # use std::sync::atomic::{AtomicU32, Ordering};
/// # use spdlog::{Record, StringBuf};
/// # use spdlog::formatter::{Pattern, PatternContext};
/// #
/// static NEXT_ID: AtomicU32 = AtomicU32::new(0);
///
/// struct MyPattern {
///     id: u32,
/// }
///
/// impl MyPattern {
///     fn new() -> Self {
///         Self {
///             id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
///         }
///     }
/// }
///
/// impl Pattern for MyPattern {
///     fn format(
///         &self,
///         record: &Record,
///         dest: &mut StringBuf,
///         _ctx: &mut PatternContext,
///     ) -> spdlog::Result<()> {
///         write!(dest, "{}", self.id).unwrap();
///         Ok(())
///     }
/// }
/// ```
///
/// If the pattern template string contains multiple placeholders that refer
/// to `MyPattern`, each placeholder will eventually be replaced by different
/// IDs:
///
/// ```
/// # use std::fmt::Write;
/// # use std::sync::atomic::{AtomicU32, Ordering};
/// # use spdlog::{info, pattern, Record, StringBuf};
/// # use spdlog::formatter::{Pattern, PatternContext, PatternFormatter};
/// #
/// # static NEXT_ID: AtomicU32 = AtomicU32::new(0);
/// #
/// # struct MyPattern {
/// #     id: u32,
/// # }
/// #
/// # impl MyPattern {
/// #     fn new() -> Self {
/// #         Self {
/// #             id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
/// #         }
/// #     }
/// # }
/// #
/// # impl Pattern for MyPattern {
/// #     fn format(
/// #         &self,
/// #         record: &Record,
/// #         dest: &mut StringBuf,
/// #         _ctx: &mut PatternContext,
/// #     ) -> spdlog::Result<()> {
/// #         write!(dest, "{}", self.id).unwrap();
/// #         Ok(())
/// #     }
/// # }
/// #
/// let pat = pattern!("[{level}] {payload} - {mypat} {mypat} {mypat}",
///     {"mypat"} => MyPattern::new,
/// );
/// let formatter = PatternFormatter::new(pat);
///
/// info!("Interesting log message");
/// // Logs: [info] Interesting log message - 0 1 2
/// ```
///
/// ## Multiple Names and Multiple Custom Patterns
///
/// You can associate multiple names with your own pattern, if you prefer:
///
/// ```
/// # use std::fmt::Write;
/// # use spdlog::{info, pattern, Record, StringBuf};
/// # use spdlog::formatter::{Pattern, PatternContext, PatternFormatter};
/// #
/// # #[derive(Default)]
/// # struct MyPattern;
/// #
/// # impl Pattern for MyPattern {
/// #     fn format(
/// #         &self,
/// #         record: &Record,
/// #         dest: &mut StringBuf,
/// #         _ctx: &mut PatternContext,
/// #     ) -> spdlog::Result<()> {
/// #         write!(dest, "My own pattern").unwrap();
/// #         Ok(())
/// #     }
/// # }
/// #
/// let pat = pattern!("[{level}] {payload} - {mypat1} {mypat2}",
///     {"mypat1", "mypat2"} => MyPattern::default,
/// );
/// let formatter = PatternFormatter::new(pat);
///
/// info!("Interesting log message");
/// // Logs: [info] Interesting log message - My own pattern My own pattern
/// ```
///
/// Of course, you can have multiple custom patterns:
///
/// ```
/// # use std::fmt::Write;
/// # use spdlog::{pattern, Record, StringBuf};
/// # use spdlog::formatter::{Pattern, PatternContext, PatternFormatter};
/// #
/// # #[derive(Default)]
/// # struct MyPattern;
/// #
/// # impl Pattern for MyPattern {
/// #     fn format(
/// #         &self,
/// #         record: &Record,
/// #         dest: &mut StringBuf,
/// #         _ctx: &mut PatternContext,
/// #     ) -> spdlog::Result<()> {
/// #         write!(dest, "My own pattern").unwrap();
/// #         Ok(())
/// #     }
/// # }
/// #
/// # #[derive(Default)]
/// # struct MyOtherPattern;
/// #
/// # impl Pattern for MyOtherPattern {
/// #     fn format(
/// #         &self,
/// #         record: &Record,
/// #         dest: &mut StringBuf,
/// #         _ctx: &mut PatternContext,
/// #     ) -> spdlog::Result<()> {
/// #         write!(dest, "My own pattern").unwrap();
/// #         Ok(())
/// #     }
/// # }
/// #
/// let pat = pattern!("[{level}] {payload} - {mypat} {mypat2}",
///     {"mypat"} => MyPattern::default,
///     {"mypat2"} => MyOtherPattern::default,
/// );
/// let formatter = PatternFormatter::new(pat);
/// ```
///
/// ## Name Conflicts are Hard Errors
///
/// It's a hard error if names of your own custom pattern conflicts with other
/// patterns:
///
/// ```compile_fail
/// # use std::fmt::Write;
/// # use spdlog::{pattern, Record, StringBuf};
/// # use spdlog::formatter::{Pattern, PatternContext, PatternFormatter};
/// #
/// # #[derive(Default)]
/// # struct MyPattern;
/// #
/// # impl Pattern for MyPattern {
/// #     fn format(
/// #         &self,
/// #         record: &Record,
/// #         dest: &mut StringBuf,
/// #         _ctx: &mut PatternContext,
/// #     ) -> spdlog::Result<()> {
/// #         write!(dest, "My own pattern").unwrap();
/// #         Ok(())
/// #     }
/// # }
/// #
/// # #[derive(Default)]
/// # struct MyOtherPattern;
/// #
/// # impl Pattern for MyOtherPattern {
/// #     fn format(
/// #         &self,
/// #         record: &Record,
/// #         dest: &mut StringBuf,
/// #         _ctx: &mut PatternContext,
/// #     ) -> spdlog::Result<()> {
/// #         write!(dest, "My own pattern").unwrap();
/// #         Ok(())
/// #     }
/// # }
/// #
/// # #[derive(Default)]
/// # struct MyOtherPattern2;
/// #
/// # impl Pattern for MyOtherPattern2 {
/// #     fn format(
/// #         &self,
/// #         record: &Record,
/// #         dest: &mut StringBuf,
/// #         _ctx: &mut PatternContext,
/// #     ) -> spdlog::Result<()> {
/// #         write!(dest, "My own pattern").unwrap();
/// #         Ok(())
/// #     }
/// # }
/// #
/// let pat = pattern!("[{level}] {payload} - {mypat}",
///     {"mypat"} => MyPattern::default,
///
///     // Error: name conflicts with another custom pattern
///     {"mypat"} => MyOtherPattern::default,
///
///     // Error: name conflicts with a built-in pattern
///     {"n"} => MyOtherPattern2::default,
/// );
/// ```
///
/// # Appendix: A Full List of Built-in Patterns and Their Placeholders
///
/// | Placeholders | Description | Example |
/// | --- | --- | --- |
/// | `{a}`, `{weekday-name}` | Abbreviated weekday name | `Mon`, `Tue` |
/// | `{A}`, `{weekday-name-full}` | Weekday name | `Monday`, `Tuesday` |
/// | `{b}`, `{month-name}` | Abbreviated month name | `Jan`, `Feb` |
/// | `{B}`, `{month-name-full}` | Month name | `January`, `February` |
/// | `{c}`, `{datetime}` | Full date time | `Thu Aug 23 15:35:46 2014` |
/// | `{C}`, `{year-short}` | Short year | `22`, `20` |
/// | `{Y}`, `{year}` | Year | `2022`, `2021` |
/// | `{D}`, `{date-short}` | Short date | `04/01/22`, `12/31/21` |
/// | `{m}`, `{month}` | Month | `01`, `12` |
/// | `{d}`, `{day}` | Day in month | `01`, `12`, `31`, `30` |
/// | `{H}`, `{hour}` | Hour in 24-hour | `01`, `12`, `23` |
/// | `{I}`, `{hour-12}` | Hour in 12-hour | `01`, `12` |
/// | `{M}`, `{minute}` | Minute | `00`, `05`, `59` |
/// | `{S}`, `{second}` | Second | `00`, `05`, `59` |
/// | `{e}`, `{millisecond}` | Millisecond | `231` |
/// | `{f}`, `{microsecond}` | Microseconds within a second | `372152` |
/// | `{F}`, `{nanosecond}` | Nanoseconds within a second | `482930154` |
/// | `{p}`, `{ampm}` | AM / PM | `AM`, `PM` |
/// | `{r}`, `{time-12}` | Time in 12-hour format | `02:55:02 PM` |
/// | `{R}`, `{time-short}` | Short time | `22:28`, `09:53` |
/// | `{T}`, `{X}`, `{time}` | Time | `22:28:02`, `09:53:41` |
/// | `{z}`, `{tz-offset}` | Timezone offset | `+08:00`, `+00:00`, `-06:00` |
/// | `{E}`, `{unix-timestamp}` | Unix timestamp | `1528834770` |
/// | `{+}`, `{full}` | Full log message | See [`FullFormatter`] |
/// | `{l}`, `{level}` | Log level | `critical`, `error`, `warn` |
/// | `{L}`, `{level-short}` | Short log level | `C`, `E`, `W` |
/// | `{@}`, `{loc}` | Log location | `main.rs:30:20` |
/// | `{s}`, `{source-basename}` | Source file basename | `main.rs` |
/// | `{g}`, `{source}` | Path to the source file | `src/main.rs` |
/// | `{#}`, `{line}` | Source file line | `30` |
/// | `{%}`, `{column}` | Source file column | `20` |
/// | `{n}`, `{logger}` | Logger name | `my-logger` |
/// | `{v}`, `{payload}` | Log payload | `log message` |
/// | `{P}`, `{pid}` | Process ID | `3824` |
/// | `{t}`, `{tid}` | Thread ID | `3132` |
///
/// [`FullFormatter`]: crate::formatter::FullFormatter
#[macro_export]
macro_rules! pattern {
    ( $($t:tt)* ) => {
        $crate::formatter::macros::pattern_impl!($($t)*)
    }
}

/// Build a [`PatternFormatter`] from a pattern built by the [`pattern`] macro
/// with the given macro arguments.
///
/// `pattern_formatter!(...)` is equivalent to
/// `PatternFormatter::new(pattern!(...))`.
///
/// ```
/// # use spdlog::pattern_formatter;
/// # use spdlog::formatter::PatternFormatter;
/// #
/// let formatter: PatternFormatter<_> = pattern_formatter!("{n}: {^[{level}]$} {v}");
/// ```
#[macro_export]
macro_rules! pattern_formatter {
    ( $($t:tt)* ) => {
        $crate::formatter::PatternFormatter::new($crate::pattern!($($t)*))
    };
}

/// A formatter that formats log records according to a specified pattern.
pub struct PatternFormatter<P> {
    pattern: P,
}

impl<P> PatternFormatter<P> {
    /// Create a new `PatternFormatter` object with the given pattern.
    ///
    /// Manually craft a pattern object `pattern` can be tedious and
    /// error-prone. It's recommended to use the [`pattern!`] macro to create
    /// a pattern object from a template string.
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
/// in various ways to create a new pattern. The [`PatternFormatter`] struct
/// provides a [`Formatter`] that formats log records according to a given
/// pattern.
///
/// # Built-in Patterns
///
/// `spdlog` provides a rich set of built-in patterns. See the [`patterns`]
/// module.
///
/// # Custom Patterns
///
/// There are 2 approaches to create your own pattern:
/// - Define a new type and implements this trait;
/// - Use the [`pattern`] macro to create a pattern from a template string.
pub trait Pattern: Send + Sync {
    /// Format this pattern against the given log record and write the formatted
    /// message into the output buffer.
    ///
    /// **For implementors:** the `ctx` parameter is reserved for internal use.
    /// You should not use it.
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

impl Pattern for str {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(self).map_err(Error::FormatRecord)?;
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

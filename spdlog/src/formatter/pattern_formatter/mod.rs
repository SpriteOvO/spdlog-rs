#[doc(hidden)]
#[path = "pattern/mod.rs"]
pub mod __pattern;

#[cfg(feature = "runtime-pattern")]
mod runtime;

use std::{fmt::Write, sync::Arc};

use dyn_clone::*;
#[cfg(feature = "runtime-pattern")]
pub use runtime::*;

use crate::{
    formatter::{Formatter, FormatterContext, TimeDate, TimeDateLazyLocked},
    Error, Record, StringBuf,
};

#[rustfmt::skip] // rustfmt currently breaks some empty lines if `#[doc = include_str!("xxx")]` exists
/// Builds a pattern from a template literal string at compile-time.
///
/// It accepts inputs in the form:
///
/// ```ignore
/// // This is not exactly a valid declarative macro, just for intuition.
/// macro_rules! pattern {
///     ( $template:literal $(,)? ) => {};
///     ( $template:literal, $( {$$custom:ident} => $ctor:expr ),+ $(,)? ) => {};
/// }
/// ```
///
/// Examples of valid inputs:
///
/// ```
/// # use spdlog::formatter::pattern;
/// # #[derive(Default)]
/// # struct MyPattern;
/// pattern!("text");
/// pattern!("current line: {line}{eol}");
/// pattern!("custom: {$my_pattern}{eol}", {$my_pattern} => MyPattern::default);
/// ```
///
/// Its first argument accepts only a literal string that is known at compile-time.
/// If you want to build a pattern from a runtime string, use
/// [`runtime_pattern!`] macro instead.
///
/// # Note
///
/// The value returned by this macro is implementation details and users should
/// not access them. If these details are changed in the future, it may not be
/// considered as a breaking change.
///
/// # Basic Usage
///
/// In its simplest form, `pattern` receives a **literal** template string and
/// converts it into a zero-cost pattern:
/// ```
/// use spdlog::formatter::{pattern, PatternFormatter};
///
/// let formatter = PatternFormatter::new(pattern!("template string"));
/// ```
///
/// # Using Built-in Patterns
///
/// A pattern that always outputs a fixed string is boring and useless.
/// Luckily, the pattern template string can contain placeholders that
/// represents built-in patterns. For example, to include the log level and
/// payload in the pattern, we can simply use `{level}` and `{payload}` in the
/// pattern template string:
/// ```
/// # use spdlog::formatter::{pattern, PatternFormatter};
/// use spdlog::info;
#[doc = include_str!(concat!(env!("OUT_DIR"), "/test_utils/common_for_doc_test.rs"))]
///
/// let formatter = PatternFormatter::new(pattern!("[{level}] {payload}{eol}"));
/// # let (doctest, sink) = test_utils::echo_logger_from_formatter(
/// #     Box::new(formatter),
/// #     None
/// # );
///
/// info!(logger: doctest, "Interesting log message");
/// # assert_eq!(
/// #     sink.clone_string().replace("\r", ""),
/// /* Output */ "[info] Interesting log message\n"
/// # );
/// ```
///
/// Here, `{level}` and `{payload}` are "placeholders" that will be replaced by
/// the output of the corresponding built-in patterns when formatting log
/// records.
///
/// What if you want to output a literal `{` or `}` character? Simply use `{{`
/// and `}}`:
/// ```
/// # use spdlog::{
/// #     formatter::{pattern, PatternFormatter},
/// #     info,
/// # };
#[doc = include_str!(concat!(env!("OUT_DIR"), "/test_utils/common_for_doc_test.rs"))]
/// let formatter = PatternFormatter::new(pattern!("[{{escaped}}] {payload}{eol}"));
/// # let (doctest, sink) = test_utils::echo_logger_from_formatter(
/// #     Box::new(formatter),
/// #     None
/// # );
///
/// info!(logger: doctest, "Interesting log message");
/// # assert_eq!(
/// #     sink.clone_string().replace("\r", ""),
/// /* Output */ "[{escaped}] Interesting log message\n"
/// # );
/// ```
///
/// You can find a full list of all built-in patterns and their corresponding
/// placeholders at [Appendix](#appendix-a-full-list-of-built-in-patterns)
/// below.
///
/// # Using Style Range
///
/// A specific portion of a formatted log message can be specified as "style
/// range". Formatted text in the style range will be rendered in a different
/// style by supported sinks. You can use `{^...}` to mark the style range in
/// the pattern template string:
/// ```
/// # use spdlog::{
/// #     formatter::{pattern, PatternFormatter},
/// #     info,
/// # };
#[doc = include_str!(concat!(env!("OUT_DIR"), "/test_utils/common_for_doc_test.rs"))]
/// let formatter = PatternFormatter::new(pattern!("{^[{level}]} {payload}{eol}"));
/// # let (doctest, sink) = test_utils::echo_logger_from_formatter(
/// #     Box::new(formatter),
/// #     None
/// # );
///
/// info!(logger: doctest, "Interesting log message");
/// # assert_eq!(
/// #     sink.clone_string().replace("\r", ""),
/// /* Output */ "[info] Interesting log message\n"
/// //            ^^^^^^ <- style range
/// # );
/// ```
/// 
/// # Using Your Own Patterns
///
/// Yes, you can refer your own implementation of [`Pattern`] in the pattern
/// template string! Let's say you have a struct that implements the
/// [`Pattern`] trait. To refer `MyPattern` in the pattern template string, you
/// need to use the extended syntax to associate `MyPattern` with a name so
/// that `pattern!` can resolve it:
/// ```
/// use std::fmt::Write;
///
/// use spdlog::{
///     formatter::{pattern, Pattern, PatternContext, PatternFormatter},
///     Record, StringBuf, info
/// };
#[doc = include_str!(concat!(env!("OUT_DIR"), "/test_utils/common_for_doc_test.rs"))]
///
/// #[derive(Default, Clone)]
/// struct MyPattern;
///
/// impl Pattern for MyPattern {
///     fn format(&self, record: &Record, dest: &mut StringBuf, _: &mut PatternContext) -> spdlog::Result<()> {
///         write!(dest, "My own pattern").map_err(spdlog::Error::FormatRecord)
///     }
/// }
///
/// let pat = pattern!("[{level}] {payload} - {$mypat}{eol}",
///     {$mypat} => MyPattern::default,
/// );
/// let formatter = PatternFormatter::new(pat);
/// # let (doctest, sink) = test_utils::echo_logger_from_formatter(
/// #     Box::new(formatter),
/// #     None
/// # );
///
/// info!(logger: doctest, "Interesting log message");
/// # assert_eq!(
/// #   sink.clone_string().replace("\r", ""),
/// /* Output */ "[info] Interesting log message - My own pattern\n"
/// # );
/// ```
///
/// Note the special `{$name} => id` syntax given to the `pattern` macro.
/// `name` is the name of your own pattern; placeholder `{$name}` in the
/// template string will be replaced by the output of your own pattern. `name`
/// can only be an identifier. `id` is a [path] that identifies a **function**
/// that can be called with **no arguments**. Instances of your own pattern
/// will be created by calling this function with no arguments.
///
/// [path]: https://doc.rust-lang.org/stable/reference/paths.html
///
/// ## Custom Pattern Creation
///
/// Each placeholder results in a new pattern instance. For example, consider a
/// custom pattern that writes a unique ID to the output. If the pattern
/// template string contains multiple placeholders that refer to `MyPattern`,
/// each placeholder will eventually be replaced by different IDs.
///
/// ```
/// # use std::{
/// #     fmt::Write,
/// #     sync::atomic::{AtomicU32, Ordering},
/// # };
/// # use spdlog::{
/// #     formatter::{pattern, Pattern, PatternContext, PatternFormatter},
/// #     prelude::*,
/// #     Record, StringBuf,
/// # };
#[doc = include_str!(concat!(env!("OUT_DIR"), "/test_utils/common_for_doc_test.rs"))]
/// static NEXT_ID: AtomicU32 = AtomicU32::new(0);
///
/// #[derive(Clone)]
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
///     fn format(&self, record: &Record, dest: &mut StringBuf, _: &mut PatternContext) -> spdlog::Result<()> {
///         write!(dest, "{}", self.id).map_err(spdlog::Error::FormatRecord)
///     }
/// }
///
/// let pat = pattern!("[{level}] {payload} - {$mypat} {$mypat} {$mypat}{eol}",
///     {$mypat} => MyPattern::new,
/// );
/// let formatter = PatternFormatter::new(pat);
/// # let (doctest, sink) = test_utils::echo_logger_from_formatter(
/// #     Box::new(formatter),
/// #     None
/// # );
///
/// info!(logger: doctest, "Interesting log message");
/// # assert_eq!(
/// #   sink.clone_string().replace("\r", ""),
/// /* Output */ "[info] Interesting log message - 0 1 2\n"
/// # );
/// ```
///
/// Of course, you can have multiple custom patterns:
/// ```
/// # use spdlog::formatter::pattern;
/// #
/// # #[derive(Default)]
/// # struct MyPattern;
/// # #[derive(Default)]
/// # struct MyOtherPattern;
/// #
/// let pat = pattern!("[{level}] {payload} - {$mypat} {$myotherpat}{eol}",
///     {$mypat} => MyPattern::default,
///     {$myotherpat} => MyOtherPattern::default,
/// );
/// ```
///
/// ## Name Conflicts are Hard Errors
///
/// It's a hard error if names of your own custom pattern conflicts with other
/// patterns:
///
/// ```compile_fail
/// # use spdlog::formatter::pattern;
/// #
/// # #[derive(Default)]
/// # struct MyPattern;
/// # #[derive(Default)]
/// # struct MyOtherPattern;
/// #
/// let pattern = pattern!("[{level}] {payload} - {$mypat}{eol}",
///     {$mypat} => MyPattern::new,
///     // Error: name conflicts with another custom pattern
///     {$mypat} => MyOtherPattern::new,
/// );
/// ```
///
/// ```compile_fail
/// # use spdlog::formatter::pattern;
/// #
/// # #[derive(Default)]
/// # struct MyPattern;
/// #
/// let pattern = pattern!("[{level}] {payload} - {$day}{eol}",
///     // Error: name conflicts with a built-in pattern
///     {$day} => MyPattern::new,
/// );
/// ```
///
/// # Appendix: Full List of Built-in Patterns
///
/// | Placeholders          | Description                  | Example                                      |
/// | --------------------- | ---------------------------- | -------------------------------------------- |
/// | `{weekday_name}`      | Abbreviated weekday name     | `Mon`, `Tue`                                 |
/// | `{weekday_name_full}` | Weekday name                 | `Monday`, `Tuesday`                          |
/// | `{month_name}`        | Abbreviated month name       | `Jan`, `Feb`                                 |
/// | `{month_name_full}`   | Month name                   | `January`, `February`                        |
/// | `{datetime}`          | Full date time               | `Thu Aug 23 15:35:46 2014`                   |
/// | `{year_short}`        | Short year                   | `22`, `20`                                   |
/// | `{year}`              | Year                         | `2022`, `2021`                               |
/// | `{date_short}`        | Short date                   | `04/01/22`, `12/31/21`                       |
/// | `{date}`              | Date (ISO 8601)              | `2022-04-01`, `2021-12-31`                   |
/// | `{month}`             | Month                        | `01`, `12`                                   |
/// | `{day}`               | Day in month                 | `01`, `12`, `31`, `30`                       |
/// | `{hour}`              | Hour in 24-hour              | `01`, `12`, `23`                             |
/// | `{hour_12}`           | Hour in 12-hour              | `01`, `12`                                   |
/// | `{minute}`            | Minute                       | `00`, `05`, `59`                             |
/// | `{second}`            | Second                       | `00`, `05`, `59`                             |
/// | `{millisecond}`       | Millisecond                  | `231`                                        |
/// | `{microsecond}`       | Microseconds within a second | `372152`                                     |
/// | `{nanosecond}`        | Nanoseconds within a second  | `482930154`                                  |
/// | `{am_pm}`             | AM / PM                      | `AM`, `PM`                                   |
/// | `{time_12}`           | Time in 12-hour format       | `02:55:02 PM`                                |
/// | `{time_short}`        | Short time                   | `22:28`, `09:53`                             |
/// | `{time}`              | Time                         | `22:28:02`, `09:53:41`                       |
/// | `{tz_offset}`         | Timezone offset              | `+08:00`, `+00:00`, `-06:00`                 |
/// | `{unix_timestamp}`    | Unix timestamp               | `1528834770`                                 |
/// | `{full}`              | Full log message             | See [`FullFormatter`]                        |
/// | `{level}`             | Log level                    | `critical`, `error`, `warn`                  |
/// | `{level_short}`       | Short log level              | `C`, `E`, `W`                                |
/// | `{source}`            | Source file and line         | `path/to/main.rs:30` [^1]                    |
/// | `{file_name}`         | Source file name             | `main.rs` [^1]                               |
/// | `{file}`              | Source file path             | `path/to/main.rs` [^1]                       |
/// | `{line}`              | Source file line             | `30` [^1]                                    |
/// | `{column}`            | Source file column           | `20` [^1]                                    |
/// | `{module_path}`       | Source module path           | `mod::module` [^1]                           |
/// | `{logger}`            | Logger name                  | `my-logger`                                  |
/// | `{payload}`           | Log payload                  | `log message`                                |
/// | `{pid}`               | Process ID                   | `3824`                                       |
/// | `{tid}`               | Thread ID                    | `3132`                                       |
/// | `{eol}`               | End of line                  | `\n` (on non-Windows) or `\r\n` (on Windows) |
/// 
/// [^1]: Patterns related to source location require that feature
///       `source-location` is enabled, otherwise the output is empty.
///
/// [`runtime_pattern!`]: crate::formatter::runtime_pattern
/// [`FullFormatter`]: crate::formatter::FullFormatter
pub use ::spdlog_macros::pattern;

// Emit a compile error if the feature is not enabled.
#[cfg(not(feature = "runtime-pattern"))]
pub use crate::__runtime_pattern_disabled as runtime_pattern;

#[cfg(not(feature = "runtime-pattern"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __runtime_pattern_disabled {
    ($($_:tt)*) => {
        compile_error!(
            "macro `runtime_pattern` required to enable crate feature `runtime-pattern` for spdlog-rs"
        );
    };
}

/// Formats logs according to a specified pattern.
#[derive(Clone)]
pub struct PatternFormatter<P> {
    pattern: P,
}

impl<P> PatternFormatter<P>
where
    P: Pattern,
{
    /// Creates a new `PatternFormatter` object with the given pattern.
    ///
    /// Currently users can only create a `pattern` object by using:
    ///
    /// - Macro [`pattern!`] to build a pattern with a literal template string
    ///   at compile-time.
    /// - Macro [`runtime_pattern!`] to build a pattern at runtime.
    #[must_use]
    pub fn new(pattern: P) -> Self {
        Self { pattern }
    }
}

impl<P> Formatter for PatternFormatter<P>
where
    P: 'static + Clone + Pattern,
{
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        fmt_ctx: &mut FormatterContext,
    ) -> crate::Result<()> {
        cfg_if::cfg_if! {
            if #[cfg(not(feature = "flexible-string"))] {
                dest.reserve(crate::string_buf::RESERVE_SIZE);
            }
        };

        fmt_ctx.locked_time_date = Some(TimeDateLazyLocked::new(record.time()));
        {
            let mut pat_ctx = PatternContext { fmt_ctx };
            self.pattern.format(record, dest, &mut pat_ctx)?;
        }
        fmt_ctx.locked_time_date = None;
        Ok(())
    }
}

/// Provides context for patterns.
///
/// There is nothing to set up here at the moment, reserved for future use.
#[derive(Debug)]
pub struct PatternContext<'a, 'b> {
    fmt_ctx: &'a mut FormatterContext<'b>,
}

impl PatternContext<'_, '_> {
    #[must_use]
    fn time_date(&mut self) -> TimeDate<'_> {
        self.fmt_ctx.locked_time_date.as_mut().unwrap().get()
    }
}

/// Represents a pattern for replacing a placeholder in templates.
///
/// A pattern will be used to replace placeholders that appear in a template
/// string. Multiple patterns can form a new pattern. The [`PatternFormatter`]
/// formats logs according to a given pattern.
///
/// # Built-in Patterns
///
/// `spdlog-rs` provides a rich set of built-in patterns. See the [`pattern`]
/// macro.
///
/// # Custom Patterns
///
/// There are 3 approaches to create your own pattern:
/// - Define a new type and implement this trait;
/// - Use [`pattern`] macro to create a pattern from a literal template string.
/// - Use [`runtime_pattern`] macro to create a pattern from a runtime template
///   string.
pub trait Pattern: Send + Sync + DynClone {
    /// Format this pattern against the given log record and write the formatted
    /// message into the output buffer.
    ///
    /// **For implementors:** the `ctx` parameter is reserved for future use.
    /// For now, please ignore it.
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()>;
}
clone_trait_object!(Pattern);

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
        dest.write_str(self).map_err(Error::FormatRecord)
    }
}

impl<T> Pattern for &T
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
    Self: Clone,
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
    Self: Clone,
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

impl<T> Pattern for &[T]
where
    T: Pattern,
    Self: Clone,
{
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        for p in *self {
            <T as Pattern>::format(p, record, dest, ctx)?;
        }
        Ok(())
    }
}

impl<T, const N: usize> Pattern for [T; N]
where
    T: Pattern,
    Self: Clone,
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

impl<T> Pattern for Vec<T>
where
    T: Pattern,
    Self: Clone,
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
                Self: Clone,
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
mod tests {
    use std::ops::Range;

    use super::*;
    use crate::{Level, SourceLocation};

    // We use `get_mock_record` and `test_pattern` in tests/pattern.rs so let's make
    // them pub in test builds.

    #[must_use]
    fn get_mock_record() -> Record<'static> {
        Record::new(
            Level::Info,
            "record_payload",
            Some(SourceLocation::__new("module", "file", 10, 20)),
            Some("logger_name"),
        )
    }

    fn test_pattern<P, T>(pattern: P, formatted: T, style_range: Option<Range<usize>>)
    where
        P: Pattern,
        T: AsRef<str>,
    {
        let record = get_mock_record();
        let mut output = StringBuf::new();
        let mut fmt_ctx = FormatterContext::new();
        fmt_ctx.locked_time_date = Some(TimeDateLazyLocked::new(record.time()));
        let mut pat_ctx = PatternContext {
            fmt_ctx: &mut fmt_ctx,
        };
        pattern.format(&record, &mut output, &mut pat_ctx).unwrap();
        fmt_ctx.locked_time_date = None;

        assert_eq!(output.as_str(), formatted.as_ref());
        assert_eq!(fmt_ctx.style_range(), style_range);
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
        #[allow(unknown_lints)]
        #[allow(clippy::needless_borrow, clippy::needless_borrows_for_generic_args)]
        test_pattern(&String::from("literal"), "literal", None);
    }

    #[test]
    fn test_pattern_mut_as_pattern() {
        // Since we now require `T: Pattern` to implement `Clone`, there is no way to
        // accept an `&mut T` as a `Pattern` anymore, since `&mut T` is not cloneable.
        //
        // test_pattern(&mut String::from("literal"), "literal", None);
        #[allow(clippy::deref_addrof)]
        test_pattern(&*&mut String::from("literal"), "literal", None);
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

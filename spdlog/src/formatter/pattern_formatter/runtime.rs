use spdlog_internal::pattern_parser::{
    error::TemplateError,
    parse::{Template, TemplateToken},
    BuiltInFormatter, BuiltInFormatterInner, Error as PatternParserError,
    PatternKind as GenericPatternKind, PatternRegistry as GenericPatternRegistry,
    Result as PatternParserResult,
};

use super::{Pattern, PatternContext, __pattern as pattern};
use crate::{
    error::{BuildPatternError, Error},
    Record, Result, StringBuf,
};

type Patterns = Vec<Box<dyn Pattern>>;
type PatternCreator = Box<dyn Fn() -> Box<dyn Pattern>>;
type PatternRegistry = GenericPatternRegistry<PatternCreator>;
type PatternKind = GenericPatternKind<PatternCreator>;

/// Build a pattern from a template string at runtime.
///
/// It accepts inputs in the form:
///
/// ```ignore
/// // This is not exactly a valid declarative macro, just for intuition.
/// macro_rules! runtime_pattern {
///     ( $template:expr $(,)? ) => {};
///     ( $template:expr, $( {$$custom:ident} => $ctor:expr ),+ $(,)? ) => {};
/// }
/// ```
///
/// The only difference between `runtime_pattern!` macro and [`pattern!`] macro
/// is that [`pattern!`] macro only accepts a string literal as the pattern
/// template, while `runtime_pattern!` macro accepts an expression that can be
/// evaluated to the pattern template string at runtime.
///
/// The returen type of `runtime_pattern!` macro is
/// `Result<RuntimePattern, spdlog::Error>`. An error will be returned when
/// parsing of the template string fails. If any of the custom patterns given
/// are invalid, a compilation error will be triggered.
///
/// For the input formats and more usages, please refer to [`pattern!`] macro.
///
/// # Example
///
/// ```
/// use spdlog::formatter::{runtime_pattern, PatternFormatter};
///
/// # type MyPattern = spdlog::formatter::__pattern::Level;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let template = String::from("[{level}] {payload} - {$mypat}{eol}");
/// let pat = runtime_pattern!(&template, {$mypat} => MyPattern::default)?;
/// let formatter = PatternFormatter::new(pat);
/// # Ok(()) }
/// ```
///
/// [`pattern!`]: crate::formatter::pattern
pub use spdlog_macros::runtime_pattern;

#[rustfmt::skip] // rustfmt currently breaks some empty lines if `#[doc = include_str!("xxx")]` exists
/// A runtime pattern built via [`runtime_pattern!`] macro.
///
/// ## Basic Usage
/// 
/// ```
/// # use spdlog::formatter::{runtime_pattern, PatternFormatter};
/// use spdlog::info;
///
/// #
#[doc = include_str!(concat!(env!("OUT_DIR"), "/test_utils/common_for_doc_test.rs"))]
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let formatter = PatternFormatter::new(runtime_pattern!("[{level}] {payload}{eol}")?);
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
/// # Ok(()) }
/// ```
/// 
/// ## With Custom Patterns
/// 
/// ```
/// use std::fmt::Write;
///
/// use spdlog::{
///     formatter::{pattern, Pattern, PatternContext, PatternFormatter, runtime_pattern, RuntimePattern},
///     Record, StringBuf, info
/// };
/// 
/// #[derive(Default, Clone)]
/// struct MyPattern;
/// 
/// impl Pattern for MyPattern {
///    fn format(&self, record: &Record, dest: &mut StringBuf, _: &mut PatternContext) -> spdlog::Result<()> {
///        write!(dest, "My own pattern").map_err(spdlog::Error::FormatRecord)
///    }
/// }
///
#[doc = include_str!(concat!(env!("OUT_DIR"), "/test_utils/common_for_doc_test.rs"))]
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let template = "[{level}] {payload} - {$mypat1} {$mypat2}{eol}";
/// # // TODO: Directly pass the closure to runtime_pattern! macro
/// fn pat() -> impl Pattern { pattern!("[{level_short}-{level}]") }
/// let formatter = PatternFormatter::new(
///     runtime_pattern!(
///         template,
///         {$mypat1} => MyPattern::default,
///         {$mypat2} => pat
///     )?
/// );
/// # let (doctest, sink) = test_utils::echo_logger_from_formatter(
/// #     Box::new(formatter),
/// #     None
/// # );
///
/// info!(logger: doctest, "Interesting log message");
/// # assert_eq!(
/// #     sink.clone_string().replace("\r", ""),
/// /* Output */ "[info] Interesting log message - My own pattern [I-info]\n"
/// # );
/// # Ok(()) }
/// ```
/// 
/// [`pattern!`]: crate::formatter::pattern
#[derive(Clone)]
pub struct RuntimePattern(Patterns);

impl RuntimePattern {
    // Private function, do not use in your code directly.
    #[doc(hidden)]
    pub fn __with_custom_patterns(template: &str, registry: PatternRegistry) -> Result<Self> {
        Template::parse(template)
            .and_then(|template| {
                Synthesiser::new(registry)
                    .synthesize(template)
                    .map(RuntimePattern)
            })
            .map_err(|err| Error::BuildPattern(BuildPatternError(err)))
    }
}

impl Pattern for RuntimePattern {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> Result<()> {
        for pattern in &self.0 {
            pattern.format(record, dest, ctx)?;
        }
        Ok(())
    }
}

struct Synthesiser {
    registry: PatternRegistry,
}

impl Synthesiser {
    fn new(registry: PatternRegistry) -> Self {
        Self { registry }
    }

    fn synthesize(&self, template: Template) -> PatternParserResult<Patterns> {
        self.build_patterns(template, false)
    }

    fn build_patterns(
        &self,
        template: Template,
        mut style_range_seen: bool,
    ) -> PatternParserResult<Patterns> {
        let mut patterns = Patterns::new();

        for token in template.tokens {
            let pattern = match token {
                TemplateToken::Literal(t) => Box::new(t.literal),
                TemplateToken::Formatter(t) => {
                    let pattern = self.registry.find(t.has_custom_prefix, t.placeholder)?;
                    match pattern {
                        PatternKind::BuiltIn(builtin) => build_builtin_pattern(builtin),
                        PatternKind::Custom { factory, .. } => factory(),
                    }
                }
                TemplateToken::StyleRange(style_range) => {
                    if style_range_seen {
                        return Err(PatternParserError::Template(
                            TemplateError::MultipleStyleRange,
                        ));
                    }
                    style_range_seen = true;
                    Box::new(pattern::StyleRange::new(
                        self.build_patterns(style_range.body, true)?,
                    ))
                }
            };
            patterns.push(pattern);
        }

        Ok(patterns)
    }
}

fn build_builtin_pattern(builtin: &BuiltInFormatter) -> Box<dyn Pattern> {
    macro_rules! match_builtin {
        (  $($name:ident),+ $(,)? ) => {
            match builtin.inner() {
                $(BuiltInFormatterInner::$name => Box::<pattern::$name>::default()),+
            }
        };
    }

    match_builtin!(
        AbbrWeekdayName,
        WeekdayName,
        AbbrMonthName,
        MonthName,
        FullDateTime,
        ShortYear,
        Year,
        ShortDate,
        Date,
        Month,
        Day,
        Hour,
        Hour12,
        Minute,
        Second,
        Millisecond,
        Microsecond,
        Nanosecond,
        AmPm,
        Time12,
        ShortTime,
        Time,
        TzOffset,
        UnixTimestamp,
        Full,
        Level,
        ShortLevel,
        Source,
        SourceFilename,
        SourceFile,
        SourceLine,
        SourceColumn,
        SourceModulePath,
        LoggerName,
        Payload,
        ProcessId,
        ThreadId,
        Eol
    )
}

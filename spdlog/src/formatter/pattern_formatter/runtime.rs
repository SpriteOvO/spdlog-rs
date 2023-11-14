use std::convert::Infallible;

use serde::Deserialize;
use spdlog_internal::pattern_parser::{
    error::TemplateError,
    parse::{Template, TemplateToken},
    BuiltInFormatter, BuiltInFormatterInner, Error as PatternParserError,
    PatternKind as GenericPatternKind, PatternRegistry as GenericPatternRegistry,
    Result as PatternParserResult,
};

use super::{Pattern, PatternContext, PatternFormatter, __pattern as pattern};
use crate::{
    config::{ComponentMetadata, Configurable},
    error::{BuildPatternError, BuildPatternErrorInner, Error},
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

#[derive(Default, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct PatternFormatterRuntimePatternParams {
    template: String,
}

#[rustfmt::skip] // rustfmt currently breaks some empty lines if `#[doc = include_str!("xxx")]` exists
/// The builder of [`RuntimePattern`].
#[doc = include_str!("../../include/doc/generic-builder-note.md")]
///
/// # Example
/// 
/// See the documentation of [`RuntimePattern`].
pub struct RuntimePatternBuilder<ArgT> {
    template: ArgT,
    custom_patterns: Vec<(String, PatternCreator)>,
}

impl<ArgT> RuntimePatternBuilder<ArgT> {
    /// Specifies the template string.
    ///
    /// This parameter is **required**.
    ///
    /// About the template string format, please see the documentation of
    /// [`pattern!`] macro.
    ///
    /// [`pattern!`]: crate::formatter::pattern
    pub fn template<S>(self, template: S) -> RuntimePatternBuilder<String>
    where
        S: Into<String>,
    {
        RuntimePatternBuilder {
            template: template.into(),
            custom_patterns: self.custom_patterns,
        }
    }

    /// Specifies a creator for a custom pattern that appears in the template
    /// string.
    ///
    /// This parameter is **optional** if there is no reference to a custom
    /// pattern in the template string, otherwise it's **required**.
    ///
    /// It is conceptually equivalent to `{$my_pat} => MyPattern::new` in
    /// [`pattern!`] macro.
    ///
    /// The placeholder argument must be an identifier, e.g. `"my_pat"`,
    /// `"_my_pat"`, etc., it cannot be `"2my_pat"`, `"r#my_pat"`, `"3"`, etc.
    ///
    /// [`pattern!`]: crate::formatter::pattern
    pub fn custom_pattern<S, P, F>(mut self, placeholder: S, pattern_creator: F) -> Self
    where
        S: Into<String>,
        P: Pattern + 'static,
        F: Fn() -> P + 'static,
    {
        self.custom_patterns.push((
            placeholder.into(),
            Box::new(move || Box::new(pattern_creator())),
        ));
        self
    }
}

impl RuntimePatternBuilder<()> {
    #[doc(hidden)]
    #[deprecated(note = "\n\n\
        builder compile-time error:\n\
        - missing required field `template`\n\n\
    ")]
    pub fn build(self, _: Infallible) {}
}

impl RuntimePatternBuilder<String> {
    /// Builds a runtime pattern.
    pub fn build(self) -> Result<RuntimePattern> {
        self.build_inner()
    }

    fn build_inner(self) -> Result<RuntimePattern> {
        let mut registry = PatternRegistry::with_builtin();
        for (name, formatter) in self.custom_patterns {
            if !(!name.is_empty()
                && name
                    .chars()
                    .next()
                    .map(|ch| ch.is_ascii_alphabetic() || ch == '_')
                    .unwrap()
                && name
                    .chars()
                    .skip(1)
                    .all(|ch| ch.is_ascii_alphanumeric() || ch == '_'))
            {
                return Err(Error::err_build_pattern(
                    BuildPatternErrorInner::InvalidCustomPlaceholder(name),
                ));
            }
            registry
                .register_custom(name, formatter)
                .map_err(Error::err_build_pattern_internal)?;
        }

        let template =
            Template::parse(&self.template).map_err(Error::err_build_pattern_internal)?;

        Synthesiser::new(registry)
            .synthesize(template)
            .map_err(Error::err_build_pattern_internal)
            .map(RuntimePattern)
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

impl Configurable for PatternFormatter<RuntimePattern> {
    type Params = PatternFormatterRuntimePatternParams;

    fn metadata() -> ComponentMetadata<'static> {
        ComponentMetadata {
            name: "PatternFormatter",
        }
    }

    fn build(params: Self::Params) -> Result<Self> {
        Ok(Self::new(RuntimePattern::new(params.template)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn builder(template: &str) -> RuntimePatternBuilder<String> {
        RuntimePattern::builder().template(template)
    }

    fn new(template: &str) -> Result<RuntimePattern> {
        RuntimePattern::new(template)
    }

    fn custom_pat_creator() -> impl Pattern {
        pattern::Level
    }

    #[test]
    fn valid() {
        assert!(new("").is_ok());
        assert!(new("{logger}").is_ok());
        assert!(builder("{logger} {$custom_pat}")
            .custom_pattern("custom_pat", custom_pat_creator)
            .build()
            .is_ok());
        assert!(builder("{logger} {$_custom_pat}")
            .custom_pattern("_custom_pat", custom_pat_creator)
            .build()
            .is_ok());
        assert!(builder("{logger} {$_2custom_pat}")
            .custom_pattern("_2custom_pat", custom_pat_creator)
            .build()
            .is_ok());
    }

    #[test]
    fn invalid() {
        assert!(matches!(new("{logger-name}"), Err(Error::BuildPattern(_))));
        assert!(matches!(new("{nonexistent}"), Err(Error::BuildPattern(_))));
        assert!(matches!(new("{}"), Err(Error::BuildPattern(_))));
        assert!(matches!(
            new("{logger} {$custom_pat_no_ref}"),
            Err(Error::BuildPattern(_))
        ));
        assert!(matches!(
            builder("{logger} {$custom_pat}")
                .custom_pattern("custom_pat", custom_pat_creator)
                .custom_pattern("", custom_pat_creator)
                .build(),
            Err(Error::BuildPattern(_))
        ));
        assert!(matches!(
            builder("{logger} {$custom_pat}")
                .custom_pattern("custom_pat", custom_pat_creator)
                .custom_pattern("custom-pat2", custom_pat_creator)
                .build(),
            Err(Error::BuildPattern(_))
        ));
        assert!(matches!(
            builder("{logger} {$custom_pat}")
                .custom_pattern("custom_pat", custom_pat_creator)
                .custom_pattern("2custom_pat", custom_pat_creator)
                .build(),
            Err(Error::BuildPattern(_))
        ));
        assert!(matches!(
            builder("{logger} {$r#custom_pat}")
                .custom_pattern("r#custom_pat", custom_pat_creator)
                .build(),
            Err(Error::BuildPattern(_))
        ));
    }

    #[test]
    fn deser_params() {
        assert!(
            toml::from_str::<PatternFormatterRuntimePatternParams>(
                r#"template = "[{level}] {payload}""#,
            )
            .unwrap()
                == PatternFormatterRuntimePatternParams {
                    template: "[{level}] {payload}".to_string()
                }
        );

        // TODO: Test ill-formed template string err
    }
}

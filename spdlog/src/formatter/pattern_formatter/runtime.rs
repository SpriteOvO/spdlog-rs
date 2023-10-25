use std::convert::Infallible;

use spdlog_internal::pattern_parser::{
    error::TemplateError,
    parse::{Template, TemplateToken},
    BuiltInFormatter, BuiltInFormatterInner, Error as PatternParserError,
    PatternKind as GenericPatternKind, PatternRegistry as GenericPatternRegistry,
    Result as PatternParserResult,
};

use super::{Pattern, PatternContext, __pattern as pattern};
use crate::{
    error::{BuildPatternErrorInner, Error},
    Record, Result, StringBuf,
};
type Patterns = Vec<Box<dyn Pattern>>;
type PatternCreator = Box<dyn Fn() -> Box<dyn Pattern>>;
type PatternRegistry = GenericPatternRegistry<PatternCreator>;
type PatternKind = GenericPatternKind<PatternCreator>;

#[rustfmt::skip] // rustfmt currently breaks some empty lines if `#[doc = include_str!("xxx")]` exists
/// A pattern built at runtime.
///
/// If your pattern is known at compile-time and you don't need to build new
/// patterns from runtime input, consider using [`pattern!`] macro.
/// 
/// # Usage
///
/// The template string format and built-in patterns are consistent with
/// [`pattern!`] macro. The only difference is the way they are built and that
/// one of them is built at compile-time and the other at runtime.
/// 
/// For other usage, please see the documentation of [`pattern!`] macro.
///
/// ## Basic Usage
/// 
/// ```
/// # use spdlog::formatter::{PatternFormatter, RuntimePattern};
/// use spdlog::info;
///
/// #
#[doc = include_str!(concat!(env!("OUT_DIR"), "/test_utils/common_for_doc_test.rs"))]
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let formatter = PatternFormatter::new(RuntimePattern::new("[{level}] {payload}{eol}")?);
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
///     formatter::{pattern, Pattern, PatternContext, PatternFormatter, RuntimePattern},
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
/// let formatter = PatternFormatter::new(
///     RuntimePattern::builder()
///         .template("[{level}] {payload} - {$mypat1} {$mypat2}{eol}")
///         .custom_pattern("mypat1", MyPattern::default)
///         .custom_pattern("mypat2", || pattern!("[{level_short}-{level}]"))
///         .build()?
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
    /// Constructs a runtime pattern from a template string.
    ///
    /// About the template string format, please see the documentation of
    /// [`pattern!`] macro.
    ///
    /// [`pattern!`]: crate::formatter::pattern
    pub fn new<T>(template: T) -> Result<Self>
    where
        T: Into<String>,
    {
        Self::builder().template(template).build()
    }

    /// Constructs a [`RuntimePatternBuilder`] to build a runtime pattern with
    /// advanced parameters.
    pub fn builder() -> RuntimePatternBuilder<()> {
        RuntimePatternBuilder {
            template: (),
            custom_patterns: Vec::new(),
        }
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
}

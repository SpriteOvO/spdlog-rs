use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::token::Paren;
use syn::{Expr, ExprLit, ExprTuple, Lit, LitStr, Path};

use crate::parse::{
    PatternTemplate, PatternTemplateColorRange, PatternTemplateFormatter, PatternTemplateLiteral,
    PatternTemplateToken,
};

pub(crate) struct Synthesiser {
    formatters: HashMap<String, Path>,
}

impl Synthesiser {
    pub(crate) fn new() -> Self {
        Self {
            formatters: HashMap::new(),
        }
    }

    pub(crate) fn with_builtin_formatters() -> Self {
        let mut synthesiser = Self::new();

        macro_rules! add_formatter_mappings {
            (
                $synthesiser:expr,
                $( [ $($name:literal),+ $(,)? ] => $formatter:ident ),+
                $(,)?
            ) => {
                $(
                    $(
                        // All built-in patterns implement the `Default` trait. So we use the
                        // `default` function to create instances of the built-in patterns.
                        $synthesiser.add_formatter_mapping(
                            String::from($name),
                            syn::parse_str(
                                stringify!(::spdlog::formatter::patterns::$formatter::default)
                            ).unwrap()
                        ).unwrap();
                    )+
                )+
            };
        }

        add_formatter_mappings!(synthesiser,
            ["v", "payload"] => Payload,
            ["t", "tid"] => ThreadId,
            ["P", "pid"] => ProcessId,
            ["n", "logger"] => LoggerName,
            ["l", "level"] => Level,
            ["L", "level-short"] => ShortLevel,
            ["a", "weekday-name"] => AbbrWeekdayName,
            ["A", "weekday-name-full"] => WeekdayName,
            ["b", "month-name"] => AbbrMonthName,
            ["B", "month-name-full"] => MonthName,
            ["c", "datetime"] => FullDateTime,
            ["C", "year-short"] => ShortYear,
            ["Y", "year"] => Year,
            ["D", "date-short"] => ShortDate,
            ["m", "month"] => Month,
            ["d", "day"] => Day,
            ["H", "hour"] => Hour,
            ["I", "hour-12"] => Hour12,
            ["M", "minute"] => Minute,
            ["S", "second"] => Second,
            ["e", "millisecond"] => Millisecond,
            ["f", "microsecond"] => Microsecond,
            ["F", "nanosecond"] => Nanosecond,
            ["p", "ampm"] => Ampm,
            ["r", "time-12"] => Time12,
            ["R", "time-short"] => ShortTime,
            ["T", "X", "time"] => Time,
            ["z", "tz-offset"] => TzOffset,
            ["E", "unix"] => UnixTimestamp,
            ["+", "full"] => Full,
            ["@", "loc"] => Loc,
            ["s", "source-basename"] => SourceBasename,
            ["g", "source"] => SourcePath,
            ["#", "line"] => SourceLine,
            ["%", "column"] => SourceColumn,
        );

        synthesiser
    }

    pub(crate) fn add_formatter_mapping(
        &mut self,
        name: String,
        formatter_factory_path: Path,
    ) -> Result<(), ConflictFormatterError> {
        if self.formatters.contains_key(&name) {
            return Err(ConflictFormatterError::new(name));
        }

        self.formatters.insert(name, formatter_factory_path);
        Ok(())
    }

    pub(crate) fn synthesis(
        &self,
        template: &PatternTemplate,
    ) -> Result<TokenStream, SynthesisError> {
        let expr = self.build_template_pattern_expr(template, false)?;
        Ok(expr.into_token_stream())
    }

    fn build_template_pattern_expr(
        &self,
        template: &PatternTemplate,
        mut color_range_seen: bool,
    ) -> Result<Expr, SynthesisError> {
        let mut template_expr = ExprTuple {
            attrs: Vec::new(),
            paren_token: Paren {
                span: Span::mixed_site(),
            },
            elems: Punctuated::new(),
        };

        for token in &template.tokens {
            let token_template_expr = match token {
                PatternTemplateToken::Literal(literal_token) => {
                    self.build_literal_template_pattern_expr(literal_token)?
                }
                PatternTemplateToken::Formatter(formatter_token) => {
                    self.build_formatter_template_pattern_expr(formatter_token)?
                }
                PatternTemplateToken::ColorRange(color_range_token) => {
                    if color_range_seen {
                        return Err(SynthesisError::MultipleColorRange);
                    }
                    color_range_seen = true;
                    self.build_color_range_template_pattern_expr(color_range_token)?
                }
            };
            template_expr.elems.push(token_template_expr);
        }

        Ok(Expr::Tuple(template_expr))
    }

    fn build_literal_template_pattern_expr(
        &self,
        literal_token: &PatternTemplateLiteral,
    ) -> Result<Expr, SynthesisError> {
        let lit = LitStr::new(&literal_token.literal, Span::mixed_site());
        let expr = Expr::Lit(ExprLit {
            attrs: Vec::new(),
            lit: Lit::Str(lit),
        });
        Ok(expr)
    }

    fn build_formatter_template_pattern_expr(
        &self,
        formatter_token: &PatternTemplateFormatter,
    ) -> Result<Expr, SynthesisError> {
        let formatter_creation_expr =
            self.build_formatter_creation_expr(&formatter_token.formatter_name)?;
        Ok(formatter_creation_expr)
    }

    fn build_color_range_template_pattern_expr(
        &self,
        color_range_token: &PatternTemplateColorRange,
    ) -> Result<Expr, SynthesisError> {
        let body_pattern_expr = self.build_template_pattern_expr(&color_range_token.body, true)?;
        let expr = self.build_color_range_pattern_creation_expr(body_pattern_expr)?;
        Ok(expr)
    }

    fn build_formatter_creation_expr(&self, formatter_name: &str) -> Result<Expr, SynthesisError> {
        let formatter_factory_path = self
            .formatters
            .get(formatter_name)
            .ok_or_else(|| SynthesisError::UnknownFormatterName(String::from(formatter_name)))?;

        let stream = quote::quote!( #formatter_factory_path () );
        let factory_call_expr = syn::parse2(stream).unwrap();
        Ok(Expr::Call(factory_call_expr))
    }

    fn build_color_range_pattern_creation_expr(&self, body: Expr) -> Result<Expr, SynthesisError> {
        let color_range_pattern_new_path: Path =
            syn::parse_str("::spdlog::formatter::patterns::ColorRange::new").unwrap();
        let stream = quote::quote!( #color_range_pattern_new_path (#body) );
        let expr = syn::parse2(stream).unwrap();
        Ok(Expr::Call(expr))
    }
}

#[derive(Debug)]
pub(crate) struct ConflictFormatterError {
    conflict_name: String,
}

impl ConflictFormatterError {
    fn new(conflict_name: String) -> Self {
        Self { conflict_name }
    }
}

impl Display for ConflictFormatterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "formatter name \"{}\" conflicts", self.conflict_name)
    }
}

impl Error for ConflictFormatterError {}

#[derive(Debug)]
pub(crate) enum SynthesisError {
    UnknownFormatterName(String),
    MultipleColorRange,
}

impl Display for SynthesisError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownFormatterName(name) => write!(f, "unknown formatter name: \"{}\"", name),
            Self::MultipleColorRange => write!(f, "more than 1 color range in the template"),
        }
    }
}

impl Error for SynthesisError {}

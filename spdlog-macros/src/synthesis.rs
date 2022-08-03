use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::token::Paren;
use syn::{Expr, ExprLit, ExprTuple, Lit, LitStr, Path};

use crate::parse::{
    PatternTemplate, PatternTemplateFormatter, PatternTemplateLiteral, PatternTemplateStyleRange,
    PatternTemplateToken,
};

pub(crate) struct Synthesiser {
    formatters: HashMap<String, PatternFormatter>,
}

impl Synthesiser {
    pub(crate) fn new() -> Self {
        Self {
            formatters: HashMap::new(),
        }
    }

    pub(crate) fn with_builtin_formatters() -> Self {
        let mut synthesiser = Self::new();

        macro_rules! map_builtin_formatters {
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
                            PatternFormatter {
                                factory_path: syn::parse_str(
                                    stringify!(::spdlog::formatter::__pattern::$formatter::default)
                                ).unwrap(),
                                kind: PatternFormatterKind::BuiltIn,
                            }
                        ).unwrap();
                    )+
                )+
            };
        }

        map_builtin_formatters! {synthesiser,
            ["weekday_name"] => AbbrWeekdayName,
            ["weekday_name_full"] => WeekdayName,
            ["month_name"] => AbbrMonthName,
            ["month_name_full"] => MonthName,
            ["datetime"] => FullDateTime,
            ["year_short"] => ShortYear,
            ["year"] => Year,
            ["date_short"] => ShortDate,
            ["date"] => Date,
            ["month"] => Month,
            ["day"] => Day,
            ["hour"] => Hour,
            ["hour_12"] => Hour12,
            ["minute"] => Minute,
            ["second"] => Second,
            ["millisecond"] => Millisecond,
            ["microsecond"] => Microsecond,
            ["nanosecond"] => Nanosecond,
            ["am_pm"] => AmPm,
            ["time_12"] => Time12,
            ["time_short"] => ShortTime,
            ["time"] => Time,
            ["tz_offset"] => TzOffset,
            ["unix_timestamp"] => UnixTimestamp,
            ["full"] => Full,
            ["level"] => Level,
            ["level_short"] => ShortLevel,
            ["loc"] => Loc,
            ["file_name"] => SourceFilename,
            ["file"] => SourceFile,
            ["line"] => SourceLine,
            ["column"] => SourceColumn,
            ["logger"] => LoggerName,
            ["payload"] => Payload,
            ["pid"] => ProcessId,
            ["tid"] => ThreadId,
        }

        synthesiser
    }

    pub(crate) fn add_formatter_mapping(
        &mut self,
        name: String,
        formatter: PatternFormatter,
    ) -> Result<(), ConflictFormatterError> {
        if self.formatters.contains_key(&name) {
            return Err(ConflictFormatterError::new(name));
        }

        self.formatters.insert(name, formatter);
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
        mut style_range_seen: bool,
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
                PatternTemplateToken::StyleRange(style_range_token) => {
                    if style_range_seen {
                        return Err(SynthesisError::MultipleStyleRange);
                    }
                    style_range_seen = true;
                    self.build_style_range_template_pattern_expr(style_range_token)?
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
        let formatter_creation_expr = self.build_formatter_creation_expr(formatter_token)?;
        Ok(formatter_creation_expr)
    }

    fn build_style_range_template_pattern_expr(
        &self,
        style_range_token: &PatternTemplateStyleRange,
    ) -> Result<Expr, SynthesisError> {
        let body_pattern_expr = self.build_template_pattern_expr(&style_range_token.body, true)?;
        let expr = self.build_style_range_pattern_creation_expr(body_pattern_expr)?;
        Ok(expr)
    }

    fn build_formatter_creation_expr(
        &self,
        formatter_token: &PatternTemplateFormatter,
    ) -> Result<Expr, SynthesisError> {
        let formatter_factory_path = &self
            .formatters
            .get(&formatter_token.name)
            .filter(|formatter| formatter_token.kind == formatter.kind)
            .ok_or_else(|| SynthesisError::UnknownFormatterName(formatter_token.name.clone()))?
            .factory_path;

        let stream = quote::quote!( #formatter_factory_path () );
        let factory_call_expr = syn::parse2(stream).unwrap();
        Ok(Expr::Call(factory_call_expr))
    }

    fn build_style_range_pattern_creation_expr(&self, body: Expr) -> Result<Expr, SynthesisError> {
        let style_range_pattern_new_path: Path =
            syn::parse_str("::spdlog::formatter::__pattern::StyleRange::new").unwrap();
        let stream = quote::quote!( #style_range_pattern_new_path (#body) );
        let expr = syn::parse2(stream).unwrap();
        Ok(Expr::Call(expr))
    }
}

pub(crate) struct PatternFormatter {
    pub(crate) factory_path: Path,
    pub(crate) kind: PatternFormatterKind,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum PatternFormatterKind {
    Custom,
    BuiltIn,
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
    MultipleStyleRange,
}

impl Display for SynthesisError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownFormatterName(name) => write!(f, "unknown formatter name: \"{}\"", name),
            Self::MultipleStyleRange => write!(f, "more than 1 style range in the template"),
        }
    }
}

impl Error for SynthesisError {}

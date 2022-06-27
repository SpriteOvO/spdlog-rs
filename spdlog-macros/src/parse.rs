use nom::Parser;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    LitStr, Path, Token,
};

/// A parsed pattern.
///
/// A [`Pattern`] gives a structural representation of a pattern parsed from the
/// token stream given to the `pattern` macro.
pub(crate) struct Pattern {
    /// The template string included in the pattern.
    pub(crate) template: PatternTemplate,

    /// Any user-provided pattern-to-formatter mapping.
    pub(crate) custom_pat_mapping: CustomPatternMapping,
}

impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let template_literal: LitStr = input.parse()?;
        let template = PatternTemplate::parse_from_template(template_literal)?;

        input.parse::<Option<Token![,]>>()?;
        let custom_pat_mapping = input.parse()?;

        Ok(Self {
            template,
            custom_pat_mapping,
        })
    }
}

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub(crate) struct PatternTemplate {
    pub(crate) tokens: Vec<PatternTemplateToken>,
}

impl PatternTemplate {
    fn parse_from_template(template: LitStr) -> syn::Result<Self> {
        let template_value = template.value();
        let mut parser = Self::parser();

        let template_str = template_value.as_str();
        let (_, parsed_template) = parser.parse(template_str).map_err(|err| {
            let parser_err = match err {
                nom::Err::Incomplete(..) => {
                    // The "complete" combinator should transform `Incomplete` into `Error`
                    unreachable!();
                }
                nom::Err::Error(err) => err,
                nom::Err::Failure(err) => err,
            };
            let err_byte_position = unsafe {
                parser_err
                    .input
                    .as_bytes()
                    .as_ptr()
                    .offset_from(template_str.as_bytes().as_ptr())
            } as usize;

            let err_span = template
                .token()
                .subspan(err_byte_position..)
                .unwrap_or_else(|| template.span());
            syn::Error::new(err_span, "failed to parse pattern template string")
        })?;

        Ok(parsed_template)
    }

    fn parser<'a>() -> impl Parser<&'a str, Self, nom::error::Error<&'a str>> {
        let token_parser = PatternTemplateToken::parser();
        nom::combinator::complete(nom::multi::many0(token_parser).and(nom::combinator::eof))
            .map(|(tokens, _)| Self { tokens })
    }

    fn parser_without_color_range<'a>() -> impl Parser<&'a str, Self, nom::error::Error<&'a str>> {
        let token_parser = PatternTemplateToken::parser_without_color_range();
        nom::combinator::complete(nom::multi::many0(token_parser).and(nom::combinator::eof))
            .map(|(tokens, _)| Self { tokens })
    }
}

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub(crate) enum PatternTemplateToken {
    Literal(PatternTemplateLiteral),
    Formatter(PatternTemplateFormatter),
    ColorRange(PatternTemplateColorRange),
}

impl PatternTemplateToken {
    fn parser<'a>() -> impl Parser<&'a str, Self, nom::error::Error<&'a str>> {
        let color_range_parser = PatternTemplateColorRange::parser();
        let other_parser = Self::parser_without_color_range();

        nom::combinator::map(color_range_parser, Self::ColorRange).or(other_parser)
    }

    fn parser_without_color_range<'a>() -> impl Parser<&'a str, Self, nom::error::Error<&'a str>> {
        let literal_parser = PatternTemplateLiteral::parser();
        let formatter_parser = PatternTemplateFormatter::parser();

        nom::combinator::map(literal_parser, Self::Literal)
            .or(nom::combinator::map(formatter_parser, Self::Formatter))
    }
}

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub(crate) struct PatternTemplateLiteral {
    pub(crate) literal: String,
}

impl PatternTemplateLiteral {
    fn parser<'a>() -> impl Parser<&'a str, Self, nom::error::Error<&'a str>> {
        let literal_char_parser = nom::combinator::value('{', nom::bytes::complete::tag("{{"))
            .or(nom::combinator::value('}', nom::bytes::complete::tag("}}")))
            .or(nom::character::complete::none_of("{"));
        nom::multi::many1(literal_char_parser).map(|literal_chars| Self {
            literal: literal_chars.into_iter().collect(),
        })
    }
}

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub(crate) struct PatternTemplateFormatter {
    pub(crate) formatter_name: String,
}

impl PatternTemplateFormatter {
    fn parser<'a>() -> impl Parser<&'a str, Self, nom::error::Error<&'a str>> {
        let open_paren_parser = nom::character::complete::char('{');
        let close_paren_parser = nom::character::complete::char('}');
        let formatter_name_parser = nom::multi::many1(nom::character::complete::none_of("{}"));

        nom::sequence::delimited(open_paren_parser, formatter_name_parser, close_paren_parser).map(
            |name_chars| Self {
                formatter_name: name_chars.into_iter().collect(),
            },
        )
    }
}

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub(crate) struct PatternTemplateColorRange {
    pub(crate) body: PatternTemplate,
}

impl PatternTemplateColorRange {
    fn parser<'a>() -> impl Parser<&'a str, Self, nom::error::Error<&'a str>> {
        nom::bytes::complete::tag("{^")
            .and(nom::bytes::complete::take_until("$}"))
            .and(nom::bytes::complete::tag("$}"))
            .map(|((_, body), _)| body)
            .and_then(PatternTemplate::parser_without_color_range())
            .map(|body| Self { body })
    }
}

/// Mapping from user-provided patterns to formatters.
pub(crate) struct CustomPatternMapping {
    pub(crate) mapping_pairs: Vec<(LitStr, CustomPatternFactoryFunctionId)>,
}

impl Parse for CustomPatternMapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let items: Punctuated<_, Token![,]> =
            input.parse_terminated(CustomPatternMappingItem::parse)?;

        let mut mapping_pairs = Vec::new();
        for i in items {
            for name in i.names {
                mapping_pairs.push((name, i.factory.clone()));
            }
        }

        Ok(Self { mapping_pairs })
    }
}

/// Identifier of a function that produces custom pattern formatters.
#[derive(Clone)]
pub(crate) struct CustomPatternFactoryFunctionId(pub(crate) Path);

impl From<Path> for CustomPatternFactoryFunctionId {
    fn from(p: Path) -> Self {
        Self(p)
    }
}

impl Parse for CustomPatternFactoryFunctionId {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let p = input.parse()?;
        Ok(Self(p))
    }
}

struct CustomPatternMappingItem {
    names: Vec<LitStr>,
    factory: CustomPatternFactoryFunctionId,
}

impl Parse for CustomPatternMappingItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let names_input;
        braced!(names_input in input);

        let names: Punctuated<_, Token![,]> =
            names_input.parse_terminated(<LitStr as Parse>::parse)?;
        let names = names.into_iter().collect();

        input.parse::<Token![=>]>()?;

        let factory: CustomPatternFactoryFunctionId = input.parse()?;
        Ok(Self { names, factory })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod template_parsing {
        use super::*;

        fn parse_template_str(template: &str) -> nom::IResult<&str, PatternTemplate> {
            PatternTemplate::parser().parse(template)
        }

        #[test]
        fn test_parse_basic() {
            assert_eq!(
                parse_template_str(r#"hello"#),
                Ok((
                    "",
                    PatternTemplate {
                        tokens: vec![PatternTemplateToken::Literal(PatternTemplateLiteral {
                            literal: String::from("hello"),
                        }),],
                    }
                ))
            );
        }

        #[test]
        fn test_parse_empty() {
            assert_eq!(
                parse_template_str(""),
                Ok(("", PatternTemplate { tokens: Vec::new() },))
            );
        }

        #[test]
        fn test_parse_escape_literal() {
            assert_eq!(
                parse_template_str(r#"hello {{name}}"#),
                Ok((
                    "",
                    PatternTemplate {
                        tokens: vec![PatternTemplateToken::Literal(PatternTemplateLiteral {
                            literal: String::from("hello {name}"),
                        }),],
                    }
                ))
            );
        }

        #[test]
        fn test_parse_escape_literal_at_beginning() {
            assert_eq!(
                parse_template_str(r#"{{name}}"#),
                Ok((
                    "",
                    PatternTemplate {
                        tokens: vec![PatternTemplateToken::Literal(PatternTemplateLiteral {
                            literal: String::from("{name}"),
                        }),],
                    }
                ))
            );
        }

        #[test]
        fn test_parse_formatter_basic() {
            assert_eq!(
                parse_template_str(r#"hello {name}!"#),
                Ok((
                    "",
                    PatternTemplate {
                        tokens: vec![
                            PatternTemplateToken::Literal(PatternTemplateLiteral {
                                literal: String::from("hello "),
                            }),
                            PatternTemplateToken::Formatter(PatternTemplateFormatter {
                                formatter_name: String::from("name"),
                            }),
                            PatternTemplateToken::Literal(PatternTemplateLiteral {
                                literal: String::from("!"),
                            }),
                        ],
                    }
                ))
            );
        }

        #[test]
        fn test_parse_literal_single_close_paren() {
            assert_eq!(
                parse_template_str(r#"hello name}"#),
                Ok((
                    "",
                    PatternTemplate {
                        tokens: vec![PatternTemplateToken::Literal(PatternTemplateLiteral {
                            literal: String::from("hello name}"),
                        }),],
                    }
                ))
            );
        }

        #[test]
        fn test_parse_formatter_invalid_name() {
            assert!(parse_template_str(r#"hello {name{}!"#).is_err());
        }

        #[test]
        fn test_parse_formatter_missing_close_paren() {
            assert!(parse_template_str(r#"hello {name"#).is_err());
        }

        #[test]
        fn test_parse_formatter_duplicate_close_paren() {
            assert_eq!(
                parse_template_str(r#"hello {name}}"#),
                Ok((
                    "",
                    PatternTemplate {
                        tokens: vec![
                            PatternTemplateToken::Literal(PatternTemplateLiteral {
                                literal: String::from("hello "),
                            }),
                            PatternTemplateToken::Formatter(PatternTemplateFormatter {
                                formatter_name: String::from("name"),
                            }),
                            PatternTemplateToken::Literal(PatternTemplateLiteral {
                                literal: String::from("}"),
                            }),
                        ],
                    }
                ))
            );
        }

        #[test]
        fn test_parse_color_range_basic() {
            assert_eq!(
                parse_template_str(r#"hello {^world$}"#),
                Ok((
                    "",
                    PatternTemplate {
                        tokens: vec![
                            PatternTemplateToken::Literal(PatternTemplateLiteral {
                                literal: String::from("hello "),
                            }),
                            PatternTemplateToken::ColorRange(PatternTemplateColorRange {
                                body: PatternTemplate {
                                    tokens: vec![PatternTemplateToken::Literal(
                                        PatternTemplateLiteral {
                                            literal: String::from("world"),
                                        }
                                    ),],
                                },
                            }),
                        ],
                    }
                ))
            );
        }

        #[test]
        fn test_parse_color_range_nested() {
            assert!(parse_template_str(r#"hello {^ hello {^ world $} $}"#).is_err());
        }
    }
}
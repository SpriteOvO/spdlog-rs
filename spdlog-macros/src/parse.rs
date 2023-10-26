use nom::Parser;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, LitStr, Path, Token,
};

use crate::{helper, synthesis::PatternFormatterKind};

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

    #[must_use]
    fn parser<'a>() -> impl Parser<&'a str, Self, nom::error::Error<&'a str>> {
        let token_parser = PatternTemplateToken::parser();
        nom::combinator::complete(nom::multi::many0(token_parser).and(nom::combinator::eof))
            .map(|(tokens, _)| Self { tokens })
    }

    #[must_use]
    fn parser_without_style_range<'a>() -> impl Parser<&'a str, Self, nom::error::Error<&'a str>> {
        let token_parser = PatternTemplateToken::parser_without_style_range();
        nom::combinator::complete(nom::multi::many0(token_parser).and(nom::combinator::eof))
            .map(|(tokens, _)| Self { tokens })
    }
}

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub(crate) enum PatternTemplateToken {
    Literal(PatternTemplateLiteral),
    Formatter(PatternTemplateFormatter),
    StyleRange(PatternTemplateStyleRange),
}

impl PatternTemplateToken {
    #[must_use]
    fn parser<'a>() -> impl Parser<&'a str, Self, nom::error::Error<&'a str>> {
        let style_range_parser = PatternTemplateStyleRange::parser();
        let other_parser = Self::parser_without_style_range();

        nom::combinator::map(style_range_parser, Self::StyleRange).or(other_parser)
    }

    #[must_use]
    fn parser_without_style_range<'a>() -> impl Parser<&'a str, Self, nom::error::Error<&'a str>> {
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
    #[must_use]
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
    pub(crate) name: String,
    pub(crate) kind: PatternFormatterKind,
}

impl PatternTemplateFormatter {
    #[must_use]
    fn parser<'a>() -> impl Parser<&'a str, Self, nom::error::Error<&'a str>> {
        let open_paren_parser = nom::character::complete::char('{');
        let close_paren_parser = nom::character::complete::char('}');
        let formatter_prefix_parser = nom::character::complete::char('$');
        let formatter_name_parser = nom::combinator::recognize(nom::sequence::tuple((
            nom::combinator::opt(formatter_prefix_parser),
            nom::branch::alt((
                nom::character::complete::alpha1,
                nom::bytes::complete::tag("_"),
            )),
            nom::multi::many0_count(nom::branch::alt((
                nom::character::complete::alphanumeric1,
                nom::bytes::complete::tag("_"),
            ))),
        )));

        nom::sequence::delimited(open_paren_parser, formatter_name_parser, close_paren_parser).map(
            |name: &str| match name.strip_prefix('$') {
                Some(custom_name) => Self {
                    name: custom_name.to_owned(),
                    kind: PatternFormatterKind::Custom,
                },
                None => Self {
                    name: name.to_owned(),
                    kind: PatternFormatterKind::BuiltIn,
                },
            },
        )
    }
}

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub(crate) struct PatternTemplateStyleRange {
    pub(crate) body: PatternTemplate,
}

impl PatternTemplateStyleRange {
    #[must_use]
    fn parser<'a>() -> impl Parser<&'a str, Self, nom::error::Error<&'a str>> {
        nom::bytes::complete::tag("{^")
            .and(helper::take_until_unbalanced('{', '}'))
            .and(nom::bytes::complete::tag("}"))
            .map(|((_, body), _)| body)
            .and_then(PatternTemplate::parser_without_style_range())
            .map(|body| Self { body })
    }
}

/// Mapping from user-provided patterns to formatters.
pub(crate) struct CustomPatternMapping {
    pub(crate) mapping_pairs: Vec<(Ident, CustomPatternFactoryFunctionId)>,
}

impl Parse for CustomPatternMapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let items = Punctuated::<CustomPatternMappingItem, Token![,]>::parse_terminated(input)?;

        let mapping_pairs = items.into_iter().fold(vec![], |mut prev, item| {
            prev.push((item.name, item.factory));
            prev
        });

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
    name: Ident,
    factory: CustomPatternFactoryFunctionId,
}

impl Parse for CustomPatternMappingItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name_input;
        braced!(name_input in input);

        name_input.parse::<Token![$]>()?;

        let name = name_input.parse()?;
        input.parse::<Token![=>]>()?;
        let factory: CustomPatternFactoryFunctionId = input.parse()?;

        Ok(Self { name, factory })
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
                parse_template_str(r#"hello {name}!{$custom}"#),
                Ok((
                    "",
                    PatternTemplate {
                        tokens: vec![
                            PatternTemplateToken::Literal(PatternTemplateLiteral {
                                literal: String::from("hello "),
                            }),
                            PatternTemplateToken::Formatter(PatternTemplateFormatter {
                                name: String::from("name"),
                                kind: PatternFormatterKind::BuiltIn
                            }),
                            PatternTemplateToken::Literal(PatternTemplateLiteral {
                                literal: String::from("!"),
                            }),
                            PatternTemplateToken::Formatter(PatternTemplateFormatter {
                                name: String::from("custom"),
                                kind: PatternFormatterKind::Custom
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
                                name: String::from("name"),
                                kind: PatternFormatterKind::BuiltIn
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
        fn test_parse_style_range_basic() {
            assert_eq!(
                parse_template_str(r#"hello {^world}"#),
                Ok((
                    "",
                    PatternTemplate {
                        tokens: vec![
                            PatternTemplateToken::Literal(PatternTemplateLiteral {
                                literal: String::from("hello "),
                            }),
                            PatternTemplateToken::StyleRange(PatternTemplateStyleRange {
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

            assert_eq!(
                parse_template_str(r#"hello {^world {b_pat} {$c_pat} {{escape}}}"#),
                Ok((
                    "",
                    PatternTemplate {
                        tokens: vec![
                            PatternTemplateToken::Literal(PatternTemplateLiteral {
                                literal: String::from("hello "),
                            }),
                            PatternTemplateToken::StyleRange(PatternTemplateStyleRange {
                                body: PatternTemplate {
                                    tokens: vec![
                                        PatternTemplateToken::Literal(PatternTemplateLiteral {
                                            literal: String::from("world "),
                                        }),
                                        PatternTemplateToken::Formatter(PatternTemplateFormatter {
                                            name: String::from("b_pat"),
                                            kind: PatternFormatterKind::BuiltIn
                                        }),
                                        PatternTemplateToken::Literal(PatternTemplateLiteral {
                                            literal: String::from(" "),
                                        }),
                                        PatternTemplateToken::Formatter(PatternTemplateFormatter {
                                            name: String::from("c_pat"),
                                            kind: PatternFormatterKind::Custom
                                        }),
                                        PatternTemplateToken::Literal(PatternTemplateLiteral {
                                            literal: String::from(" {escape}"),
                                        }),
                                    ],
                                },
                            }),
                        ],
                    }
                ))
            );
        }

        #[test]
        fn test_parse_style_range_nested() {
            assert!(parse_template_str(r#"hello {^ hello {^ world } }"#).is_err());
        }
    }
}

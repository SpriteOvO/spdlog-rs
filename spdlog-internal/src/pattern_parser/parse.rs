use nom::{error::Error as NomError, Parser};

use super::{helper, Error, Result};

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct Template<'a> {
    pub tokens: Vec<TemplateToken<'a>>,
}

impl<'a> Template<'a> {
    pub fn parse(template: &'a str) -> Result<Self> {
        let mut parser = Self::parser();

        let (_, parsed_template) = parser.parse(template).map_err(|err| {
            let err = match err {
                // The "complete" combinator should transform `Incomplete` into `Error`
                nom::Err::Incomplete(..) => unreachable!(),
                nom::Err::Error(err) | nom::Err::Failure(err) => err,
            };
            Error::Parse(NomError::new(err.input.into(), err.code))
        })?;

        Ok(parsed_template)
    }
}

impl<'a> Template<'a> {
    #[must_use]
    fn parser() -> impl Parser<&'a str, Output = Template<'a>, Error = NomError<&'a str>> {
        let token_parser = TemplateToken::parser();
        nom::combinator::complete(nom::multi::many0(token_parser).and(nom::combinator::eof))
            .map(|(tokens, _)| Self { tokens })
    }

    #[must_use]
    fn parser_without_style_range(
    ) -> impl Parser<&'a str, Output = Template<'a>, Error = NomError<&'a str>> {
        let token_parser = TemplateToken::parser_without_style_range();
        nom::combinator::complete(nom::multi::many0(token_parser).and(nom::combinator::eof))
            .map(|(tokens, _)| Self { tokens })
    }
}

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub enum TemplateToken<'a> {
    Literal(TemplateLiteral),
    Formatter(TemplateFormatterToken<'a>),
    StyleRange(TemplateStyleRange<'a>),
}

impl<'a> TemplateToken<'a> {
    #[must_use]
    fn parser() -> impl Parser<&'a str, Output = TemplateToken<'a>, Error = NomError<&'a str>> {
        let style_range_parser = TemplateStyleRange::parser();
        let other_parser = Self::parser_without_style_range();

        nom::combinator::map(style_range_parser, Self::StyleRange).or(other_parser)
    }

    #[must_use]
    fn parser_without_style_range(
    ) -> impl Parser<&'a str, Output = TemplateToken<'a>, Error = NomError<&'a str>> {
        let literal_parser = TemplateLiteral::parser();
        let formatter_parser = TemplateFormatterToken::parser();

        nom::combinator::map(literal_parser, Self::Literal)
            .or(nom::combinator::map(formatter_parser, Self::Formatter))
    }
}

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct TemplateLiteral {
    pub literal: String,
}

impl TemplateLiteral {
    #[must_use]
    fn parser<'a>() -> impl Parser<&'a str, Output = Self, Error = NomError<&'a str>> {
        let literal_char_parser = nom::combinator::value('{', nom::bytes::complete::tag("{{"))
            .or(nom::combinator::value('}', nom::bytes::complete::tag("}}")))
            .or(nom::character::complete::none_of("{"));
        nom::multi::many1(literal_char_parser).map(|literal_chars| Self {
            literal: literal_chars.into_iter().collect(),
        })
    }
}

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct TemplateFormatterToken<'a> {
    pub has_custom_prefix: bool,
    pub placeholder: &'a str,
}

impl<'a> TemplateFormatterToken<'a> {
    #[must_use]
    fn parser(
    ) -> impl Parser<&'a str, Output = TemplateFormatterToken<'a>, Error = NomError<&'a str>> {
        let open_paren = nom::character::complete::char('{');
        let close_paren = nom::character::complete::char('}');
        let formatter_prefix = nom::character::complete::char('$');
        let formatter_placeholder = nom::combinator::recognize((
            nom::combinator::opt(formatter_prefix),
            nom::branch::alt((
                nom::character::complete::alpha1,
                nom::bytes::complete::tag("_"),
            )),
            nom::multi::many0_count(nom::branch::alt((
                nom::character::complete::alphanumeric1,
                nom::bytes::complete::tag("_"),
            ))),
        ));

        nom::sequence::delimited(open_paren, formatter_placeholder, close_paren).map(
            move |placeholder: &str| match placeholder.strip_prefix('$') {
                Some(placeholder) => Self {
                    has_custom_prefix: true,
                    placeholder,
                },
                None => Self {
                    has_custom_prefix: false,
                    placeholder,
                },
            },
        )
    }
}

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct TemplateStyleRange<'a> {
    pub body: Template<'a>,
}

impl<'a> TemplateStyleRange<'a> {
    #[must_use]
    fn parser() -> impl Parser<&'a str, Output = TemplateStyleRange<'a>, Error = NomError<&'a str>>
    {
        nom::bytes::complete::tag("{^")
            .and(helper::take_until_unbalanced('{', '}'))
            .and(nom::bytes::complete::tag("}"))
            .map(|((_, body), _)| body)
            .and_then(Template::parser_without_style_range())
            .map(|body| Self { body })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod template_parsing {
        use super::*;

        fn parse_template_str(template: &str) -> nom::IResult<&str, Template<'_>> {
            Template::parser().parse(template)
        }

        #[test]
        fn test_parse_basic() {
            assert_eq!(
                parse_template_str(r#"hello"#),
                Ok((
                    "",
                    Template {
                        tokens: vec![TemplateToken::Literal(TemplateLiteral {
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
                Ok(("", Template { tokens: Vec::new() },))
            );
        }

        #[test]
        fn test_parse_escape_literal() {
            assert_eq!(
                parse_template_str(r#"hello {{name}}"#),
                Ok((
                    "",
                    Template {
                        tokens: vec![TemplateToken::Literal(TemplateLiteral {
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
                    Template {
                        tokens: vec![TemplateToken::Literal(TemplateLiteral {
                            literal: String::from("{name}"),
                        }),],
                    }
                ))
            );
        }

        #[test]
        fn test_parse_formatter_basic() {
            assert_eq!(
                parse_template_str(r#"hello {full}!{$custom}"#),
                Ok((
                    "",
                    Template {
                        tokens: vec![
                            TemplateToken::Literal(TemplateLiteral {
                                literal: String::from("hello "),
                            }),
                            TemplateToken::Formatter(TemplateFormatterToken {
                                has_custom_prefix: false,
                                placeholder: "full"
                            }),
                            TemplateToken::Literal(TemplateLiteral {
                                literal: String::from("!"),
                            }),
                            TemplateToken::Formatter(TemplateFormatterToken {
                                has_custom_prefix: true,
                                placeholder: "custom",
                            }),
                        ],
                    }
                ))
            );

            assert_eq!(
                parse_template_str(r#"hello {not_exists}!{$custom}"#),
                Ok((
                    "",
                    Template {
                        tokens: vec![
                            TemplateToken::Literal(TemplateLiteral {
                                literal: String::from("hello "),
                            }),
                            TemplateToken::Formatter(TemplateFormatterToken {
                                has_custom_prefix: false,
                                placeholder: "not_exists",
                            }),
                            TemplateToken::Literal(TemplateLiteral {
                                literal: String::from("!"),
                            }),
                            TemplateToken::Formatter(TemplateFormatterToken {
                                has_custom_prefix: true,
                                placeholder: "custom",
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
                    Template {
                        tokens: vec![TemplateToken::Literal(TemplateLiteral {
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
                parse_template_str(r#"hello {time}}"#),
                Ok((
                    "",
                    Template {
                        tokens: vec![
                            TemplateToken::Literal(TemplateLiteral {
                                literal: String::from("hello "),
                            }),
                            TemplateToken::Formatter(TemplateFormatterToken {
                                has_custom_prefix: false,
                                placeholder: "time",
                            }),
                            TemplateToken::Literal(TemplateLiteral {
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
                    Template {
                        tokens: vec![
                            TemplateToken::Literal(TemplateLiteral {
                                literal: String::from("hello "),
                            }),
                            TemplateToken::StyleRange(TemplateStyleRange {
                                body: Template {
                                    tokens: vec![TemplateToken::Literal(TemplateLiteral {
                                        literal: String::from("world"),
                                    }),],
                                },
                            }),
                        ],
                    }
                ))
            );

            assert_eq!(
                parse_template_str(r#"hello {^world {level} {$c_pat} {{escape}}}"#),
                Ok((
                    "",
                    Template {
                        tokens: vec![
                            TemplateToken::Literal(TemplateLiteral {
                                literal: String::from("hello "),
                            }),
                            TemplateToken::StyleRange(TemplateStyleRange {
                                body: Template {
                                    tokens: vec![
                                        TemplateToken::Literal(TemplateLiteral {
                                            literal: String::from("world "),
                                        }),
                                        TemplateToken::Formatter(TemplateFormatterToken {
                                            has_custom_prefix: false,
                                            placeholder: "level",
                                        }),
                                        TemplateToken::Literal(TemplateLiteral {
                                            literal: String::from(" "),
                                        }),
                                        TemplateToken::Formatter(TemplateFormatterToken {
                                            has_custom_prefix: true,
                                            placeholder: "c_pat",
                                        }),
                                        TemplateToken::Literal(TemplateLiteral {
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

use std::fmt::{self, Display};

use nom::error::Error as NomError;
use thiserror::Error;

use super::PatternKind;
use crate::impossible;

#[derive(Error, Debug)]
pub enum Error {
    ConflictName {
        existing: PatternKind<()>,
        incoming: PatternKind<()>,
    },
    Template(TemplateError),
    Parse(NomError<String>),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ConflictName { existing, incoming } => match (existing, incoming) {
                (PatternKind::BuiltIn(_), PatternKind::Custom { .. }) => {
                    write!(
                        f,
                        "'{}' is already a built-in pattern, please try another name",
                        existing.placeholder()
                    )
                }
                (PatternKind::Custom { .. }, PatternKind::Custom { .. }) => {
                    write!(
                        f,
                        "the constructor of custom pattern '{}' is specified more than once",
                        existing.placeholder()
                    )
                }
                (_, PatternKind::BuiltIn { .. }) => {
                    impossible!("{}", self)
                }
            },
            Error::Template(err) => {
                write!(f, "template ill-format: {}", err)
            }
            Error::Parse(err) => {
                write!(f, "failed to parse template string: {}", err)
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum TemplateError {
    WrongPatternKindReference {
        is_builtin_as_custom: bool,
        placeholder: String,
    },
    UnknownPatternReference {
        is_custom: bool,
        placeholder: String,
    },
    MultipleStyleRange,
}

impl Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateError::WrongPatternKindReference {
                is_builtin_as_custom,
                placeholder,
            } => {
                if *is_builtin_as_custom {
                    write!(
                        f,
                        "'{}' is a built-in pattern, it cannot be used as a custom pattern. try to replace it with `{{{}}}`",
                        placeholder, placeholder
                    )
                } else {
                    write!(
                        f,
                        "'{}' is a custom pattern, it cannot be used as a built-in pattern. try to replace it with `{{${}}}`",
                        placeholder, placeholder
                    )
                }
            }
            TemplateError::UnknownPatternReference {
                is_custom,
                placeholder,
            } => {
                if *is_custom {
                    write!(
                        f,
                        "the constructor of custom pattern '{}' is not specified",
                        placeholder
                    )
                } else {
                    write!(f, "no built-in pattern named '{}'", placeholder)
                }
            }
            TemplateError::MultipleStyleRange => {
                write!(f, "multiple style ranges are not currently supported")
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

use std::{
    borrow::Cow,
    collections::{hash_map::Entry, HashMap},
    fmt::Debug,
    hash::Hash,
};

use super::{error::TemplateError, BuiltInFormatter, Error, PatternKind, Result};
use crate::impossible;

#[derive(Clone, Debug)]
pub struct PatternRegistry<F> {
    formatters: HashMap<Cow<'static, str>, PatternKind<F>>,
}

impl<F> PatternRegistry<F> {
    #[must_use]
    pub fn with_builtin() -> Self {
        let mut registry = Self {
            formatters: HashMap::new(),
        };

        BuiltInFormatter::iter().for_each(|formatter| registry.register_builtin(formatter));
        registry
    }

    pub fn register_custom(
        &mut self,
        placeholder: impl Into<Cow<'static, str>>,
        factory: F,
    ) -> Result<()> {
        let placeholder = placeholder.into();

        let incoming = PatternKind::Custom {
            placeholder: placeholder.clone(),
            factory,
        };

        match self.formatters.entry(placeholder) {
            Entry::Occupied(entry) => Err(Error::ConflictName {
                existing: entry.get().to_factory_erased(),
                incoming: incoming.to_factory_erased(),
            }),
            Entry::Vacant(entry) => {
                entry.insert(incoming);
                Ok(())
            }
        }
    }

    pub fn find(&self, find_custom: bool, placeholder: impl AsRef<str>) -> Result<&PatternKind<F>> {
        let placeholder = placeholder.as_ref();

        match self.formatters.get(placeholder) {
            Some(found) => match (found, find_custom) {
                (PatternKind::BuiltIn(_), false) => Ok(found),
                (PatternKind::Custom { .. }, true) => Ok(found),
                (PatternKind::BuiltIn(_), true) => {
                    Err(Error::Template(TemplateError::WrongPatternKindReference {
                        is_builtin_as_custom: true,
                        placeholder: placeholder.into(),
                    }))
                }
                (PatternKind::Custom { .. }, false) => {
                    Err(Error::Template(TemplateError::WrongPatternKindReference {
                        is_builtin_as_custom: false,
                        placeholder: placeholder.into(),
                    }))
                }
            },
            None => Err(Error::Template(TemplateError::UnknownPatternReference {
                is_custom: find_custom,
                placeholder: placeholder.into(),
            })),
        }
    }
}

impl<F> PatternRegistry<F> {
    pub(crate) fn register_builtin(&mut self, formatter: BuiltInFormatter) {
        match self
            .formatters
            .entry(Cow::Borrowed(formatter.placeholder()))
        {
            Entry::Occupied(_) => {
                impossible!("formatter={:?}", formatter)
            }
            Entry::Vacant(entry) => {
                entry.insert(PatternKind::BuiltIn(formatter));
            }
        }
    }
}

pub fn check_custom_pattern_names<N, I>(names: I) -> Result<()>
where
    N: AsRef<str> + Eq + PartialEq + Hash,
    I: IntoIterator<Item = N>,
{
    let mut seen_names: HashMap<N, usize> = HashMap::new();
    let mut result = Ok(());

    for name in names {
        if let Some(existing) = BuiltInFormatter::iter().find(|f| f.placeholder() == name.as_ref())
        {
            result = Error::push_err(
                result,
                Error::ConflictName {
                    existing: PatternKind::BuiltIn(existing),
                    incoming: PatternKind::Custom {
                        placeholder: Cow::Owned(name.as_ref().into()),
                        factory: (),
                    },
                },
            );
        }

        if let Some(seen_count) = seen_names.get_mut(&name) {
            *seen_count += 1;
            if *seen_count == 2 {
                let conflict_pattern = PatternKind::Custom {
                    placeholder: Cow::Owned(name.as_ref().into()),
                    factory: (),
                };
                result = Error::push_err(
                    result,
                    Error::ConflictName {
                        existing: conflict_pattern.clone(),
                        incoming: conflict_pattern,
                    },
                );
            }
        } else {
            seen_names.insert(name, 1);
        }
    }

    debug_assert!(seen_names.iter().all(|(_, seen_count)| *seen_count == 1) || result.is_err());

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_pattern_names_checker() {
        use check_custom_pattern_names as check;

        assert!(check(["a", "b"]).is_ok());
        assert_eq!(
            check(["a", "a"]),
            Err(Error::ConflictName {
                existing: PatternKind::Custom {
                    placeholder: "a".into(),
                    factory: ()
                },
                incoming: PatternKind::Custom {
                    placeholder: "a".into(),
                    factory: ()
                }
            })
        );
        assert_eq!(
            check(["a", "b", "a"]),
            Err(Error::ConflictName {
                existing: PatternKind::Custom {
                    placeholder: "a".into(),
                    factory: ()
                },
                incoming: PatternKind::Custom {
                    placeholder: "a".into(),
                    factory: ()
                }
            })
        );
        assert_eq!(
            check(["date"]),
            Err(Error::ConflictName {
                existing: PatternKind::BuiltIn(BuiltInFormatter::Date),
                incoming: PatternKind::Custom {
                    placeholder: "date".into(),
                    factory: ()
                }
            })
        );
        assert_eq!(
            check(["date", "a", "a"]),
            Err(Error::Multiple(vec![
                Error::ConflictName {
                    existing: PatternKind::BuiltIn(BuiltInFormatter::Date),
                    incoming: PatternKind::Custom {
                        placeholder: "date".into(),
                        factory: ()
                    }
                },
                Error::ConflictName {
                    existing: PatternKind::Custom {
                        placeholder: "a".into(),
                        factory: ()
                    },
                    incoming: PatternKind::Custom {
                        placeholder: "a".into(),
                        factory: ()
                    }
                }
            ]))
        );
        assert_eq!(
            check(["date", "a", "a", "a"]),
            Err(Error::Multiple(vec![
                Error::ConflictName {
                    existing: PatternKind::BuiltIn(BuiltInFormatter::Date),
                    incoming: PatternKind::Custom {
                        placeholder: "date".into(),
                        factory: ()
                    }
                },
                Error::ConflictName {
                    existing: PatternKind::Custom {
                        placeholder: "a".into(),
                        factory: ()
                    },
                    incoming: PatternKind::Custom {
                        placeholder: "a".into(),
                        factory: ()
                    }
                }
            ]))
        );
        assert_eq!(
            check(["b", "date", "a", "b", "a", "a"]),
            Err(Error::Multiple(vec![
                Error::ConflictName {
                    existing: PatternKind::BuiltIn(BuiltInFormatter::Date),
                    incoming: PatternKind::Custom {
                        placeholder: "date".into(),
                        factory: ()
                    }
                },
                Error::ConflictName {
                    existing: PatternKind::Custom {
                        placeholder: "b".into(),
                        factory: ()
                    },
                    incoming: PatternKind::Custom {
                        placeholder: "b".into(),
                        factory: ()
                    }
                },
                Error::ConflictName {
                    existing: PatternKind::Custom {
                        placeholder: "a".into(),
                        factory: ()
                    },
                    incoming: PatternKind::Custom {
                        placeholder: "a".into(),
                        factory: ()
                    }
                }
            ]))
        );
    }
}

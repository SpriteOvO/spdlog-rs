use std::{
    borrow::Cow,
    collections::{hash_map::Entry, HashMap},
    fmt::Debug,
};

use super::{error::TemplateError, BuiltInFormatter, Error, PatternKind, Result};
use crate::impossible;

#[derive(Clone, Debug)]
pub struct PatternRegistry<F> {
    formatters: HashMap<Cow<'static, str>, PatternKind<F>>,
}

impl<F> PatternRegistry<F> {
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

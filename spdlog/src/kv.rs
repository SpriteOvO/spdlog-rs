use std::borrow::Cow;

use value_bag::{OwnedValueBag, ValueBag};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KeyInner<'a> {
    Str(&'a str),
    StaticStr(&'static str),
}

// TODO: PartialEq
#[derive(Debug, Clone)]
pub struct Key<'a>(KeyInner<'a>);

impl<'a> Key<'a> {
    #[doc(hidden)]
    pub fn __from_static_str(key: &'static str) -> Self {
        Key(KeyInner::StaticStr(key))
    }

    fn from_str(key: &'a str) -> Self {
        Key(KeyInner::Str(key))
    }

    pub(crate) fn to_owned(&self) -> KeyOwned {
        let inner = match self.0 {
            KeyInner::Str(s) => KeyOwnedInner::CowStr(Cow::Owned(s.to_string())),
            KeyInner::StaticStr(s) => KeyOwnedInner::CowStr(Cow::Borrowed(s)),
        };
        KeyOwned(inner)
    }

    #[cfg(test)]
    pub(crate) fn inner(&self) -> KeyInner<'a> {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
enum KeyOwnedInner {
    CowStr(Cow<'static, str>),
}

#[derive(Debug, Clone)]
pub(crate) struct KeyOwned(KeyOwnedInner);

impl KeyOwned {
    pub(crate) fn as_ref(&self) -> Key {
        let inner = match &self.0 {
            KeyOwnedInner::CowStr(s) => match s {
                Cow::Borrowed(s) => KeyInner::StaticStr(s),
                Cow::Owned(s) => KeyInner::Str(s),
            },
        };
        Key(inner)
    }
}

pub type Value<'a> = ValueBag<'a>;
pub(crate) type ValueOwned = OwnedValueBag;

pub(crate) type Pair<'a> = (Key<'a>, Value<'a>);

#[cfg(feature = "log")]
pub(crate) struct LogCrateConverter<'a>(Vec<(Key<'a>, Value<'a>)>);

#[cfg(feature = "log")]
impl<'a> LogCrateConverter<'a> {
    pub(crate) fn new(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub(crate) fn finalize(self) -> Cow<'a, [Pair<'a>]> {
        Cow::Owned(self.0)
    }
}

#[cfg(feature = "log")]
impl<'a> log::kv::VisitSource<'a> for LogCrateConverter<'a> {
    fn visit_pair(
        &mut self,
        key: log::kv::Key<'a>,
        value: log::kv::Value<'a>,
    ) -> Result<(), log::kv::Error> {
        self.0.push((
            Key::from_str(key.as_str()),
            todo!("convert `lov::kv::Value` to `Value`"),
        ));
        Ok(())
    }
}

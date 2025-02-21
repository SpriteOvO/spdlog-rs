use std::{borrow::Cow, fmt, slice};

use value_bag::{OwnedValueBag, ValueBag};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KeyInner<'a> {
    Str(&'a str),
    StaticStr(&'static str),
}

#[derive(Debug, Clone)]
pub struct Key<'a>(KeyInner<'a>);

impl Key<'_> {
    pub fn as_str(&self) -> &str {
        match &self.0 {
            KeyInner::Str(s) => s,
            KeyInner::StaticStr(s) => s,
        }
    }
}

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

impl PartialEq for Key<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
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

enum KeyValuesInner<'a> {
    Borrowed(&'a [Pair<'a>]),
    Owned(&'a [(KeyOwned, ValueOwned)]),
}
enum KeyValuesIterInner<'a> {
    Borrowed(slice::Iter<'a, Pair<'a>>),
    Owned(slice::Iter<'a, (KeyOwned, ValueOwned)>),
}

pub struct KeyValues<'a>(KeyValuesInner<'a>);

impl<'a> KeyValues<'a> {
    pub fn len(&self) -> usize {
        match self.0 {
            KeyValuesInner::Borrowed(p) => p.len(),
            KeyValuesInner::Owned(p) => p.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.0 {
            KeyValuesInner::Borrowed(p) => p.is_empty(),
            KeyValuesInner::Owned(p) => p.is_empty(),
        }
    }

    pub fn iter(&self) -> KeyValuesIter<'a> {
        match &self.0 {
            KeyValuesInner::Borrowed(p) => KeyValuesIter(KeyValuesIterInner::Borrowed(p.iter())),
            KeyValuesInner::Owned(p) => KeyValuesIter(KeyValuesIterInner::Owned(p.iter())),
        }
    }

    pub(crate) fn with_borrowed(pairs: &'a [Pair<'a>]) -> Self {
        Self(KeyValuesInner::Borrowed(pairs))
    }

    pub(crate) fn with_owned(pairs: &'a [(KeyOwned, ValueOwned)]) -> Self {
        Self(KeyValuesInner::Owned(pairs))
    }

    pub(crate) fn write_to(&self, dest: &mut impl fmt::Write, leading_space: bool) -> fmt::Result {
        let mut iter = self.iter();
        let first = iter.next();
        if let Some((key, value)) = first {
            if leading_space {
                dest.write_str(" { ")?;
            } else {
                dest.write_str("{ ")?;
            }

            // Reduce branch prediction misses for performance
            // So we manually process the first KV pair
            dest.write_str(key.as_str())?;
            dest.write_str("=")?;
            write!(dest, "{}", value)?;

            for (key, value) in iter {
                dest.write_str(", ")?;
                dest.write_str(key.as_str())?;
                dest.write_str("=")?;
                write!(dest, "{}", value)?;
            }

            dest.write_str(" }")?;
        }
        Ok(())
    }
}

impl<'a> IntoIterator for KeyValues<'a> {
    type Item = Pair<'a>;
    type IntoIter = KeyValuesIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct KeyValuesIter<'a>(KeyValuesIterInner<'a>);

impl<'a> Iterator for KeyValuesIter<'a> {
    type Item = Pair<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            // The 2 clones should be cheap
            KeyValuesIterInner::Borrowed(iter) => iter.next().map(|(k, v)| (k.clone(), v.clone())),
            KeyValuesIterInner::Owned(iter) => iter.next().map(|(k, v)| (k.as_ref(), v.by_ref())),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.0 {
            KeyValuesIterInner::Borrowed(iter) => iter.size_hint(),
            KeyValuesIterInner::Owned(iter) => iter.size_hint(),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_partial_eq() {
        assert_eq!(Key::from_str("a"), Key::__from_static_str("a"));
        assert_ne!(Key::from_str("a"), Key::__from_static_str("b"));
    }
}

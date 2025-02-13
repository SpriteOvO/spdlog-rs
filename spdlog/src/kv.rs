use std::{borrow::Cow, fmt, marker::PhantomData};

use value_bag::{OwnedValueBag, ValueBag};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KeyInner<'a> {
    Str(&'a str),
    StaticStr(&'static str),
}

// TODO: PartialEq
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

pub struct KeyValuesIter<'a, I> {
    iter: I,
    len: usize,
    phantom: PhantomData<&'a ()>,
}

impl<I> KeyValuesIter<'_, I> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<'a, I> KeyValuesIter<'a, I>
where
    I: Iterator<Item = (Key<'a>, Value<'a>)>,
{
    pub(crate) fn new(iter: I, len: usize) -> Self {
        Self {
            iter,
            len,
            phantom: PhantomData,
        }
    }

    pub(crate) fn write_to(
        mut self,
        dest: &mut impl fmt::Write,
        leading_space: bool,
    ) -> fmt::Result {
        let first = self.next();
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

            for (key, value) in self {
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

impl<'a, I> Iterator for KeyValuesIter<'a, I>
where
    I: Iterator<Item = (Key<'a>, Value<'a>)>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
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

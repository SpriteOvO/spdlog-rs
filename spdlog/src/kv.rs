//! Structured logging.
//!
//! Structured logging enhances traditional text-based log records with
//! user-defined attributes. Structured logs can be analyzed using a variety of
//! data processing techniques, without needing to find and parse attributes
//! from unstructured text first.
//!
//! [`Key`]s are strings and [`Value`]s are a datum of any type that can be
//! formatted or serialized. Simple types like strings, booleans, and numbers
//! are supported, as well as arbitrarily complex structures involving nested
//! objects and sequences.
//!
//! [`Value`] uses [_value-bag_ crate] as the backend, which is an alias of
//! [`value_bag::ValueBag`].
//!
//! KVs will be passed into a [`Record`] to be processed by [`Formatter`]s via
//! [`Record::key_values`] method.
//!
//! ## Examples
//!
//! #### Basic syntax
//!
//! In logging macros, an optional named parameter `kv` (like `logger`) is used
//! to add key-values to a log.
//!
//! ```
//! # use spdlog::prelude::*;
//! info!("program started", kv: { pid = std::process::id() });
//!
//! # let telemetry = spdlog::default_logger();
//! trace!(logger: telemetry, "user logged in", kv: { username = "John" });
//!
//! let ip = "1.1.1.1";
//! trace!("DNS setup", kv: { ip });
//! //                        ^^ Shorthand syntax, equivalent to `ip = ip`
//! ```
//!
//! #### Modifier
//!
//! A value is stored directly with its type by default (after erasure, of
//! course), using _modifier_ if you want it to be stored in another format.
//!
//! | Modifier | Description                                                                           |
//! |----------|---------------------------------------------------------------------------------------|
//! |          | No modifier, capture the value directly                                               |
//! | `:`      | Capture the value using [`Display`] trait                                             |
//! | `:?`     | Capture the value using [`Debug`] trait                                               |
//! | `:sval`  | Capture the value using [`sval::Value`] trait, crate feature `sval` is required       |
//! | `:serde` | Capture the value using [`serde::Serialize`] trait, crate feature `serde` is required |
//!
//! ```
//! # use spdlog::prelude::*;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # struct Url;
//! # impl Url { fn parse(_: &str) -> Result<Self, Box<dyn std::error::Error>> { Ok(Self) } }
//! # impl std::fmt::Display for Url {
//! #     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//! #         write!(f, "")
//! #     }
//! # }
//! let url = Url::parse("https://example.com")?;
//! trace!("user browsed website", kv: { url: });
//! //                                      ^ Capture the value using `Display` trait
//! //                                   ^^^^ Shorthand syntax, equivalent to `url: = url`
//!
//! let orders = vec!["coffee", "pizza", "soup"];
//! info!("order received", kv: { orders:? });
//! //                                  ^^ Capture the value using `Debug` trait
//! //                            ^^^^^^^^ Shorthand syntax, equivalent to `orders:? = orders`
//!
//! #[derive(sval_derive::Value)]
//! struct Point { x: f32, y: f32 }
//!
//! let pos = Point { x: 11.4, y: 5.14 };
//! # #[cfg(feature = "sval")]
//! trace!("user clicked", kv: { pos:sval });
//! //                              ^^^^^ Capture the value using `sval::Value` trait
//! //                           ^^^^^^^^ Shorthand syntax, equivalent to `pos:sval = pos`
//! # Ok(()) }
//! ```
//! [_value-bag_ crate]: https://crates.io/crates/value-bag
//! [`Record`]: crate::Record
//! [`Formatter`]: crate::formatter::Formatter
//! [`Display`]: std::fmt::Display
//! [`Record::key_values`]: crate::Record::key_values
//!
//! [`sval::Value`]: https://docs.rs/sval/latest/sval/trait.Value.html
// TODO: This above link annotation is unnecessary, but Rustdoc has bug:
//       https://github.com/rust-lang/cargo/issues/3475
//       Remove it when the bug is fixed.
use std::{borrow::Cow, fmt, slice};

use value_bag::{OwnedValueBag, ValueBag};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KeyInner<'a> {
    Str(&'a str),
    StaticStr(&'static str),
}

/// Represents a key in a key-value pair.
#[derive(Debug, Clone)]
pub struct Key<'a>(KeyInner<'a>);

impl Key<'_> {
    /// Gets the key string.
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

    pub(crate) fn from_str(key: &'a str) -> Self {
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

/// Represents a value in a key-value pair.
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

/// Represents a collection of key-value pairs.
///
/// ## Examples
///
/// ```
/// use std::fmt::Write;
/// use spdlog::{
///     formatter::{Formatter, FormatterContext},
///     Record, StringBuf,
/// };
///
/// #[derive(Clone)]
/// struct MyFormatter;
///
/// impl Formatter for MyFormatter {
///     fn format(
///         &self,
///         record: &Record,
///         dest: &mut StringBuf,
///         ctx: &mut FormatterContext,
///     ) -> spdlog::Result<()> {
///         dest.write_str(record.payload())
///             .map_err(spdlog::Error::FormatRecord)?;
///         for (key, value) in record.key_values() {
///             write!(dest, " {}={}", key.as_str(), value).map_err(spdlog::Error::FormatRecord)?;
///         }
///         Ok(())
///     }
/// }
/// ```
pub struct KeyValues<'a>(KeyValuesInner<'a>);

impl<'a> KeyValues<'a> {
    /// Gets the number of key-value pairs.
    pub fn len(&self) -> usize {
        match self.0 {
            KeyValuesInner::Borrowed(p) => p.len(),
            KeyValuesInner::Owned(p) => p.len(),
        }
    }

    /// Checks if there are no key-value pairs.
    pub fn is_empty(&self) -> bool {
        match self.0 {
            KeyValuesInner::Borrowed(p) => p.is_empty(),
            KeyValuesInner::Owned(p) => p.is_empty(),
        }
    }

    /// Gets the value of the specified key.
    pub fn get(&self, key: Key) -> Option<Value<'a>> {
        match self.0 {
            KeyValuesInner::Borrowed(p) => {
                p.iter()
                    .find_map(|(k, v)| if k == &key { Some(v.clone()) } else { None })
            }
            KeyValuesInner::Owned(p) => p.iter().find_map(|(k, v)| {
                if k.as_ref() == key {
                    Some(v.by_ref())
                } else {
                    None
                }
            }),
        }
    }

    /// Gets an iterator over the key-value pairs.
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

/// Represents an iterator over key-value pairs.
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
pub(crate) struct LogCrateConverter<'a>(Vec<(log::kv::Key<'a>, ValueOwned)>);

#[cfg(feature = "log")]
impl<'a> LogCrateConverter<'a> {
    pub(crate) fn new(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub(crate) fn finalize(self) -> Vec<(log::kv::Key<'a>, ValueOwned)> {
        self.0
    }
}

#[cfg(feature = "log")]
impl<'a> log::kv::VisitSource<'a> for LogCrateConverter<'a> {
    fn visit_pair(
        &mut self,
        key: log::kv::Key<'a>,
        value: log::kv::Value<'a>,
    ) -> Result<(), log::kv::Error> {
        struct Visitor(Option<ValueOwned>);

        macro_rules! visit_fn {
            ( $($fn:ident: $ty:ty => $from:ident),+$(,)? ) => {
                $(fn $fn(&mut self, value: $ty) -> Result<(), log::kv::Error> {
                    self.0 = Some(Value::$from(value).to_owned());
                    Ok(())
                })+
            };
        }

        impl log::kv::VisitValue<'_> for Visitor {
            fn visit_any(&mut self, value: log::kv::Value) -> Result<(), log::kv::Error> {
                // Since we have no way to extract the underlying `&dyn Display`, we have to
                // `to_owned()` here
                self.0 = Some(Value::from_display(&value).to_owned());
                Ok(())
            }

            fn visit_null(&mut self) -> Result<(), log::kv::Error> {
                self.0 = Some(Value::empty().to_owned());
                Ok(())
            }

            visit_fn! {
                visit_u64: u64 => from_u64,
                visit_i64: i64 => from_i64,
                visit_u128: u128 => from_u128,
                visit_i128: i128 => from_i128,
                visit_f64: f64 => from_f64,
                visit_bool: bool => from_bool,
                visit_str: &str => from_str,
                visit_borrowed_str: &str => from_str,
                visit_char: char => from_char,
            }
        }

        let mut visitor = Visitor(None);
        value.visit(&mut visitor)?;
        self.0.push((key, visitor.0.unwrap()));
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

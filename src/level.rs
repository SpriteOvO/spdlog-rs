//! Provides stuff related to log levels

use std::{cmp, fmt, str::FromStr};

use crate::Error;

pub(crate) const LOG_LEVEL_NAMES: [&str; 7] =
    ["off", "critical", "error", "warn", "info", "debug", "trace"];

/// An enum representing the available verbosity levels of the logger.
///
/// Typical usage includes: specifying the `Level` of [`log!`], and comparing a
/// `Level` directly to a [`LevelFilter`].
///
/// [`log!`]: crate::log
#[repr(usize)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Level {
    /// The "critical" level.
    ///
    /// Designates critical errors.
    // This way these line up with the discriminants for LevelFilter below
    // This works because Rust treats field-less enums the same way as C does:
    // https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-field-less-enumerations
    Critical = 1,
    /// The "error" level.
    ///
    /// Designates very serious errors.
    Error,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn,
    /// The "info" level.
    ///
    /// Designates useful information.
    Info,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug,
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace,
}

impl Level {
    fn from_usize(u: usize) -> Option<Level> {
        match u {
            1 => Some(Level::Critical),
            2 => Some(Level::Error),
            3 => Some(Level::Warn),
            4 => Some(Level::Info),
            5 => Some(Level::Debug),
            6 => Some(Level::Trace),
            _ => None,
        }
    }

    /// Returns the most verbose logging level.
    pub fn max() -> Level {
        Level::Trace
    }

    /// Converts the `Level` to the equivalent [`LevelFilter`].
    pub fn to_level_filter(&self) -> LevelFilter {
        LevelFilter::from_usize(*self as usize).unwrap()
    }

    /// Returns the string representation of the `Level`.
    ///
    /// This returns the same string as the `fmt::Display` implementation.
    pub fn as_str(&self) -> &'static str {
        LOG_LEVEL_NAMES[*self as usize]
    }

    /// Iterate through all supported logging levels.
    ///
    /// The order of iteration is from more severe to less severe log messages.
    ///
    /// # Examples
    ///
    /// ```
    /// use spdlog::Level;
    ///
    /// let mut levels = Level::iter();
    ///
    /// assert_eq!(Some(Level::Critical), levels.next());
    /// assert_eq!(Some(Level::Trace), levels.last());
    /// ```
    pub fn iter() -> impl Iterator<Item = Self> {
        (1..=Self::max() as usize).map(|i| Self::from_usize(i).unwrap())
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Level {
    type Err = Error;

    fn from_str(level: &str) -> Result<Level, Self::Err> {
        LOG_LEVEL_NAMES
            .iter()
            .position(|&name| name.eq_ignore_ascii_case(level))
            .into_iter()
            .filter(|&idx| idx != 0)
            .map(|idx| Level::from_usize(idx).unwrap())
            .next()
            .ok_or_else(|| Error::ParseLevel(level.to_string()))
    }
}

impl PartialEq<LevelFilter> for Level {
    fn eq(&self, other: &LevelFilter) -> bool {
        *self as usize == *other as usize
    }
}

impl PartialOrd<LevelFilter> for Level {
    fn partial_cmp(&self, other: &LevelFilter) -> Option<cmp::Ordering> {
        Some((*self as usize).cmp(&(*other as usize)))
    }

    fn lt(&self, other: &LevelFilter) -> bool {
        (*self as usize) < *other as usize
    }

    fn le(&self, other: &LevelFilter) -> bool {
        *self as usize <= *other as usize
    }

    fn gt(&self, other: &LevelFilter) -> bool {
        *self as usize > *other as usize
    }

    fn ge(&self, other: &LevelFilter) -> bool {
        *self as usize >= *other as usize
    }
}

/// An enum representing the available verbosity level filters of the logger.
///
/// A `LevelFilter` may be compared directly to a [`Level`].
#[repr(usize)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum LevelFilter {
    /// A level lower than all log levels.
    Off,
    /// Corresponds to the `Critical` log level.
    Critical,
    /// Corresponds to the `Error` log level.
    Error,
    /// Corresponds to the `Warn` log level.
    Warn,
    /// Corresponds to the `Info` log level.
    Info,
    /// Corresponds to the `Debug` log level.
    Debug,
    /// Corresponds to the `Trace` log level.
    Trace,
}

impl LevelFilter {
    fn from_usize(u: usize) -> Option<LevelFilter> {
        match u {
            0 => Some(LevelFilter::Off),
            1 => Some(LevelFilter::Critical),
            2 => Some(LevelFilter::Error),
            3 => Some(LevelFilter::Warn),
            4 => Some(LevelFilter::Info),
            5 => Some(LevelFilter::Debug),
            6 => Some(LevelFilter::Trace),
            _ => None,
        }
    }

    /// Returns the most verbose logging level filter.
    pub fn max() -> LevelFilter {
        LevelFilter::Trace
    }

    /// Converts the `LevelFilter` to the equivalent [`Level`].
    ///
    /// Returns `None` if `self` is `LevelFilter::Off`.
    pub fn to_level(&self) -> Option<Level> {
        Level::from_usize(*self as usize)
    }

    /// Returns the string representation of the `LevelFilter`.
    ///
    /// This returns the same string as the `fmt::Display` implementation.
    pub fn as_str(&self) -> &'static str {
        LOG_LEVEL_NAMES[*self as usize]
    }

    /// Iterate through all supported filtering levels.
    ///
    /// The order of iteration is from less to more verbose filtering.
    ///
    /// # Examples
    ///
    /// ```
    /// use spdlog::LevelFilter;
    ///
    /// let mut levels = LevelFilter::iter();
    ///
    /// assert_eq!(Some(LevelFilter::Off), levels.next());
    /// assert_eq!(Some(LevelFilter::Trace), levels.last());
    /// ```
    pub fn iter() -> impl Iterator<Item = Self> {
        (0..=Self::max() as usize).map(|i| Self::from_usize(i).unwrap())
    }
}

impl fmt::Display for LevelFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for LevelFilter {
    type Err = Error;

    fn from_str(level: &str) -> Result<LevelFilter, Self::Err> {
        LOG_LEVEL_NAMES
            .iter()
            .position(|&name| name.eq_ignore_ascii_case(level))
            .map(|p| LevelFilter::from_usize(p).unwrap())
            .ok_or_else(|| Error::ParseLevel(level.to_string()))
    }
}

impl PartialEq<Level> for LevelFilter {
    fn eq(&self, other: &Level) -> bool {
        other.eq(self)
    }
}

impl PartialOrd<Level> for LevelFilter {
    fn partial_cmp(&self, other: &Level) -> Option<cmp::Ordering> {
        Some((*self as usize).cmp(&(*other as usize)))
    }

    fn lt(&self, other: &Level) -> bool {
        (*self as usize) < *other as usize
    }

    fn le(&self, other: &Level) -> bool {
        *self as usize <= *other as usize
    }

    fn gt(&self, other: &Level) -> bool {
        *self as usize > *other as usize
    }

    fn ge(&self, other: &Level) -> bool {
        *self as usize >= *other as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consistency() {
        assert_eq!(Level::max() as usize, LevelFilter::max() as usize);
        assert_eq!(Level::max() as usize + 1, LOG_LEVEL_NAMES.len());

        for i in 0..=Level::max() as usize {
            let (level, level_filter) = (Level::from_usize(i), LevelFilter::from_usize(i));

            if i == 0 {
                assert!(level.is_none());
                assert!(level_filter.is_some());

                let level_filter = level_filter.unwrap();

                assert!(level_filter.to_level().is_none());
                assert_eq!(level_filter.as_str().to_lowercase(), "off");
            } else {
                let (level, level_filter) = (level.unwrap(), level_filter.unwrap());

                assert_eq!(level as usize, level_filter as usize);
                assert_eq!(level.as_str(), level_filter.as_str());
                assert_eq!(level.to_level_filter(), level_filter);
                assert_eq!(level_filter.to_level().unwrap(), level);
            }
        }
    }

    #[test]
    fn from_str() {
        fn to_random_case(input: &str) -> String {
            input
                .char_indices()
                .map(|(i, char)| {
                    if i % 2 != 0 {
                        char.to_ascii_uppercase()
                    } else {
                        char.to_ascii_lowercase()
                    }
                })
                .collect::<String>()
        }

        for (i, &name) in LOG_LEVEL_NAMES.iter().enumerate() {
            let from_usize = Level::from_usize(i);
            let from_str = (
                Level::from_str(&name.to_lowercase()),
                Level::from_str(&name.to_uppercase()),
                Level::from_str(&to_random_case(name)),
            );

            if i == 0 {
                assert!(from_usize.is_none());
                assert!(from_str.0.is_err());
                assert!(from_str.1.is_err());
                assert!(from_str.2.is_err());
            } else {
                assert_eq!(from_usize.unwrap(), from_str.0.unwrap());
                assert_eq!(from_usize.unwrap(), from_str.1.unwrap());
                assert_eq!(from_usize.unwrap(), from_str.2.unwrap());
            }
        }

        for (i, &name) in LOG_LEVEL_NAMES.iter().enumerate() {
            let from_usize = LevelFilter::from_usize(i).unwrap();
            let from_str = (
                LevelFilter::from_str(&name.to_lowercase()).unwrap(),
                LevelFilter::from_str(&name.to_uppercase()).unwrap(),
                LevelFilter::from_str(&to_random_case(name)).unwrap(),
            );

            assert_eq!(from_usize, from_str.0);
            assert_eq!(from_usize, from_str.1);
            assert_eq!(from_usize, from_str.2);
        }

        assert!(Level::from_str("notexist").is_err());
        assert!(LevelFilter::from_str("notexisttoo").is_err());
    }

    #[test]
    fn iter() {
        let mut iter = Level::iter();
        assert_eq!(iter.next(), Some(Level::Critical));
        assert_eq!(iter.next(), Some(Level::Error));
        assert_eq!(iter.next(), Some(Level::Warn));
        assert_eq!(iter.next(), Some(Level::Info));
        assert_eq!(iter.next(), Some(Level::Debug));
        assert_eq!(iter.next(), Some(Level::Trace));
        assert_eq!(iter.next(), None);

        let mut iter = LevelFilter::iter();
        assert_eq!(iter.next(), Some(LevelFilter::Off));
        assert_eq!(iter.next(), Some(LevelFilter::Critical));
        assert_eq!(iter.next(), Some(LevelFilter::Error));
        assert_eq!(iter.next(), Some(LevelFilter::Warn));
        assert_eq!(iter.next(), Some(LevelFilter::Info));
        assert_eq!(iter.next(), Some(LevelFilter::Debug));
        assert_eq!(iter.next(), Some(LevelFilter::Trace));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn eq_ord() {
        for i in 1..=Level::max() as usize {
            let (level, level_filter) = (
                Level::from_usize(i).unwrap(),
                LevelFilter::from_usize(i).unwrap(),
            );

            assert!(level == level_filter);
            assert!(level_filter == level);
        }

        for i in 2..Level::max() as usize {
            let level = (
                Level::from_usize(i - 1).unwrap(),
                Level::from_usize(i).unwrap(),
                Level::from_usize(i + 1).unwrap(),
            );

            let level_filter = (
                LevelFilter::from_usize(i - 1).unwrap(),
                LevelFilter::from_usize(i).unwrap(),
                LevelFilter::from_usize(i + 1).unwrap(),
            );

            assert!(level.0 == level_filter.0);
            assert!(level.1 == level_filter.1);
            assert!(level.2 == level_filter.2);

            assert!(level.0 < level_filter.1);
            assert!(level.1 < level_filter.2);

            assert!(level.1 > level_filter.0);
            assert!(level.2 > level_filter.1);
        }
    }
}

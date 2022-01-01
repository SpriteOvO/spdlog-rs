//! Provides stuff related to log levels

use std::{fmt, str::FromStr};

use crate::Error;

pub(crate) const LOG_LEVEL_NAMES: [&str; Level::count()] =
    ["critical", "error", "warn", "info", "debug", "trace"];

/// An enum representing the available verbosity levels of the logger.
///
/// Typical usage includes: specifying the `Level` of [`log!`], and comparing a
/// `Level` directly to a [`LevelFilter`].
///
/// [`log!`]: crate::log
#[repr(usize)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Level {
    /// The "critical" level.
    ///
    /// Designates critical errors.
    Critical = 0,
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
            0 => Some(Level::Critical),
            1 => Some(Level::Error),
            2 => Some(Level::Warn),
            3 => Some(Level::Info),
            4 => Some(Level::Debug),
            5 => Some(Level::Trace),
            _ => None,
        }
    }

    const fn min_usize() -> usize {
        Self::most_severe() as usize
    }

    const fn max_usize() -> usize {
        Self::most_verbose() as usize
    }

    pub(crate) const fn count() -> usize {
        Self::max_usize() + 1
    }

    /// Returns the most severe logging level.
    pub const fn most_severe() -> Level {
        Level::Critical
    }

    /// Returns the most verbose logging level.
    pub const fn most_verbose() -> Level {
        Level::Trace
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
        (Self::min_usize()..=Self::max_usize()).map(|i| Self::from_usize(i).unwrap())
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
            .map(|idx| Level::from_usize(idx).unwrap())
            .next()
            .ok_or_else(|| Error::ParseLevel(level.to_string()))
    }
}

/// An enum representing the available verbosity level filters of the logger.
///
/// A `LevelFilter` may be compared directly to a [`Level`].
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum LevelFilter {
    /// Disables all log levels.
    Off,
    /// Enables if the target level is equal to the filter level.
    Equal(Level),
    /// Enables if the target level is not equal to the filter level.
    NotEqual(Level),
    /// Enables if the target level is more severe than the filter level.
    MoreSevere(Level),
    /// Enables if the target level is more severe than or equal to the filter level.
    MoreSevereEqual(Level),
    /// Enables if the target level is more verbose than the filter level.
    MoreVerbose(Level),
    /// Enables if the target level is more verbose than or equal to the filter level.
    MoreVerboseEqual(Level),
    /// Enables all log levels.
    All,
}

impl LevelFilter {
    /// Compares the log level filter condition
    pub fn compare(&self, level: Level) -> bool {
        self.__compare_const(level)
    }

    // Users should not use this function directly.
    #[doc(hidden)]
    pub const fn __compare_const(&self, level: Level) -> bool {
        let level_usize: usize = level as usize;

        match *self {
            Self::Off => false,
            Self::Equal(stored) => level_usize == stored as usize,
            Self::NotEqual(stored) => level_usize != stored as usize,
            Self::MoreSevere(stored) => level_usize < stored as usize,
            Self::MoreSevereEqual(stored) => level_usize <= stored as usize,
            Self::MoreVerbose(stored) => level_usize > stored as usize,
            Self::MoreVerboseEqual(stored) => level_usize >= stored as usize,
            Self::All => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_usize() {
        assert_eq!(
            Level::most_severe(),
            Level::from_usize(Level::min_usize()).unwrap()
        );

        assert_eq!(
            Level::most_verbose(),
            Level::from_usize(Level::max_usize()).unwrap()
        );
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

            assert_eq!(from_usize.unwrap(), from_str.0.unwrap());
            assert_eq!(from_usize.unwrap(), from_str.1.unwrap());
            assert_eq!(from_usize.unwrap(), from_str.2.unwrap());
        }

        assert!(Level::from_str("notexist").is_err());
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
    }

    #[test]
    fn filter() {
        assert!(!LevelFilter::Off.compare(Level::Trace));
        assert!(!LevelFilter::Off.compare(Level::Critical));
        assert!(!LevelFilter::Off.compare(Level::Warn));

        assert!(LevelFilter::Equal(Level::Error).compare(Level::Error));
        assert!(!LevelFilter::Equal(Level::Error).compare(Level::Warn));
        assert!(!LevelFilter::Equal(Level::Error).compare(Level::Critical));

        assert!(LevelFilter::NotEqual(Level::Error).compare(Level::Trace));
        assert!(LevelFilter::NotEqual(Level::Error).compare(Level::Info));
        assert!(!LevelFilter::NotEqual(Level::Error).compare(Level::Error));

        assert!(LevelFilter::MoreSevere(Level::Info).compare(Level::Warn));
        assert!(LevelFilter::MoreSevere(Level::Info).compare(Level::Error));
        assert!(!LevelFilter::MoreSevere(Level::Info).compare(Level::Info));

        assert!(LevelFilter::MoreSevereEqual(Level::Info).compare(Level::Warn));
        assert!(LevelFilter::MoreSevereEqual(Level::Info).compare(Level::Info));
        assert!(!LevelFilter::MoreSevereEqual(Level::Info).compare(Level::Trace));

        assert!(LevelFilter::MoreVerbose(Level::Error).compare(Level::Warn));
        assert!(LevelFilter::MoreVerbose(Level::Error).compare(Level::Info));
        assert!(!LevelFilter::MoreVerbose(Level::Error).compare(Level::Error));

        assert!(LevelFilter::MoreVerboseEqual(Level::Error).compare(Level::Warn));
        assert!(LevelFilter::MoreVerboseEqual(Level::Error).compare(Level::Error));
        assert!(!LevelFilter::MoreVerboseEqual(Level::Error).compare(Level::Critical));

        assert!(LevelFilter::All.compare(Level::Trace));
        assert!(LevelFilter::All.compare(Level::Critical));
        assert!(LevelFilter::All.compare(Level::Error));
    }
}

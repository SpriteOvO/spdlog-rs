use std::{
    fmt::{self, Debug},
    str::FromStr,
    sync::atomic::Ordering,
};

use crate::Error;

pub(crate) const LOG_LEVEL_NAMES: [&str; Level::count()] =
    ["critical", "error", "warn", "info", "debug", "trace"];

const LOG_LEVEL_SHORT_NAMES: [&str; Level::count()] = ["C", "E", "W", "I", "D", "T"];

/// Represents log levels.
///
/// Typical usage:
/// - specifying the `level` parameter of macro [`log!`];
/// - comparing a `Level` to a [`LevelFilter`] through [`LevelFilter::test`].
///
/// # Note
///
/// Users should never cast variants of this enum to integers for persistent
/// storage (e.g., configuration files), using [`Level::as_str`] instead,
/// because integers corresponding to variants may change in the future.
///
/// Do **not** do this:
/// ```
/// # use spdlog::prelude::*;
/// # fn save_to_config_file(_: u32) {}
/// # let level: Level = Level::Info;
/// let value = level as u32; // Never do numeric casting!
///
/// save_to_config_file(value);
/// ```
///
/// Instead:
/// ```
/// # use spdlog::prelude::*;
/// # fn save_to_config_file(_: &str) {}
/// # let level: Level = Level::Info;
/// let value = level.as_str();
///
/// save_to_config_file(value);
/// ```
///
/// # Examples
///
/// ```
/// use spdlog::prelude::*;
///
/// log!(Level::Info, "hello, world");
/// ```
///
/// [`log!`]: crate::log!
#[repr(u16)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, bytemuck::NoUninit)]
pub enum Level {
    /// Designates critical errors.
    Critical = 0,
    /// Designates very serious errors.
    Error,
    /// Designates hazardous situations.
    Warn,
    /// Designates useful information.
    Info,
    /// Designates lower priority information.
    Debug,
    /// Designates very low priority, often extremely verbose, information.
    Trace,
}

#[cfg(feature = "serde")]
impl serde::Serialize for Level {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl Level {
    #[must_use]
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

    #[must_use]
    const fn min_usize() -> usize {
        Self::most_severe() as usize
    }

    #[must_use]
    const fn max_usize() -> usize {
        Self::most_verbose() as usize
    }

    /// Returns the number of logging levels.
    #[must_use]
    pub const fn count() -> usize {
        Self::max_usize() + 1
    }

    /// Returns the most severe logging level.
    #[must_use]
    pub const fn most_severe() -> Level {
        Level::Critical
    }

    /// Returns the most verbose logging level.
    #[must_use]
    pub const fn most_verbose() -> Level {
        Level::Trace
    }

    /// Returns the string representation.
    ///
    /// This returns the same string as the `fmt::Display` implementation.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        LOG_LEVEL_NAMES[*self as usize]
    }

    #[must_use]
    pub(crate) fn as_short_str(&self) -> &'static str {
        LOG_LEVEL_SHORT_NAMES[*self as usize]
    }

    /// Iterates through all logging levels.
    ///
    /// The order of iteration is from more severe to more verbose.
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

#[cfg(feature = "log")]
impl From<log::Level> for Level {
    fn from(level: log::Level) -> Self {
        match level {
            log::Level::Error => Self::Error,
            log::Level::Warn => Self::Warn,
            log::Level::Info => Self::Info,
            log::Level::Debug => Self::Debug,
            log::Level::Trace => Self::Trace,
        }
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

#[repr(u16)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, bytemuck::NoUninit)]
enum LevelFilterDiscriminant {
    Off,
    Equal,
    NotEqual,
    MoreSevere,
    MoreSevereEqual,
    MoreVerbose,
    MoreVerboseEqual,
    All,
}

/// Represents log level logical filter conditions.
///
/// Use [`LevelFilter::test`] method to check if a [`Level`] satisfies the
/// filter condition.
#[repr(align(4))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum LevelFilter {
    /// Disables all levels.
    Off,
    /// Enables if the target level is equal to the filter level.
    Equal(Level),
    /// Enables if the target level is not equal to the filter level.
    NotEqual(Level),
    /// Enables if the target level is more severe than the filter level.
    MoreSevere(Level),
    /// Enables if the target level is more severe than or equal to the filter
    /// level.
    MoreSevereEqual(Level),
    /// Enables if the target level is more verbose than the filter level.
    MoreVerbose(Level),
    /// Enables if the target level is more verbose than or equal to the filter
    /// level.
    MoreVerboseEqual(Level),
    /// Enables all levels.
    All,
}

impl LevelFilter {
    /// Checks the given level if satisfies the filter condition.
    #[deprecated(
        since = "0.4.0",
        note = "it may be removed in the future, use method `test()` instead"
    )]
    #[must_use]
    pub fn compare(&self, level: Level) -> bool {
        self.__test_const(level)
    }

    /// Checks the given level if satisfies the filter condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use spdlog::prelude::*;
    ///
    /// let level_filter = LevelFilter::MoreSevere(Level::Info);
    ///
    /// assert_eq!(level_filter.test(Level::Trace), false);
    /// assert_eq!(level_filter.test(Level::Info), false);
    /// assert_eq!(level_filter.test(Level::Warn), true);
    /// assert_eq!(level_filter.test(Level::Error), true);
    /// ```
    #[must_use]
    pub fn test(&self, level: Level) -> bool {
        self.__test_const(level)
    }

    // Users should not use this function directly.
    #[doc(hidden)]
    #[must_use]
    pub const fn __test_const(&self, level: Level) -> bool {
        let level_num: u16 = level as u16;

        match *self {
            Self::Off => false,
            Self::Equal(stored) => level_num == stored as u16,
            Self::NotEqual(stored) => level_num != stored as u16,
            Self::MoreSevere(stored) => level_num < stored as u16,
            Self::MoreSevereEqual(stored) => level_num <= stored as u16,
            Self::MoreVerbose(stored) => level_num > stored as u16,
            Self::MoreVerboseEqual(stored) => level_num >= stored as u16,
            Self::All => true,
        }
    }

    #[must_use]
    pub(crate) fn from_str_for_env(text: &str) -> Option<LevelFilter> {
        if let Ok(level) = Level::from_str(text) {
            Some(LevelFilter::MoreSevereEqual(level))
        } else if text.eq_ignore_ascii_case("off") {
            Some(LevelFilter::Off)
        } else if text.eq_ignore_ascii_case("all") {
            Some(LevelFilter::All)
        } else {
            None
        }
    }

    #[must_use]
    fn discriminant(&self) -> LevelFilterDiscriminant {
        match self {
            Self::Off => LevelFilterDiscriminant::Off,
            Self::Equal(_) => LevelFilterDiscriminant::Equal,
            Self::NotEqual(_) => LevelFilterDiscriminant::NotEqual,
            Self::MoreSevere(_) => LevelFilterDiscriminant::MoreSevere,
            Self::MoreSevereEqual(_) => LevelFilterDiscriminant::MoreSevereEqual,
            Self::MoreVerbose(_) => LevelFilterDiscriminant::MoreVerbose,
            Self::MoreVerboseEqual(_) => LevelFilterDiscriminant::MoreVerboseEqual,
            Self::All => LevelFilterDiscriminant::All,
        }
    }

    #[must_use]
    fn level(&self) -> Option<Level> {
        match *self {
            Self::Equal(level)
            | Self::NotEqual(level)
            | Self::MoreSevere(level)
            | Self::MoreSevereEqual(level)
            | Self::MoreVerbose(level)
            | Self::MoreVerboseEqual(level) => Some(level),
            Self::Off | Self::All => None,
        }
    }
}

#[cfg(feature = "log")]
impl From<log::LevelFilter> for LevelFilter {
    fn from(filter: log::LevelFilter) -> Self {
        match filter {
            log::LevelFilter::Off => Self::Off,
            filter => Self::MoreSevereEqual(Level::from(filter.to_level().unwrap())),
        }
    }
}

// Atomic

// This is a version of `LevelFilter` that does not contain uninitialized bytes.
// For variants like `LevelFilter::{Off,All}`, since they have no field, their
// field memory space is treated as uninitialized bytes. `Atomic<T>` from the
// `atomic` crate uses `std::mem::transmute`, and its operations on
// uninitialized bytes are undefined behavior. Therefore, we created this
// `LevelFilterLayout` type, where uninitialized bytes are filled with
// `UNDEFINED_FALLBACK`, enabling safe atomic operations.
#[repr(C, align(4))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, bytemuck::NoUninit)]
struct LevelFilterLayout {
    discriminant: LevelFilterDiscriminant,
    level: Level,
}

impl LevelFilterLayout {
    const UNDEFINED_FALLBACK: Level = Level::Critical;
}

impl From<LevelFilter> for LevelFilterLayout {
    fn from(value: LevelFilter) -> Self {
        Self {
            discriminant: value.discriminant(),
            level: value.level().unwrap_or(Self::UNDEFINED_FALLBACK),
        }
    }
}

impl From<LevelFilterLayout> for LevelFilter {
    fn from(layout: LevelFilterLayout) -> Self {
        match layout.discriminant {
            LevelFilterDiscriminant::Off => Self::Off,
            LevelFilterDiscriminant::Equal => Self::Equal(layout.level),
            LevelFilterDiscriminant::NotEqual => Self::NotEqual(layout.level),
            LevelFilterDiscriminant::MoreSevere => Self::MoreSevere(layout.level),
            LevelFilterDiscriminant::MoreSevereEqual => Self::MoreSevereEqual(layout.level),
            LevelFilterDiscriminant::MoreVerbose => Self::MoreVerbose(layout.level),
            LevelFilterDiscriminant::MoreVerboseEqual => Self::MoreVerboseEqual(layout.level),
            LevelFilterDiscriminant::All => Self::All,
        }
    }
}

/// Atomic version of [`LevelFilter`] which can be safely shared between
/// threads.
pub struct AtomicLevelFilter {
    inner: atomic::Atomic<LevelFilterLayout>,
}

impl AtomicLevelFilter {
    const ORDERING: Ordering = Ordering::Relaxed;

    /// Creates a new `AtomicLevelFilter`.
    #[must_use]
    pub fn new(init: LevelFilter) -> Self {
        Self {
            inner: atomic::Atomic::new(init.into()),
        }
    }

    /// Loads the level filter with `Relaxed` ordering.
    pub fn get(&self) -> LevelFilter {
        self.load(Self::ORDERING)
    }

    /// Stores a level filter with `Relaxed` ordering.
    pub fn set(&self, new: LevelFilter) {
        self.store(new, Self::ORDERING);
    }

    /// Loads the level filter.
    ///
    /// # Panics
    ///
    /// Panics if the ordering is `Release` or `AcqRel`.
    pub fn load(&self, ordering: Ordering) -> LevelFilter {
        self.inner.load(ordering).into()
    }

    /// Stores a level filter.
    ///
    /// # Panics
    ///
    /// Panics if the ordering is `Acquire` or `AcqRel`.
    pub fn store(&self, value: LevelFilter, ordering: Ordering) {
        self.inner.store(value.into(), ordering);
    }

    /// Stores a level filter, returning the old level filter.
    pub fn swap(&self, new: LevelFilter, ordering: Ordering) -> LevelFilter {
        self.inner.swap(new.into(), ordering).into()
    }
}

impl Debug for AtomicLevelFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.get().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;
    use crate::utils::const_assert;

    const_assert!(atomic::Atomic::<Level>::is_lock_free());
    const_assert!(atomic::Atomic::<LevelFilter>::is_lock_free());
    const_assert!(atomic::Atomic::<LevelFilterLayout>::is_lock_free());
    const_assert!(size_of::<Level>() * 2 == size_of::<LevelFilter>());
    const_assert!(align_of::<Level>() * 2 == align_of::<LevelFilter>());
    const_assert!(size_of::<LevelFilter>() == size_of::<LevelFilterLayout>());
    const_assert!(align_of::<LevelFilter>() == align_of::<LevelFilterLayout>());

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
    fn as_short_str() {
        for (&name, &short_name) in LOG_LEVEL_NAMES.iter().zip(LOG_LEVEL_SHORT_NAMES.iter()) {
            assert_eq!(
                name.chars()
                    .next()
                    .unwrap()
                    .to_ascii_uppercase()
                    .to_string(),
                short_name
            );
        }
    }

    #[test]
    fn level_filter_from_str_for_env() {
        assert_eq!(
            LevelFilter::MoreSevereEqual(Level::Info),
            LevelFilter::from_str_for_env("iNFo").unwrap()
        );

        assert_eq!(
            LevelFilter::MoreSevereEqual(Level::Warn),
            LevelFilter::from_str_for_env("wARn").unwrap()
        );

        assert_eq!(
            LevelFilter::Off,
            LevelFilter::from_str_for_env("oFf").unwrap()
        );

        assert_eq!(
            LevelFilter::All,
            LevelFilter::from_str_for_env("aLl").unwrap()
        );
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
        assert!(!LevelFilter::Off.test(Level::Trace));
        assert!(!LevelFilter::Off.test(Level::Critical));
        assert!(!LevelFilter::Off.test(Level::Warn));

        assert!(LevelFilter::Equal(Level::Error).test(Level::Error));
        assert!(!LevelFilter::Equal(Level::Error).test(Level::Warn));
        assert!(!LevelFilter::Equal(Level::Error).test(Level::Critical));

        assert!(LevelFilter::NotEqual(Level::Error).test(Level::Trace));
        assert!(LevelFilter::NotEqual(Level::Error).test(Level::Info));
        assert!(!LevelFilter::NotEqual(Level::Error).test(Level::Error));

        assert!(LevelFilter::MoreSevere(Level::Info).test(Level::Warn));
        assert!(LevelFilter::MoreSevere(Level::Info).test(Level::Error));
        assert!(!LevelFilter::MoreSevere(Level::Info).test(Level::Info));

        assert!(LevelFilter::MoreSevereEqual(Level::Info).test(Level::Warn));
        assert!(LevelFilter::MoreSevereEqual(Level::Info).test(Level::Info));
        assert!(!LevelFilter::MoreSevereEqual(Level::Info).test(Level::Trace));

        assert!(LevelFilter::MoreVerbose(Level::Error).test(Level::Warn));
        assert!(LevelFilter::MoreVerbose(Level::Error).test(Level::Info));
        assert!(!LevelFilter::MoreVerbose(Level::Error).test(Level::Error));

        assert!(LevelFilter::MoreVerboseEqual(Level::Error).test(Level::Warn));
        assert!(LevelFilter::MoreVerboseEqual(Level::Error).test(Level::Error));
        assert!(!LevelFilter::MoreVerboseEqual(Level::Error).test(Level::Critical));

        assert!(LevelFilter::All.test(Level::Trace));
        assert!(LevelFilter::All.test(Level::Critical));
        assert!(LevelFilter::All.test(Level::Error));
    }

    #[cfg(feature = "log")]
    #[test]
    fn filter_from_log() {
        assert_eq!(LevelFilter::from(log::LevelFilter::Off), LevelFilter::Off);
        assert_eq!(
            LevelFilter::from(log::LevelFilter::Error),
            LevelFilter::MoreSevereEqual(Level::Error)
        );
        assert_eq!(
            LevelFilter::from(log::LevelFilter::Warn),
            LevelFilter::MoreSevereEqual(Level::Warn)
        );
        assert_eq!(
            LevelFilter::from(log::LevelFilter::Info),
            LevelFilter::MoreSevereEqual(Level::Info)
        );
        assert_eq!(
            LevelFilter::from(log::LevelFilter::Debug),
            LevelFilter::MoreSevereEqual(Level::Debug)
        );
        assert_eq!(
            LevelFilter::from(log::LevelFilter::Trace),
            LevelFilter::MoreSevereEqual(Level::Trace)
        );
    }

    #[test]
    fn atomic_level_filter() {
        let atomic_level_filter = AtomicLevelFilter::new(LevelFilter::All);

        let assert_this = |new: LevelFilter| {
            assert_ne!(atomic_level_filter.get(), new);
            atomic_level_filter.set(new);
            assert_eq!(atomic_level_filter.get(), new);
        };

        fn produce_all(cond: impl Fn(Level) -> LevelFilter) -> impl Iterator<Item = LevelFilter> {
            Level::iter().map(cond)
        }

        assert_this(LevelFilter::Off);
        produce_all(LevelFilter::Equal).for_each(assert_this);
        produce_all(LevelFilter::NotEqual).for_each(assert_this);
        produce_all(LevelFilter::MoreSevere).for_each(assert_this);
        produce_all(LevelFilter::MoreSevereEqual).for_each(assert_this);
        produce_all(LevelFilter::MoreVerbose).for_each(assert_this);
        produce_all(LevelFilter::MoreVerboseEqual).for_each(assert_this);
        assert_this(LevelFilter::All);
    }
}

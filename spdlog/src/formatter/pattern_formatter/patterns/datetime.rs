use std::{fmt::Write, marker::PhantomData};

use chrono::{Datelike, Timelike};

use crate::{
    formatter::{
        local_time_cacher::LOCAL_TIME_CACHER,
        pattern_formatter::{Pattern, PatternContext},
    },
    Error, Record, StringBuf,
};

/// A pattern that writes the abbreviated weekday name of log records into the
/// output. Example: `Mon`, `Tue`.
///
/// This pattern corresponds to `{a}` or `{weekday-name}` in the pattern
/// template string.
#[derive(Clone, Debug)]
pub struct AbbrWeekdayName {
    base: WeekdayNameBase,
}

impl AbbrWeekdayName {
    /// Create a new `AbbrWeekday` pattern.
    pub fn new() -> Self {
        Self {
            base: WeekdayNameBase::new(["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        }
    }
}

impl Default for AbbrWeekdayName {
    fn default() -> Self {
        Self::new()
    }
}

impl Pattern for AbbrWeekdayName {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        self.base.format(record, dest, ctx)
    }
}

/// A pattern that writes the weekday name of log records into the output.
/// Example: `Monday`, `Tuesday`.
///
/// This pattern corresponds to `{A}` or `{weekday-name-full}` in the pattern
/// template string.
#[derive(Clone, Debug)]
pub struct WeekdayName {
    base: WeekdayNameBase,
}

impl WeekdayName {
    /// Create a new `Weekday` pattern.
    pub fn new() -> Self {
        Self {
            base: WeekdayNameBase::new([
                "Monday",
                "Tuesday",
                "Wednesday",
                "Thursday",
                "Friday",
                "Saturday",
                "Sunday",
            ]),
        }
    }
}

impl Default for WeekdayName {
    fn default() -> Self {
        Self::new()
    }
}

impl Pattern for WeekdayName {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        self.base.format(record, dest, ctx)
    }
}

/// A pattern that writes the abbreviated month name of log records into the
/// output. Example: `Jan`, `Feb`.
///
/// This pattern corresponds to `{b}` or `{month-name}` in the pattern template
/// string.
#[derive(Clone, Debug)]
pub struct AbbrMonthName {
    base: MonthNameBase,
}

impl AbbrMonthName {
    /// Create a new `AbbrMonthName` pattern.
    pub fn new() -> Self {
        Self {
            base: MonthNameBase::new([
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
            ]),
        }
    }
}

impl Default for AbbrMonthName {
    fn default() -> Self {
        Self::new()
    }
}

impl Pattern for AbbrMonthName {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        self.base.format(record, dest, ctx)
    }
}

/// A pattern that writes the month name of log records into the output.
/// Example: `January`, `February`.
///
/// This pattern corresponds to `{B}` or `{month-name-full}` in the pattern
/// template string.
#[derive(Clone, Debug)]
pub struct MonthName {
    base: MonthNameBase,
}

impl MonthName {
    /// Create a new `MonthName` pattern.
    pub fn new() -> Self {
        Self {
            base: MonthNameBase::new([
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ]),
        }
    }
}

impl Default for MonthName {
    fn default() -> Self {
        Self::new()
    }
}

impl Pattern for MonthName {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        self.base.format(record, dest, ctx)
    }
}

/// A pattern that writes the full date time of log records into the output.
/// Example: `Thu Aug 23 15:35:46 2014`.
///
/// The full date time includes the following parts:
/// - Abbreviated weekday name as formatted by the [`AbbrWeekdayName`] pattern;
/// - Abbreviated month name as formatted by the [`AbbrMonthName`] pattern;
/// - Day of month;
/// - Time of day with second precision;
/// - Year.
///
/// This pattern corresponds to `{c}` or `{datetime}` in the pattern
/// template string.
#[derive(Clone, Debug, Default)]
pub struct FullDateTime {
    abbr_weekday: AbbrWeekdayName,
    abbr_month: AbbrMonthName,
}

impl FullDateTime {
    /// Create a new `FullDateTime` pattern.
    pub fn new() -> Self {
        Self {
            abbr_weekday: AbbrWeekdayName::new(),
            abbr_month: AbbrMonthName::new(),
        }
    }
}

impl Pattern for FullDateTime {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        self.abbr_weekday.format(record, dest, ctx)?;
        dest.write_str(" ").map_err(Error::FormatRecord)?;

        self.abbr_month.format(record, dest, ctx)?;
        dest.write_str(" ").map_err(Error::FormatRecord)?;

        let (day_str, hour_str, minute_str, second_str, year_str) = {
            let mut time_cacher_lock = LOCAL_TIME_CACHER.lock();
            let cached_time = time_cacher_lock.get(record.time());
            (
                cached_time.day_str(),
                cached_time.hour_str(),
                cached_time.minute_str(),
                cached_time.second_str(),
                cached_time.year_str(),
            )
        };

        write!(
            dest,
            "{} {}:{}:{} {}",
            day_str, hour_str, minute_str, second_str, year_str
        )
        .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the short year of log records into the output.
/// Examples: `22`, `20`.
///
/// This pattern corresponds to `{C}` or `{year-short}` in the pattern
/// template string.
#[derive(Clone, Debug, Default)]
pub struct ShortYear {
    /// This field prevents users from creating `ShortYear` objects literally.
    _phantom: PhantomData<()>,
}

impl ShortYear {
    /// Create a new `ShortYear` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for ShortYear {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let year_str = LOCAL_TIME_CACHER.lock().get(record.time()).year_str();
        let short_year_str = {
            let year_str_bytes = year_str.as_bytes();
            debug_assert_eq!(year_str_bytes.len(), 4);

            unsafe { std::str::from_utf8_unchecked(&year_str_bytes[2..4]) }
        };

        dest.write_str(short_year_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the year of log records into the output.
/// Examples: `2022`, `2021`.
///
/// This pattern corresponds to `{Y}` or `{year}` in the pattern
/// template string.
#[derive(Clone, Debug, Default)]
pub struct Year {
    /// This field prevents users from creating `Year` objects literally.
    _phantom: PhantomData<()>,
}

impl Year {
    /// Create a new `Year` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for Year {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let year_str = LOCAL_TIME_CACHER.lock().get(record.time()).year_str();
        dest.write_str(&**year_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the short date of log records in `MM/DD/YY` format
/// into the output. Examples: `04/01/22`, `12/31/21`.
///
/// This pattern corresponds to `{D}` or `{short-date}` in the pattern template
/// string.
#[derive(Clone, Debug, Default)]
pub struct ShortDate {
    _phantom: PhantomData<()>,
}

impl ShortDate {
    /// Create a new `ShortDate` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for ShortDate {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let (month_str, day_str, year_str) = {
            let mut local_cacher_lock = LOCAL_TIME_CACHER.lock();
            let cached_time = local_cacher_lock.get(record.time());
            (
                cached_time.month_str(),
                cached_time.day_str(),
                cached_time.year_str(),
            )
        };

        let short_year_str = {
            let year_str_bytes = year_str.as_bytes();
            debug_assert_eq!(year_str_bytes.len(), 4);

            unsafe { std::str::from_utf8_unchecked(&year_str_bytes[2..4]) }
        };

        write!(dest, "{}/{}/{}", month_str, day_str, short_year_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the month of log records into the output.
/// Examples: `01`, `12`.
///
/// This pattern corresponds to `{m}` or `{month}` in the pattern
/// template string.
#[derive(Clone, Debug, Default)]
pub struct Month {
    /// This field prevents users from creating `Month` objects literally.
    _phantom: PhantomData<()>,
}

impl Month {
    /// Create a new `Month` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for Month {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let month_str = LOCAL_TIME_CACHER.lock().get(record.time()).month_str();
        dest.write_str(&**month_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the day of log records into the output.
/// Examples: `01`, `12`, `31`, `30`.
///
/// This pattern corresponds to `{d}` or `{day}` in the pattern
/// template string.
#[derive(Clone, Debug, Default)]
pub struct Day {
    /// This field prevents users from creating `Day` objects literally.
    _phantom: PhantomData<()>,
}

impl Day {
    /// Create a new `Day` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for Day {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let day_str = LOCAL_TIME_CACHER.lock().get(record.time()).day_str();
        dest.write_str(&**day_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the hour of log records into the output. Examples:
/// `01`, `12`, `23`.
///
/// This pattern corresponds to `{H}` or `{hour}` in the pattern template
/// string.
#[derive(Clone, Debug, Default)]
pub struct Hour {
    _phantom: PhantomData<()>,
}

impl Hour {
    /// Create a new `Hour` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for Hour {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let hour_str = LOCAL_TIME_CACHER.lock().get(record.time()).hour_str();
        dest.write_str(&**hour_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the hour in 12-hour format of log records into the
/// output. Examples: `01`, `12`.
///
/// This pattern corresponds to `{I}` or `{hour-12}` in the pattern template
/// string.
#[derive(Clone, Debug, Default)]
pub struct Hour12 {
    _phantom: PhantomData<()>,
}

impl Hour12 {
    /// Create a new `Hour12` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for Hour12 {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let hour_12_str = LOCAL_TIME_CACHER.lock().get(record.time()).hour_12_str();
        dest.write_str(&**hour_12_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the minute of log records into the output. Examples:
/// `00`, `05`, `59`.
///
/// This pattern corresponds to `{M}` or `{minute}` in the pattern template
/// string.
#[derive(Clone, Debug, Default)]
pub struct Minute {
    _phantom: PhantomData<()>,
}

impl Minute {
    /// Create a new `Minutes` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for Minute {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let minute_str = LOCAL_TIME_CACHER.lock().get(record.time()).minute_str();
        dest.write_str(&**minute_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the second of log records into the output. Examples:
/// `00`, `05`, `59`.
///
/// This pattern corresponds to `{S}` or `{second}` in the pattern template
/// string.
#[derive(Clone, Debug, Default)]
pub struct Second {
    _phantom: PhantomData<()>,
}

impl Second {
    /// Create a new `Second` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for Second {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let second_str = LOCAL_TIME_CACHER.lock().get(record.time()).second_str();
        dest.write_str(&**second_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the millisecond part within a second of the timestamp
/// of a log record into the output. Example: `231`.
///
/// This pattern corresponds to `{e}` or `{millisecond}` in the pattern
/// template string.
#[derive(Clone, Debug, Default)]
pub struct Millisecond {
    /// This field prevents users from creating `Millisecond` objects
    /// literally.
    _phantom: PhantomData<()>,
}

impl Millisecond {
    /// Create a new `Millisecond` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for Millisecond {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let time = chrono::DateTime::<chrono::Local>::from(record.time());
        dest.write_fmt(format_args!("{:03}", time.nanosecond() / 1_000_000))
            .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the microsecond part within a second of the timestamp
/// of a log record into the output. Example: `372152`.
///
/// This pattern corresponds to `{f}` or `{microsecond}` in the pattern
/// template string.
#[derive(Clone, Debug, Default)]
pub struct Microsecond {
    /// This field prevents users from creating `Microsecond` objects
    /// literally.
    _phantom: PhantomData<()>,
}

impl Microsecond {
    /// Create a new `Microsecond` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for Microsecond {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let time = chrono::DateTime::<chrono::Local>::from(record.time());
        dest.write_fmt(format_args!("{:06}", time.nanosecond() / 1_000))
            .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the nanosecond part within a second of the timestamp
/// of a log record into the output. Example: `482930154`.
///
/// This pattern corresponds to `{F}` or `{nanosecond}` in the pattern
/// template string.
#[derive(Clone, Debug, Default)]
pub struct Nanosecond {
    /// This field prevents users from creating `Nanosecond` objects literally.
    _phantom: PhantomData<()>,
}

impl Nanosecond {
    /// Create a new `Nanosecond` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for Nanosecond {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let time = chrono::DateTime::<chrono::Local>::from(record.time());
        dest.write_fmt(format_args!("{:09}", time.nanosecond()))
            .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes "AM" or "PM" into the output according to the
/// timestamp of a log record. Example: `AM`, `PM`.
///
/// This pattern corresponds to `{p}` or `{ampm}` in the pattern
/// template string.
#[derive(Clone, Debug, Default)]
pub struct Ampm {
    /// This field prevents users from creating `Ampm` objects literally.
    _phantom: PhantomData<()>,
}

impl Ampm {
    /// Create a new `Ampm` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for Ampm {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let time = chrono::DateTime::<chrono::Local>::from(record.time());
        if time.hour12().0 {
            dest.write_str("PM").map_err(Error::FormatRecord)
        } else {
            dest.write_str("AM").map_err(Error::FormatRecord)
        }
    }
}

/// A pattern that writes the time of log records in 12-hour format into the
/// output. Examples: `02:55:02 PM`.
///
/// This pattern corresponds to `{r}` or `{time-12}` in the pattern template
/// string.
#[derive(Clone, Debug, Default)]
pub struct Time12 {
    ampm: Ampm,
}

impl Time12 {
    /// Create a new `Time12` pattern.
    pub fn new() -> Self {
        Self { ampm: Ampm::new() }
    }
}

impl Pattern for Time12 {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let (hour_str, minute_str, second_str) = {
            let mut time_cacher_lock = LOCAL_TIME_CACHER.lock();
            let cached_time = time_cacher_lock.get(record.time());
            (
                cached_time.hour_12_str(),
                cached_time.minute_str(),
                cached_time.second_str(),
            )
        };

        write!(dest, "{}:{}:{}", hour_str, minute_str, second_str).map_err(Error::FormatRecord)?;

        dest.write_str(" ").map_err(Error::FormatRecord)?;
        self.ampm.format(record, dest, ctx)?;

        Ok(())
    }
}

/// A pattern that writes the short time of log records into the output.
/// Examples: `22:28`, `09:53`.
///
/// This pattern corresponds to `{R}` or `{time-short}` in the pattern template
/// string.
#[derive(Clone, Debug, Default)]
pub struct ShortTime {
    _phantom: PhantomData<()>,
}

impl ShortTime {
    /// Create a new `ShortTime` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for ShortTime {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let (hour_str, minute_str) = {
            let mut time_cacher_lock = LOCAL_TIME_CACHER.lock();
            let cached_time = time_cacher_lock.get(record.time());
            (cached_time.hour_str(), cached_time.minute_str())
        };

        write!(dest, "{}:{}", hour_str, minute_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the time of log records into the output. Examples:
/// `22:28:02`, `09:53:41`.
///
/// This pattern corresponds to `{T}`, `{X}` or `{time}` in the pattern template
/// string.
#[derive(Clone, Debug, Default)]
pub struct Time {
    _phantom: PhantomData<()>,
}

impl Time {
    /// Create a new `Time` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for Time {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let (hour_str, minute_str, second_str) = {
            let mut time_cacher_lock = LOCAL_TIME_CACHER.lock();
            let cached_time = time_cacher_lock.get(record.time());
            (
                cached_time.hour_str(),
                cached_time.minute_str(),
                cached_time.second_str(),
            )
        };

        write!(dest, "{}:{}:{}", hour_str, minute_str, second_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the timezone offset of log records into the output.
/// Examples: `+08:00`, `+00:00`, `-06:00`.
///
/// This pattern corresponds to `{z}` or `{tz-offset}` in the pattern template
/// string.
#[derive(Clone, Debug, Default)]
pub struct TzOffset {
    _phantom: PhantomData<()>,
}

impl TzOffset {
    /// Create a new `TzOffset` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for TzOffset {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let tz_offset_str = LOCAL_TIME_CACHER.lock().get(record.time()).tz_offset_str();
        dest.write_str(&**tz_offset_str)
            .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the unix timestamp of log records into the output.
/// Examples: `1528834770`.
///
/// This pattern corresponds to `{E}` or `{unix}` in the pattern template
/// string.
pub struct UnixTimestamp {
    _phantom: PhantomData<()>,
}

impl UnixTimestamp {
    /// Create a new `UnixTimestamp` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for UnixTimestamp {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let unix_timestamp_str = LOCAL_TIME_CACHER
            .lock()
            .get(record.time())
            .unix_timestamp_str();
        dest.write_str(&**unix_timestamp_str)
            .map_err(Error::FormatRecord)
    }
}

#[derive(Clone, Debug)]
struct WeekdayNameBase {
    weekday_names: [&'static str; 7],
}

impl WeekdayNameBase {
    fn new(weekday_names: [&'static str; 7]) -> Self {
        Self { weekday_names }
    }
}

impl Pattern for WeekdayNameBase {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let local_time = chrono::DateTime::<chrono::Local>::from(record.time());
        dest.write_str(self.weekday_names[local_time.weekday().num_days_from_monday() as usize])
            .map_err(Error::FormatRecord)
    }
}

#[derive(Clone, Debug)]
struct MonthNameBase {
    month_names: [&'static str; 12],
}

impl MonthNameBase {
    fn new(month_names: [&'static str; 12]) -> Self {
        Self { month_names }
    }
}

impl Pattern for MonthNameBase {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let local_time = chrono::DateTime::<chrono::Local>::from(record.time());
        dest.write_str(self.month_names[local_time.weekday().num_days_from_monday() as usize])
            .map_err(Error::FormatRecord)
    }
}

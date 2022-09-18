use std::{fmt::Write, marker::PhantomData};

use crate::{
    formatter::{
        local_time_cacher::LOCAL_TIME_CACHER,
        pattern_formatter::{Pattern, PatternContext},
    },
    Error, Record, StringBuf,
};

/// A pattern that writes the abbreviated weekday name of log records into the
/// output. Example: `Mon`, `Tue`.
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
        let year_short_str = LOCAL_TIME_CACHER.lock().get(record.time()).year_short_str();
        dest.write_str(&year_short_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the year of log records into the output.
/// Examples: `2022`, `2021`.
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
        dest.write_str(&year_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the date of log records in `YYYY-MM-DD` format (ISO
/// 8601) into the output. Examples: `2022-04-01`, `2021-12-31`.
#[derive(Clone, Debug, Default)]
pub struct Date {
    _phantom: PhantomData<()>,
}

impl Date {
    /// Create a new `Date` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for Date {
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

        write!(dest, "{}-{}-{}", year_str, month_str, day_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the short date of log records in `MM/DD/YY` format
/// into the output. Examples: `04/01/22`, `12/31/21`.
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
        let (month_str, day_str, year_short_str) = {
            let mut local_cacher_lock = LOCAL_TIME_CACHER.lock();
            let cached_time = local_cacher_lock.get(record.time());
            (
                cached_time.month_str(),
                cached_time.day_str(),
                cached_time.year_short_str(),
            )
        };

        write!(dest, "{}/{}/{}", month_str, day_str, year_short_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the month of log records into the output.
/// Examples: `01`, `12`.
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
        dest.write_str(&month_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the day of log records into the output.
/// Examples: `01`, `12`, `31`, `30`.
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
        dest.write_str(&day_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the hour of log records into the output. Examples:
/// `01`, `12`, `23`.
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
        dest.write_str(&hour_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the hour in 12-hour format of log records into the
/// output. Examples: `01`, `12`.
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
        let hour_12_str = LOCAL_TIME_CACHER.lock().get(record.time()).hour12_str();
        dest.write_str(&hour_12_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the minute of log records into the output. Examples:
/// `00`, `05`, `59`.
#[derive(Clone, Debug, Default)]
pub struct Minute {
    _phantom: PhantomData<()>,
}

impl Minute {
    /// Create a new `Minute` pattern.
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
        dest.write_str(&minute_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the second of log records into the output. Examples:
/// `00`, `05`, `59`.
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
        dest.write_str(&second_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the millisecond part within a second of the timestamp
/// of a log record into the output. Example: `231`.
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
        let nanosecond = LOCAL_TIME_CACHER.lock().get(record.time()).nanosecond();
        write!(dest, "{:03}", nanosecond / 1_000_000).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the microsecond part within a second of the timestamp
/// of a log record into the output. Example: `372152`.
#[derive(Clone, Debug, Default)]
pub struct Microsecond {
    // This field prevents users from creating `Microsecond` objects literally.
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
        let nanosecond = LOCAL_TIME_CACHER.lock().get(record.time()).nanosecond();
        write!(dest, "{:06}", nanosecond / 1_000).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the nanosecond part within a second of the timestamp
/// of a log record into the output. Example: `482930154`.
#[derive(Clone, Debug, Default)]
pub struct Nanosecond {
    // This field prevents users from creating `Nanosecond` objects literally.
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
        let nanosecond = LOCAL_TIME_CACHER.lock().get(record.time()).nanosecond();
        write!(dest, "{:09}", nanosecond).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes "AM" or "PM" into the output according to the
/// timestamp of a log record. Example: `AM`, `PM`.
#[derive(Clone, Debug, Default)]
pub struct AmPm {
    // This field prevents users from creating `AmPm` objects literally.
    _phantom: PhantomData<()>,
}

impl AmPm {
    /// Create a new `AmPm` pattern.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl Pattern for AmPm {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let hour12 = LOCAL_TIME_CACHER.lock().get(record.time()).hour12();
        if hour12.0 {
            dest.write_str("PM")
        } else {
            dest.write_str("AM")
        }
        .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the time of log records in 12-hour format into the
/// output. Examples: `02:55:02 PM`.
#[derive(Clone, Debug, Default)]
pub struct Time12 {
    am_pm: AmPm,
}

impl Time12 {
    /// Create a new `Time12` pattern.
    pub fn new() -> Self {
        Self { am_pm: AmPm::new() }
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
                cached_time.hour12_str(),
                cached_time.minute_str(),
                cached_time.second_str(),
            )
        };

        write!(dest, "{}:{}:{}", hour_str, minute_str, second_str).map_err(Error::FormatRecord)?;

        dest.write_str(" ").map_err(Error::FormatRecord)?;
        self.am_pm.format(record, dest, ctx)?;

        Ok(())
    }
}

/// A pattern that writes the short time of log records into the output.
/// Examples: `22:28`, `09:53`.
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
        dest.write_str(&tz_offset_str).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the unix timestamp of log records into the output.
/// Examples: `1528834770`.
#[derive(Clone)]
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
        dest.write_str(&unix_timestamp_str)
            .map_err(Error::FormatRecord)
    }
}

impl Default for UnixTimestamp {
    fn default() -> Self {
        Self::new()
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
        let weekday_index = LOCAL_TIME_CACHER
            .lock()
            .get(record.time())
            .weekday_from_monday_0();
        dest.write_str(self.weekday_names[weekday_index as usize])
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
        let month = LOCAL_TIME_CACHER.lock().get(record.time()).month();
        dest.write_str(self.month_names[month as usize - 1])
            .map_err(Error::FormatRecord)
    }
}

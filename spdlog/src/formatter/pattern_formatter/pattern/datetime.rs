use std::{fmt::Write as _, marker::PhantomData};

use crate::{
    formatter::pattern_formatter::{Pattern, PatternContext},
    Error, Record, StringBuf,
};

/// A pattern that writes the abbreviated weekday name of log records into the
/// output. Example: `Mon`, `Tue`.
#[derive(Clone, Default)]
pub struct AbbrWeekdayName;

impl Pattern for AbbrWeekdayName {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(ctx.time_date().weekday_name().short)
            .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the weekday name of log records into the output.
/// Example: `Monday`, `Tuesday`.
#[derive(Clone, Default)]
pub struct WeekdayName;

impl Pattern for WeekdayName {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(ctx.time_date().weekday_name().full)
            .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the abbreviated month name of log records into the
/// output. Example: `Jan`, `Feb`.
#[derive(Clone, Default)]
pub struct AbbrMonthName;

impl Pattern for AbbrMonthName {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(ctx.time_date().month_name().short)
            .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the month name of log records into the output.
/// Example: `January`, `February`.
#[derive(Clone, Default)]
pub struct MonthName;

impl Pattern for MonthName {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(ctx.time_date().month_name().full)
            .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the full date time of log records into the output.
/// Example: `Thu Aug 23 15:35:46 2014`.
#[derive(Clone, Default)]
pub struct FullDateTime;

impl Pattern for FullDateTime {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        (|| {
            dest.write_str(ctx.time_date().weekday_name().short)?;
            dest.write_char(' ')?;
            dest.write_str(ctx.time_date().month_name().short)?;
            dest.write_char(' ')?;
            dest.write_str(ctx.time_date().day_str())?;
            dest.write_char(' ')?;
            dest.write_str(ctx.time_date().hour_str())?;
            dest.write_char(':')?;
            dest.write_str(ctx.time_date().minute_str())?;
            dest.write_char(':')?;
            dest.write_str(ctx.time_date().second_str())?;
            dest.write_char(' ')?;
            dest.write_str(ctx.time_date().year_str())
        })()
        .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the short year of log records into the output.
/// Examples: `22`, `20`.
#[derive(Clone, Default)]
pub struct ShortYear;

impl Pattern for ShortYear {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(ctx.time_date().year_short_str())
            .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the year of log records into the output.
/// Examples: `2022`, `2021`.
#[derive(Clone, Default)]
pub struct Year;

impl Pattern for Year {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(ctx.time_date().year_str())
            .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the date of log records in `YYYY-MM-DD` format (ISO
/// 8601) into the output. Examples: `2022-04-01`, `2021-12-31`.
#[derive(Clone, Default)]
pub struct Date;

impl Pattern for Date {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        (|| {
            dest.write_str(ctx.time_date().year_str())?;
            dest.write_char('-')?;
            dest.write_str(ctx.time_date().month_str())?;
            dest.write_char('-')?;
            dest.write_str(ctx.time_date().day_str())
        })()
        .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the short date of log records in `MM/DD/YY` format
/// into the output. Examples: `04/01/22`, `12/31/21`.
#[derive(Clone, Default)]
pub struct ShortDate;

impl Pattern for ShortDate {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        (|| {
            dest.write_str(ctx.time_date().month_str())?;
            dest.write_char('/')?;
            dest.write_str(ctx.time_date().day_str())?;
            dest.write_char('/')?;
            dest.write_str(ctx.time_date().year_short_str())
        })()
        .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the month of log records into the output.
/// Examples: `01`, `12`.
#[derive(Clone, Default)]
pub struct Month;

impl Pattern for Month {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(ctx.time_date().month_str())
            .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the day of log records into the output.
/// Examples: `01`, `12`, `31`, `30`.
#[derive(Clone, Default)]
pub struct Day;

impl Pattern for Day {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(ctx.time_date().day_str())
            .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the hour of log records into the output. Examples:
/// `01`, `12`, `23`.
#[derive(Clone, Default)]
pub struct Hour;

impl Pattern for Hour {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(ctx.time_date().hour_str())
            .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the hour in 12-hour format of log records into the
/// output. Examples: `01`, `12`.
#[derive(Clone, Default)]
pub struct Hour12;

impl Pattern for Hour12 {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(ctx.time_date().hour12_str())
            .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the minute of log records into the output. Examples:
/// `00`, `05`, `59`.
#[derive(Clone, Default)]
pub struct Minute;

impl Pattern for Minute {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(ctx.time_date().minute_str())
            .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the second of log records into the output. Examples:
/// `00`, `05`, `59`.
#[derive(Clone, Default)]
pub struct Second;

impl Pattern for Second {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(ctx.time_date().second_str())
            .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the millisecond part within a second of the timestamp
/// of a log record into the output. Example: `231`.
#[derive(Clone, Default)]
pub struct Millisecond {
    /// This field prevents users from creating `Millisecond` objects
    /// literally.
    _phantom: PhantomData<()>,
}

impl Pattern for Millisecond {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        write!(dest, "{:03}", ctx.time_date().millisecond()).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the microsecond part within a second of the timestamp
/// of a log record into the output. Example: `372152`.
#[derive(Clone, Default)]
pub struct Microsecond;

impl Pattern for Microsecond {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        let nanosecond = ctx.time_date().nanosecond();
        write!(dest, "{:06}", nanosecond / 1_000).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the nanosecond part within a second of the timestamp
/// of a log record into the output. Example: `482930154`.
#[derive(Clone, Default)]
pub struct Nanosecond;

impl Pattern for Nanosecond {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        write!(dest, "{:09}", ctx.time_date().nanosecond()).map_err(Error::FormatRecord)
    }
}

/// A pattern that writes "AM" or "PM" into the output according to the
/// timestamp of a log record. Example: `AM`, `PM`.
#[derive(Clone, Default)]
pub struct AmPm;

impl Pattern for AmPm {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(ctx.time_date().am_pm_str())
            .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the time of log records in 12-hour format into the
/// output. Examples: `02:55:02 PM`.
#[derive(Clone, Default)]
pub struct Time12;

impl Pattern for Time12 {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        (|| {
            dest.write_str(ctx.time_date().hour12_str())?;
            dest.write_char(':')?;
            dest.write_str(ctx.time_date().minute_str())?;
            dest.write_char(':')?;
            dest.write_str(ctx.time_date().second_str())?;
            dest.write_str(" ")?;
            dest.write_str(ctx.time_date().am_pm_str())
        })()
        .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the short time of log records into the output.
/// Examples: `22:28`, `09:53`.
#[derive(Clone, Default)]
pub struct ShortTime;

impl Pattern for ShortTime {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        (|| {
            dest.write_str(ctx.time_date().hour_str())?;
            dest.write_char(':')?;
            dest.write_str(ctx.time_date().minute_str())
        })()
        .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the time of log records into the output. Examples:
/// `22:28:02`, `09:53:41`.
#[derive(Clone, Default)]
pub struct Time;

impl Pattern for Time {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        (|| {
            dest.write_str(ctx.time_date().hour_str())?;
            dest.write_char(':')?;
            dest.write_str(ctx.time_date().minute_str())?;
            dest.write_char(':')?;
            dest.write_str(ctx.time_date().second_str())
        })()
        .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the timezone offset of log records into the output.
/// Examples: `+08:00`, `+00:00`, `-06:00`.
#[derive(Clone, Default)]
pub struct TzOffset;

impl Pattern for TzOffset {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(ctx.time_date().tz_offset_str())
            .map_err(Error::FormatRecord)
    }
}

/// A pattern that writes the unix timestamp of log records into the output.
/// Examples: `1528834770`.
#[derive(Clone, Default)]
pub struct UnixTimestamp;

impl Pattern for UnixTimestamp {
    fn format(
        &self,
        _record: &Record,
        dest: &mut StringBuf,
        ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        dest.write_str(ctx.time_date().unix_timestamp_str())
            .map_err(Error::FormatRecord)
    }
}

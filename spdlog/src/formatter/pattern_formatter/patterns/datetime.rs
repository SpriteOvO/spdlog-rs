use std::fmt::Write;

use chrono::Datelike;

use crate::{
    formatter::{
        pattern_formatter::{Pattern, PatternContext},
    },
    Error, Record, StringBuf,
};

/// A pattern that writes the abbreviated weekday name of log records into the
/// output. Example: `Mon`, `Tue`.
///
/// This pattern corresponds to `{a}` or `{weekday-name}` in the pattern
/// template string.
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

use std::time::SystemTime;

use chrono::prelude::*;
use once_cell::sync::Lazy;

use crate::sync::*;

pub(crate) static LOCAL_TIME_CACHER: Lazy<SpinMutex<LocalTimeCacher>> =
    Lazy::new(|| SpinMutex::new(LocalTimeCacher::new()));

#[derive(Clone)]
pub(crate) struct LocalTimeCacher {
    stored_key: CacheKey,
    cache_values: Option<CacheValues>,
}

pub(crate) struct TimeDate<'a> {
    cached: &'a mut CacheValues,
    nanosecond: u32,
    millisecond: u32,
}

#[derive(Clone, Eq, PartialEq)]
enum CacheKey {
    NonLeap(i64),
    Leap(i64),
}

#[derive(Clone, Eq, PartialEq)]
struct CacheValues {
    local_time: DateTime<Local>,
    is_leap_second: bool,
    full_second_str: Option<String>,
    year: Option<i32>,
    year_str: Option<String>,
    year_short_str: Option<String>,
    month: Option<u32>,
    month_str: Option<String>,
    month_name: Option<MultiName<&'static str>>,
    weekday_name: Option<MultiName<&'static str>>,
    day: Option<u32>,
    day_str: Option<String>,
    hour: Option<u32>,
    hour_str: Option<String>,
    hour12: Option<(bool, u32)>,
    hour12_str: Option<String>,
    am_pm_str: Option<&'static str>,
    minute: Option<u32>,
    minute_str: Option<String>,
    second: Option<u32>,
    second_str: Option<String>,
    tz_offset_str: Option<String>,
    unix_timestamp_str: Option<String>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct MultiName<T> {
    pub(crate) short: T,
    pub(crate) full: T,
}

impl LocalTimeCacher {
    #[must_use]
    fn new() -> LocalTimeCacher {
        LocalTimeCacher {
            stored_key: CacheKey::NonLeap(0),
            cache_values: None,
        }
    }

    #[must_use]
    pub(crate) fn get(&mut self, system_time: SystemTime) -> TimeDate {
        self.get_inner(system_time.into())
    }

    fn get_inner(&mut self, utc_time: DateTime<Utc>) -> TimeDate {
        const LEAP_BOUNDARY: u32 = 1_000_000_000;

        let nanosecond = utc_time.nanosecond();
        let is_leap_second = nanosecond >= LEAP_BOUNDARY;
        let reduced_nanosecond = if is_leap_second {
            nanosecond - LEAP_BOUNDARY
        } else {
            nanosecond
        };
        let millisecond = reduced_nanosecond / 1_000_000;

        let cache_key = CacheKey::new(&utc_time, is_leap_second);
        if self.cache_values.is_none() || self.stored_key != cache_key {
            self.cache_values = Some(CacheValues::new(utc_time, is_leap_second));
            self.stored_key = cache_key;
        }

        TimeDate::new(
            self.cache_values.as_mut().unwrap(),
            reduced_nanosecond,
            millisecond,
        )
    }
}

macro_rules! impl_cache_fields_getter {
    ( $($field:ident: $type:ty),*$(,)? ) => {
        #[must_use]
        $(pub(crate) fn $field(&mut self) -> $type {
            match self.cached.$field {
                Some(value) => value,
                None => {
                    let value = self.cached.local_time.$field();
                    self.cached.$field = Some(value);
                    value
                }
            }
        })*
    };
}

macro_rules! impl_cache_fields_str_getter {
    ( $($field:ident => $str_field:ident : $fmt:literal),* $(,)? ) => {
        #[must_use]
        $(pub(crate) fn $str_field(&mut self) -> &str {
            if self.cached.$str_field.is_none() {
                self.cached.$str_field = Some(format!($fmt, self.cached.local_time.$field()));
            }
            self.cached.$str_field.as_deref().unwrap()
        })*
    };
}

impl<'a> TimeDate<'a> {
    #[must_use]
    fn new(cached: &'a mut CacheValues, nanosecond: u32, millisecond: u32) -> Self {
        Self {
            cached,
            nanosecond,
            millisecond,
        }
    }

    #[must_use]
    pub(crate) fn full_second_str(&mut self) -> &str {
        if self.cached.full_second_str.is_none() {
            // `local_time.format("%Y-%m-%d %H:%M:%S")` is slower than this way
            self.cached.full_second_str = Some(format!(
                "{}-{:02}-{:02} {:02}:{:02}:{:02}",
                self.year(),
                self.month(),
                self.day(),
                self.hour(),
                self.minute(),
                self.second()
            ));
        }
        self.cached.full_second_str.as_deref().unwrap()
    }

    impl_cache_fields_getter! {
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        hour12: (bool, u32),
        minute: u32,
    }

    impl_cache_fields_str_getter! {
        year => year_str : "{:04}",
        month => month_str : "{:02}",
        day => day_str : "{:02}",
        hour => hour_str : "{:02}",
        minute => minute_str : "{:02}",
        second => second_str : "{:02}",
        timestamp => unix_timestamp_str : "{}",
    }

    #[must_use]
    pub(crate) fn second(&mut self) -> u32 {
        match self.cached.second {
            Some(value) => value,
            None => {
                let value = if !self.cached.is_leap_second {
                    self.cached.local_time.second()
                } else {
                    // https://www.itu.int/dms_pubrec/itu-r/rec/tf/R-REC-TF.460-6-200202-I!!PDF-E.pdf
                    60
                };
                self.cached.second = Some(value);
                value
            }
        }
    }

    #[must_use]
    pub(crate) fn weekday_name(&mut self) -> MultiName<&'static str> {
        match self.cached.weekday_name {
            Some(value) => value,
            None => {
                let value = {
                    const SHORT: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
                    const FULL: [&str; 7] = [
                        "Monday",
                        "Tuesday",
                        "Wednesday",
                        "Thursday",
                        "Friday",
                        "Saturday",
                        "Sunday",
                    ];

                    let weekday_from_monday_0 =
                        self.cached.local_time.weekday().num_days_from_monday() as usize;

                    MultiName {
                        short: SHORT[weekday_from_monday_0],
                        full: FULL[weekday_from_monday_0],
                    }
                };
                self.cached.weekday_name = Some(value);
                value
            }
        }
    }

    #[must_use]
    pub(crate) fn month_name(&mut self) -> MultiName<&'static str> {
        match self.cached.month_name {
            Some(value) => value,
            None => {
                let value = {
                    const SHORT: [&str; 12] = [
                        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct",
                        "Nov", "Dec",
                    ];
                    const FULL: [&str; 12] = [
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
                    ];

                    let month_index = self.cached.local_time.month() as usize - 1;

                    MultiName {
                        short: SHORT[month_index],
                        full: FULL[month_index],
                    }
                };
                self.cached.month_name = Some(value);
                value
            }
        }
    }

    #[must_use]
    pub(crate) fn nanosecond(&mut self) -> u32 {
        self.nanosecond
    }

    #[must_use]
    pub(crate) fn millisecond(&mut self) -> u32 {
        self.millisecond
    }

    #[must_use]
    pub(crate) fn hour12_str(&mut self) -> &str {
        if self.cached.hour12_str.is_none() {
            self.cached.hour12_str = Some(format!("{:02}", self.hour12().1));
        }
        self.cached.hour12_str.as_deref().unwrap()
    }

    #[must_use]
    pub(crate) fn am_pm_str(&mut self) -> &'static str {
        match self.cached.am_pm_str {
            Some(value) => value,
            None => {
                let value = if !self.hour12().0 { "AM" } else { "PM" };
                self.cached.am_pm_str = Some(value);
                value
            }
        }
    }

    #[must_use]
    pub(crate) fn year_short_str(&mut self) -> &str {
        if self.cached.year_short_str.is_none() {
            self.cached.year_short_str = Some(format!("{:02}", self.year() % 100));
        }
        self.cached.year_short_str.as_deref().unwrap()
    }

    #[must_use]
    pub(crate) fn tz_offset_str(&mut self) -> &str {
        if self.cached.tz_offset_str.is_none() {
            self.cached.tz_offset_str = {
                let offset_secs = self.cached.local_time.offset().local_minus_utc();
                let offset_secs_abs = offset_secs.abs();

                let sign_str = if offset_secs >= 0 { "+" } else { "-" };
                let offset_hours = offset_secs_abs / 3600;
                let offset_minutes = offset_secs_abs % 3600 / 60;

                Some(format!(
                    "{}{:02}:{:02}",
                    sign_str, offset_hours, offset_minutes
                ))
            };
        }
        self.cached.tz_offset_str.as_deref().unwrap()
    }
}

impl CacheKey {
    #[must_use]
    fn new(utc_time: &DateTime<Utc>, is_leap_second: bool) -> Self {
        let timestamp = utc_time.timestamp();
        if !is_leap_second {
            Self::NonLeap(timestamp)
        } else {
            Self::Leap(timestamp)
        }
    }
}

impl CacheValues {
    #[must_use]
    fn new(utc_time: DateTime<Utc>, is_leap_second: bool) -> Self {
        CacheValues {
            local_time: utc_time.into(),
            is_leap_second,
            full_second_str: None,
            year: None,
            year_str: None,
            year_short_str: None,
            month: None,
            month_str: None,
            month_name: None,
            weekday_name: None,
            day: None,
            day_str: None,
            hour: None,
            hour_str: None,
            hour12: None,
            hour12_str: None,
            am_pm_str: None,
            minute: None,
            minute_str: None,
            second: None,
            second_str: None,
            tz_offset_str: None,
            unix_timestamp_str: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_second() {
        let (date_2015, date_2022) = (
            NaiveDate::from_ymd_opt(2015, 6, 30).unwrap(),
            NaiveDate::from_ymd_opt(2022, 6, 30).unwrap(),
        );

        enum Kind {
            NonLeap,
            Leap,
        }

        let datetimes = [
            (
                Kind::NonLeap,
                date_2015.and_hms_nano_opt(23, 59, 59, 100_000_000).unwrap(),
            ),
            (
                Kind::Leap,
                date_2015
                    .and_hms_nano_opt(23, 59, 59, 1_000_000_000)
                    .unwrap(),
            ),
            (
                Kind::Leap,
                date_2015
                    .and_hms_nano_opt(23, 59, 59, 1_100_000_000)
                    .unwrap(),
            ),
            (Kind::NonLeap, date_2022.and_hms_opt(23, 59, 59).unwrap()),
            (
                Kind::NonLeap,
                date_2022.and_hms_nano_opt(23, 59, 59, 100_000_000).unwrap(),
            ),
        ];

        let mut cacher = LocalTimeCacher::new();

        for datetime in datetimes {
            let leap = match datetime.0 {
                Kind::NonLeap => false,
                Kind::Leap => true,
            };
            let datetime = datetime.1;

            println!(" => checking '{datetime}'");

            let mut result = cacher.get_inner(datetime.and_local_timezone(Utc).unwrap());
            assert_eq!(result.cached.is_leap_second, leap);
            assert_eq!(result.second(), if !leap { 59 } else { 60 });
        }
    }
}

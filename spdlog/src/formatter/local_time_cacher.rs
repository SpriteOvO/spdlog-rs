use std::{fmt, time::SystemTime};

use chrono::prelude::*;
use once_cell::sync::Lazy;

use crate::{formatter::FormatterContext, sync::*, Record};

static LOCAL_TIME_CACHER: Lazy<SpinMutex<LocalTimeCacher>> =
    Lazy::new(|| SpinMutex::new(LocalTimeCacher::new()));

pub(crate) fn fmt_with_time<R, F>(ctx: &mut FormatterContext, record: &Record, mut callback: F) -> R
where
    F: FnMut(TimeDate) -> R,
{
    if let Some(time_date) = ctx.locked_time_date.as_mut() {
        callback(time_date.get())
    } else {
        callback(LOCAL_TIME_CACHER.lock().get(record.time()))
    }
}

#[derive(Clone)]
pub(crate) struct LocalTimeCacher {
    stored_key: u64,
    cache_values: Option<CacheValues>,
}

pub(crate) struct TimeDate<'a> {
    cached: &'a mut CacheValues,
    nanosecond: u32,
    millisecond: u32,
}

#[derive(Clone, Eq, PartialEq)]
struct CacheValues {
    local_time: DateTime<Local>,
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
            stored_key: 0,
            cache_values: None,
        }
    }

    #[must_use]
    pub(crate) fn get(&mut self, system_time: SystemTime) -> TimeDate {
        let since_epoch = system_time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let nanosecond = since_epoch.subsec_nanos();
        let millisecond = nanosecond / 1_000_000;

        let cache_key = since_epoch.as_secs(); // Unix timestamp
        if self.cache_values.is_none() || self.stored_key != cache_key {
            self.cache_values = Some(CacheValues::new(system_time));
            self.stored_key = cache_key;
        }

        TimeDate {
            cached: self.cache_values.as_mut().unwrap(),
            nanosecond,
            millisecond,
        }
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

impl TimeDate<'_> {
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
        second: u32,
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

impl CacheValues {
    #[must_use]
    fn new(system_time: SystemTime) -> Self {
        CacheValues {
            local_time: system_time.into(),
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

struct TimeDateLocked<'a> {
    cached: SpinMutexGuard<'a, LocalTimeCacher>,
    nanosecond: u32,
    millisecond: u32,
}

pub(crate) struct TimeDateLazyLocked<'a> {
    time: SystemTime,
    locked: Option<TimeDateLocked<'a>>,
}

impl TimeDateLazyLocked<'_> {
    #[must_use]
    pub(crate) fn new(time: SystemTime) -> Self {
        Self { time, locked: None }
    }

    #[must_use]
    pub(crate) fn get(&mut self) -> TimeDate<'_> {
        let locked = self.locked.get_or_insert_with(|| {
            let mut cached = LOCAL_TIME_CACHER.lock();
            let time_date = cached.get(self.time);
            let (nanosecond, millisecond) = (time_date.nanosecond, time_date.millisecond);
            TimeDateLocked {
                cached,
                nanosecond,
                millisecond,
            }
        });

        TimeDate {
            cached: locked.cached.cache_values.as_mut().unwrap(),
            nanosecond: locked.nanosecond,
            millisecond: locked.millisecond,
        }
    }
}

impl fmt::Debug for TimeDateLazyLocked<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TimeDateLazyLocked")
            .field("time", &self.time)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation() {
        let mut cacher = LocalTimeCacher::new();

        let begin = SystemTime::now();
        loop {
            let now = SystemTime::now();
            if now.duration_since(begin).unwrap().as_secs() >= 3 {
                break;
            }
            let from_cache = cacher.get(now);
            let from_chrono = DateTime::<Local>::from(now);

            assert_eq!(
                from_cache.cached.local_time.with_nanosecond(0),
                from_chrono.with_nanosecond(0)
            );
            assert_eq!(from_cache.nanosecond, from_chrono.nanosecond());
            assert_eq!(from_cache.millisecond, from_chrono.nanosecond() / 1_000_000);
        }
    }
}

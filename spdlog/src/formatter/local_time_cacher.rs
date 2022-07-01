use std::{
    cell::{RefCell, RefMut},
    sync::Arc,
    time::SystemTime,
};

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
    full_second_str: RefCell<Option<String>>,
    year: RefCell<Option<i32>>,
    year_str: RefCell<Option<Arc<String>>>,
    month: RefCell<Option<u32>>,
    month_str: RefCell<Option<Arc<String>>>,
    weekday_from_monday_0: RefCell<Option<u32>>,
    day: RefCell<Option<u32>>,
    day_str: RefCell<Option<Arc<String>>>,
    hour: RefCell<Option<u32>>,
    hour_str: RefCell<Option<Arc<String>>>,
    hour12: RefCell<Option<(bool, u32)>>,
    hour12_str: RefCell<Option<Arc<String>>>,
    minute: RefCell<Option<u32>>,
    minute_str: RefCell<Option<Arc<String>>>,
    second: RefCell<Option<u32>>,
    second_str: RefCell<Option<Arc<String>>>,
    tz_offset_str: RefCell<Option<Arc<String>>>,
    unix_timestamp_str: RefCell<Option<Arc<String>>>,
}

impl LocalTimeCacher {
    fn new() -> LocalTimeCacher {
        LocalTimeCacher {
            stored_key: CacheKey::NonLeap(0),
            cache_values: None,
        }
    }

    pub(crate) fn get(&mut self, system_time: SystemTime) -> TimeDate {
        const LEAP_BOUNDARY: u32 = 1_000_000_000;

        let utc_time: DateTime<Utc> = system_time.into();
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
        $(pub(crate) fn $field(&self) -> $type {
            *self
                .cached
                .$field
                .borrow_mut()
                .get_or_insert_with(|| self.cached.local_time.$field())
        })*
    };
}

macro_rules! impl_cache_fields_str_getter {
    ( $($field:ident => $str_field:ident : $fmt:literal),* $(,)? ) => {
        $(pub(crate) fn $str_field(&self) -> Arc<String> {
            self.cached
                .$str_field
                .borrow_mut()
                .get_or_insert_with(|| Arc::new(format!($fmt, self.cached.local_time.$field())))
                .clone()
        })*
    };
}

impl<'a> TimeDate<'a> {
    fn new(cached: &'a mut CacheValues, nanosecond: u32, millisecond: u32) -> Self {
        Self {
            cached,
            nanosecond,
            millisecond,
        }
    }

    // A closed Rust PR "WIP: Downgrading of `RefMut` to `Ref`"
    // https://github.com/rust-lang/rust/pull/57401
    // There is nothing like `RefMut::downgrade()` for now, just keep in mind don't
    // modify the return value :)
    pub(crate) fn full_second_str(&self) -> RefMut<'_, str> {
        RefMut::map(self.cached.full_second_str.borrow_mut(), |opt| {
            opt.get_or_insert_with(|| {
                // `local_time.format("%Y-%m-%d %H:%M:%S")` is slower than this way
                format!(
                    "{}-{:02}-{:02} {:02}:{:02}:{:02}",
                    self.year(),
                    self.month(),
                    self.day(),
                    self.hour(),
                    self.minute(),
                    self.second()
                )
            })
            .as_mut()
        })
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

    pub(crate) fn second(&self) -> u32 {
        *self.cached.second.borrow_mut().get_or_insert_with(|| {
            if !self.cached.is_leap_second {
                self.cached.local_time.second()
            } else {
                // https://www.itu.int/dms_pubrec/itu-r/rec/tf/R-REC-TF.460-6-200202-I!!PDF-E.pdf
                60
            }
        })
    }

    pub(crate) fn weekday_from_monday_0(&self) -> u32 {
        *self
            .cached
            .weekday_from_monday_0
            .borrow_mut()
            .get_or_insert_with(|| self.cached.local_time.weekday().num_days_from_monday())
    }

    #[allow(dead_code)] // TODO: Remove this attr when it is used somewhere
    pub(crate) fn nanosecond(&self) -> u32 {
        self.nanosecond
    }

    pub(crate) fn millisecond(&self) -> u32 {
        self.millisecond
    }

    pub(crate) fn hour12_str(&self) -> Arc<String> {
        self.cached
            .hour12_str
            .borrow_mut()
            .get_or_insert_with(|| Arc::new(format!("{:02}", self.hour12().1)))
            .clone()
    }

    pub(crate) fn tz_offset_str(&self) -> Arc<String> {
        self.cached
            .tz_offset_str
            .borrow_mut()
            .get_or_insert_with(|| {
                let offset_secs = self.cached.local_time.offset().local_minus_utc();
                let offset_secs_abs = offset_secs.abs();

                let sign_str = if offset_secs >= 0 { "+" } else { "-" };
                let offset_hours = offset_secs_abs / 3600;
                let offset_minutes = offset_secs_abs % 3600 / 60;
                Arc::new(format!(
                    "{}{:02}:{:02}",
                    sign_str, offset_hours, offset_minutes
                ))
            })
            .clone()
    }
}

impl CacheKey {
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
    fn new(utc_time: DateTime<Utc>, is_leap_second: bool) -> Self {
        CacheValues {
            local_time: utc_time.into(),
            is_leap_second,
            full_second_str: RefCell::new(None),
            year: RefCell::new(None),
            year_str: RefCell::new(None),
            month: RefCell::new(None),
            month_str: RefCell::new(None),
            weekday_from_monday_0: RefCell::new(None),
            day: RefCell::new(None),
            day_str: RefCell::new(None),
            hour: RefCell::new(None),
            hour_str: RefCell::new(None),
            hour12: RefCell::new(None),
            hour12_str: RefCell::new(None),
            minute: RefCell::new(None),
            minute_str: RefCell::new(None),
            second: RefCell::new(None),
            second_str: RefCell::new(None),
            tz_offset_str: RefCell::new(None),
            unix_timestamp_str: RefCell::new(None),
        }
    }
}

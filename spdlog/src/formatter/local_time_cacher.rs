use std::{
    cell::{RefCell, RefMut},
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
    month: RefCell<Option<u32>>,
    day: RefCell<Option<u32>>,
    hour: RefCell<Option<u32>>,
    minute: RefCell<Option<u32>>,
    second: RefCell<Option<u32>>,
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
        minute: u32,
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

    #[allow(dead_code)] // TODO: Remove this attr when it is used somewhere
    pub(crate) fn nanosecond(&self) -> u32 {
        self.nanosecond
    }

    pub(crate) fn millisecond(&self) -> u32 {
        self.millisecond
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
            month: RefCell::new(None),
            day: RefCell::new(None),
            hour: RefCell::new(None),
            minute: RefCell::new(None),
            second: RefCell::new(None),
        }
    }
}

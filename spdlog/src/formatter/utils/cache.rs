use chrono::{DateTime, Local, Utc};

use crate::Record;

#[derive(Clone, Debug)]
pub struct FormattedDateTimeCacher<K> {
    inner: Cacher<K, String>,
}

impl<K> FormattedDateTimeCacher<K> {
    pub fn new() -> Self {
        Self {
            inner: Cacher::new(),
        }
    }
}

impl<K> FormattedDateTimeCacher<K>
where
    K: PartialEq,
{
    pub fn update<P, F>(
        &mut self,
        record: &Record,
        datetime_cache_key: P,
        datetime_formatter: F,
    ) -> &str
    where
        P: FnOnce(DateTime<Utc>) -> K,
        F: FnOnce(DateTime<Local>) -> String,
    {
        let utc_time = DateTime::<Utc>::from(record.time());
        let prec = datetime_cache_key(utc_time);
        self.inner.update(prec, move || {
            let local_time = DateTime::<Local>::from(utc_time);
            datetime_formatter(local_time)
        })
    }
}

impl<K> Default for FormattedDateTimeCacher<K> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
struct Cacher<K, V> {
    last_key: Option<K>,
    cached_value: Option<V>,
}

impl<K, V> Cacher<K, V> {
    fn new() -> Self {
        Self {
            last_key: None,
            cached_value: None,
        }
    }
}

impl<K, V> Cacher<K, V>
where
    K: PartialEq,
{
    fn update<F>(&mut self, key: K, value_computation: F) -> &V
    where
        F: FnOnce() -> V,
    {
        if self.cached_value.is_some() {
            if let Some(last_key) = &self.last_key {
                if *last_key == key {
                    return self.cached_value.as_ref().unwrap();
                }
            }
        }

        let value = value_computation();
        self.last_key = Some(key);
        self.cached_value = Some(value);

        self.cached_value.as_ref().unwrap()
    }
}

impl<K, V> Default for Cacher<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

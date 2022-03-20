#[derive(Clone, Debug)]
pub struct Cacher<K, V> {
    last_key: Option<K>,
    cached_value: Option<V>,
}

impl<K, V> Cacher<K, V> {
    pub fn new() -> Self {
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
    pub fn update<F>(&mut self, key: K, value_computation: F) -> &V
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

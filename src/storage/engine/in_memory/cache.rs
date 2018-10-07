use std::{
    collections::BTreeMap,
    ops::Bound,
    sync::{Arc, RwLock},
};

pub(crate) type Key = Vec<u8>;
pub(crate) type Value = Vec<u8>;
type Inner = BTreeMap<Key, Value>;

pub(crate) struct Range {
    pub start: Key,
    pub end: Key,
}

#[derive(Default)]
pub(crate) struct Cache {
    inner: Arc<RwLock<Inner>>,
}

impl Cache {
    pub(crate) fn get(&self, key: Key) -> Option<Vec<u8>> {
        let cache = self.inner.clone();
        let cache = cache.read().unwrap();
        Self::get_locked(&cache, key)
    }

    #[inline(always)]
    fn get_locked(cache: &Inner, key: Key) -> Option<Vec<u8>> {
        cache.get(&key).cloned()
    }

    pub(crate) fn get_range(&self, range: &Range) -> Vec<(Key, Value)> {
        let cache = self.inner.clone();
        let cache = cache.read().unwrap();
        Self::get_range_locked(&cache, range)
    }

    #[inline(always)]
    fn get_range_locked(cache: &Inner, range: &Range) -> Vec<(Key, Value)> {
        cache
            .range((
                Bound::Included(range.start.to_vec()),
                Bound::Included(range.end.to_vec()),
            )).map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub(crate) fn put(&self, key: Key, value: Value) {
        let cache = self.inner.clone();
        Self::put_locked(&mut cache.write().unwrap(), key, value);
    }

    #[inline(always)]
    fn put_locked(cache: &mut Inner, key: Key, value: Value) {
        cache.insert(key, value);
    }
}

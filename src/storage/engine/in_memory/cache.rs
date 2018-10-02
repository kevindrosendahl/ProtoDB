use std::{
    collections::BTreeMap,
    ops::Bound,
    sync::{Arc, RwLock},
};

#[derive(Default)]
pub(crate) struct Cache {
    inner: Arc<RwLock<BTreeMap<Vec<u8>, Vec<u8>>>>,
}

impl Cache {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let cache = self.inner.clone();
        let cache = cache.read().unwrap();
        cache.get(key).cloned()
    }

    fn get_range(&self, lower: &[u8], upper: &[u8]) -> Option<Vec<(Vec<u8>, Vec<u8>)>> {
        let cache = self.inner.clone();
        let cache = cache.read().unwrap();
        Some(
            cache
                .range((
                    Bound::Included(lower.to_vec()),
                    Bound::Included(upper.to_vec()),
                )).map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        )
    }

    fn put(&self, key: &[u8], value: &[u8]) {
        let cache = self.inner.clone();
        cache.write().unwrap().insert(key.to_vec(), value.to_vec());
    }
}

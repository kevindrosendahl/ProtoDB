use std::collections::BTreeMap;
use std::ops::Bound;
use std::sync::{Arc, RwLock};

use storage::kv::KVStorageEngine;

pub struct InMemoryStorageEngine {
    cache: Arc<RwLock<BTreeMap<Vec<u8>, Vec<u8>>>>
}

impl InMemoryStorageEngine {
    pub fn new() -> InMemoryStorageEngine {
        InMemoryStorageEngine { cache: Arc::new(RwLock::new(BTreeMap::new())) }
    }
}

impl KVStorageEngine for InMemoryStorageEngine {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let cache = self.cache.clone();
        let cache = cache.read().unwrap();
        cache.get(key).map(|v| v.clone())
    }

    fn get_range(&self, lower: &[u8], upper: &[u8]) -> Option<Vec<Vec<u8>>> {
        let cache = self.cache.clone();
        let cache = cache.read().unwrap();
        Some(
            cache.range((Bound::Included(lower.to_vec()), Bound::Included(upper.to_vec())))
                .map(|(_k, v)| v.clone())
                .collect()
        )
    }

    fn put(&self, key: &[u8], value: &[u8]) {
        let cache = self.cache.clone();
        cache.write().unwrap().insert(key.to_vec(), value.to_vec());
    }
}

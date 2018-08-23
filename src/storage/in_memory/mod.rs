use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use storage::kv::KVStorageEngine;
use database;

pub struct InMemoryStorageEngine {
    cache: Arc<RwLock<BTreeMap<String, String>>>
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
        cache.get(&String::from_utf8(key.to_vec()).unwrap()).map(|b| b.clone().into_bytes())
    }

    fn get_range(&self, lower: &[u8], upper: &[u8]) -> Option<Vec<Vec<u8>>> {
        let lower = String::from_utf8(lower.to_vec()).unwrap();
        let upper = String::from_utf8(upper.to_vec()).unwrap();

        let cache = self.cache.clone();
        let cache = cache.read().unwrap();
        Some(cache.iter().filter_map(|(k, v)| {
            if k.lt(&lower) || k.gt(&upper) {
                return None;
            }
            Some(v.as_bytes().to_vec())
        }).collect())
    }

    fn put(&self, key: &[u8], value: &[u8]) {
        let cache = self.cache.clone();
        cache.write().unwrap().insert(String::from_utf8(key.to_vec()).unwrap(), String::from_utf8(value.to_vec()).unwrap());
    }
}

struct Database {
    name: String,
}

impl database::Database for Database {
    fn name(&self) -> String {
        self.name.clone()
    }
}
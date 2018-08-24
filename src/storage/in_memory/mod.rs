use std::collections::BTreeMap;
use std::ops::Bound;
use std::sync::{Arc, RwLock};

use storage::storage_engine::StorageEngine;

const DATABASES_PREFIX: &'static str = "databases";
const DELIMITER: &'static str = "/";

pub struct InMemoryStorageEngine {
    cache: Arc<RwLock<BTreeMap<Vec<u8>, Vec<u8>>>>
}

impl InMemoryStorageEngine {
    pub fn new() -> InMemoryStorageEngine {
        InMemoryStorageEngine { cache: Arc::new(RwLock::new(BTreeMap::new())) }
    }

    #[allow(dead_code)]
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

impl  StorageEngine for InMemoryStorageEngine {
    fn list_databases(&self) -> Vec<String> {
        let lower = format!("{}{}", DATABASES_PREFIX, DELIMITER);
        let delimiter_plus_1 = DELIMITER.as_bytes()[0] + 1;
        let upper = format!("{}{}", DATABASES_PREFIX, delimiter_plus_1);

        let databases = self.get_range(lower.as_bytes(), upper.as_bytes()).unwrap();
        databases.iter().map(|b| String::from_utf8(b.clone()).unwrap()).collect()
    }

    fn create_database(&self, name: &str) {
        let key = format!("{}{}{}", DATABASES_PREFIX, DELIMITER, name);
        self.put(key.as_bytes(), name.as_bytes());
    }
}

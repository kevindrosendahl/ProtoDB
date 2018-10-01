use std::{
    collections::BTreeMap,
    ops::Bound,
    sync::{Arc, RwLock},
};

use prost_types::DescriptorProto;

#[derive(Default)]
pub struct Collection {
    pub schema: DescriptorProto,
    cache: Arc<RwLock<BTreeMap<Vec<u8>, Vec<u8>>>>,
}

impl Collection {
    pub fn new(schema: &DescriptorProto) -> Collection {
        Collection {
            schema: schema.clone(),
            cache: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let cache = self.cache.clone();
        let cache = cache.read().unwrap();
        cache.get(key).cloned()
    }

    fn get_range(&self, lower: &[u8], upper: &[u8]) -> Option<Vec<(Vec<u8>, Vec<u8>)>> {
        let cache = self.cache.clone();
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
        let cache = self.cache.clone();
        cache.write().unwrap().insert(key.to_vec(), value.to_vec());
    }
}

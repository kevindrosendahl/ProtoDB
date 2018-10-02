use std::{
    collections::BTreeMap,
    ops::Bound,
    sync::{Arc, RwLock},
};

use crate::storage::{
    errors,
    schema::{
        errors::SchemaError,
        Schema,
    },
};

use prost_types::DescriptorProto;

pub(crate) struct Collection {
    pub schema: Schema,
    cache: Arc<RwLock<BTreeMap<Vec<u8>, Vec<u8>>>>,
}

impl Collection {
    pub fn new(descriptor: &DescriptorProto) -> Result<Collection, SchemaError> {
        let schema = Schema::new(descriptor)?;
        Ok(Collection {
            schema,
            cache: Arc::new(RwLock::new(BTreeMap::new())),
        })
    }

    fn insert_object(&self, object: &[u8]) -> Result<(), errors::InsertError> {
        Ok(())
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

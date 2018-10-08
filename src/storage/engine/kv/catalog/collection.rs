use std::sync::Arc;

use super::super::store::KVStore;
use crate::storage::{
    catalog::collection::CollectionCatalogEntry,
    errors,
    schema::{errors::SchemaError, Schema},
};

use prost_types::DescriptorProto;

const KEY_DELIMITER: &str = "/";

#[derive(Clone)]
pub struct KVCollectionCatalogEntry {
    kv_store: Arc<dyn KVStore>,

    database: String,
    name: String,
    schema: Schema,
}

impl KVCollectionCatalogEntry {
    pub fn new(
        kv_store: Arc<dyn KVStore>,
        database: String,
        name: String,
        descriptor: &DescriptorProto,
    ) -> Result<KVCollectionCatalogEntry, SchemaError> {
        let schema = Schema::new(descriptor)?;
        Ok(KVCollectionCatalogEntry {
            kv_store,

            database,
            name,
            schema,
        })
    }

    #[inline(always)]
    fn tag_from_key(&self, key: String, id: u64) -> i32 {
        let prefix = self.object_key_prefix(id);
        let parts: Vec<&str> = key.split(&prefix).collect();
        if parts.len() != 2 {
            panic!("corrupted key for id {}: {}", id, key)
        }

        let suffix = parts[1];
        let parts: Vec<&str> = suffix.split(KEY_DELIMITER).collect();
        if parts.len() != 2 {
            panic!("corrupted key for id {}: {}", id, key)
        }

        let tag = parts[1];
        tag.parse().unwrap()
    }

    #[inline(always)]
    fn object_key_prefix(&self, id: u64) -> String {
        format!(
            "{database}{delimiter}{collection}{delimiter}{id}",
            database = self.database,
            delimiter = KEY_DELIMITER,
            collection = self.name,
            id = id,
        )
    }

    #[inline(always)]
    fn field_key(&self, id: u64, tag: i32) -> String {
        format!(
            "{prefix}{delimiter}{tag}",
            prefix = self.object_key_prefix(id),
            delimiter = KEY_DELIMITER,
            tag = tag,
        )
    }
}

impl CollectionCatalogEntry for KVCollectionCatalogEntry {
    fn name(&self) -> &str {
        &self.name
    }

    fn schema(&self) -> &Schema {
        &self.schema
    }

    fn find_object(&self, id: u64) -> Result<Vec<u8>, errors::collection::FindObjectError> {
        Ok(vec![])
    }

    fn insert_object(&self, object: &[u8]) -> Result<(), errors::collection::InsertObjectError> {
        Ok(())
    }
}

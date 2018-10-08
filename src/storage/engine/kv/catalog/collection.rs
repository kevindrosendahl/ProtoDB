use std::sync::Arc;

use super::super::store::{KVStore, KVStoreWriteBatch};
use crate::storage::{
    catalog::collection::CollectionCatalogEntry,
    errors,
    schema::{
        encoding::{FieldInfo, FieldValue},
        errors::{ObjectError, SchemaError},
        Schema,
    },
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
        let start = self.object_key_prefix(id);

        // add 1 to the byte value of the last byte in the prefix
        // this should make the range span over all keys with the prefix
        // and no more
        let end = start.clone();
        let mut end = end.into_bytes();
        let last = end.pop().unwrap();
        end.push(last + 1);

        let start = start.into_bytes();

        // allocate the buffer that we'll be encoding the message into
        let mut buf = Vec::new();

        let store = self.kv_store.clone();
        for (key, value) in store.bounded_prefix_iterator(&start, &end) {
            // FIXME: handle this error
            let tag = self.tag_from_key(String::from_utf8(key.to_vec()).unwrap(), id);
            let wire_type = match self.schema.wire_type(tag) {
                Some(wire_type) => wire_type,
                // this indicates there's a field in the cache that isn't in the schema
                // this shouldn't currently be possible
                None => continue,
            };

            Schema::encode_field(tag, wire_type, &value, &mut buf);
        }

        Ok(buf)
    }

    fn insert_object(&self, object: &[u8]) -> Result<(), errors::collection::InsertObjectError> {
        let mut id = None;
        let fields = self
            .schema
            .decode_object(object)
            .map(|f| {
                // check to see if this field is the id field. if it is,
                // ensure that the value is a Uint64 (for now) and set id
                // to it
                if f.is_err() {
                    return f;
                }

                let f = f.unwrap();
                if f.tag != self.schema.id_field {
                    return Ok(f);
                }

                match f.value {
                    FieldValue::Uint64(val) => {
                        id = Some(val);
                        Ok(f)
                    }
                    _ => Err(ObjectError::SchemaError(SchemaError::InvalidIdType(
                        format!("{:?}", f.value),
                    ))),
                }
            }).collect::<Result<Vec<FieldInfo>, ObjectError>>()?;

        if id.is_none() {
            return Err(errors::collection::InsertObjectError::ObjectError(
                ObjectError::SchemaError(SchemaError::MissingIdField),
            ));
        }

        let store = self.kv_store.clone();

        let id = id.unwrap();
        let id_key = self.field_key(id, self.schema.id_field).as_bytes().to_vec();
        if store.get(&id_key).is_some() {
            return Err(errors::collection::InsertObjectError::ObjectExists);
        }

        let mut batch = Vec::new();
        for field in fields {
            batch.push((
                self.field_key(id, field.tag),
                Schema::encode_value(field.value),
            ));
        }

        store.write(
            batch
                .iter()
                .map(|(k, v)| (k.as_bytes(), v.as_slice()))
                .collect(),
        );
        Ok(())
    }
}

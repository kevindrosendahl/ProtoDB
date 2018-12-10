use std::sync::Arc;

use crate::index::{
    errors::{BuildIndexError, IndexInsertError},
    IndexAccessMethod, IteratorMode,
};

use super::super::super::errors::InternalStorageEngineError;
use super::{
    keys::*,
    store::{KVStore, KVStoreBytes},
};

use prost_types::field_descriptor_proto::Type;
use protodb_schema::{encoding::FieldValue, DecodedIdObject, Schema};

pub struct KVIndexAccessMethod {
    kv_store: Arc<dyn KVStore>,

    database: String,
    collection: String,
    index_id: usize,

    schema: Schema,
    field: i32,
    field_type: Type,
}

impl KVIndexAccessMethod {
    pub fn new(
        kv_store: Arc<dyn KVStore>,
        database: String,
        collection: String,
        index_id: usize,
        schema: Schema,
        field: i32,
    ) -> KVIndexAccessMethod {
        let (_, _, field_type) = schema.fields.info(&field).unwrap();
        let field_type = *field_type;
        KVIndexAccessMethod {
            kv_store,
            database,
            collection,
            index_id,
            schema,
            field,
            field_type,
        }
    }
}

impl IndexAccessMethod for KVIndexAccessMethod {
    fn build(
        &self,
        objects: Box<dyn Iterator<Item = Result<Vec<u8>, InternalStorageEngineError>>>,
    ) -> Result<(), BuildIndexError> {
        for object in objects {
            let object = object?;
            let decoded = self.schema.decoded_object(&object).unwrap();
            self.insert(decoded)?;
        }
        Ok(())
    }

    fn insert(&self, obj: DecodedIdObject) -> Result<(), IndexInsertError> {
        let field_value = match obj.field(self.field) {
            Some(value) => value,
            None => return Ok(()),
        };

        let key = index_object_key(
            &self.database,
            &self.collection,
            self.index_id,
            &field_value,
            obj.id,
        );

        let value = Vec::new();
        let batch = vec![(key.as_bytes(), value.as_slice())];
        let store = self.kv_store.clone();
        store.write(batch)?;
        Ok(())
    }

    fn iter(&self, mode: IteratorMode) -> Box<Iterator<Item = (FieldValue, u64)>> {
        Box::new(KVIndexAccessMethodIterator::new(
            &self.kv_store,
            mode,
            self.database.clone(),
            self.collection.clone(),
            self.index_id,
            self.field_type,
        ))
    }
}

struct KVIndexAccessMethodIterator {
    inner: Box<dyn Iterator<Item = KVStoreBytes>>,

    database: String,
    collection: String,
    index_id: usize,
    field_type: Type,
}

impl KVIndexAccessMethodIterator {
    fn new(
        kv_store: &Arc<dyn KVStore>,
        mode: IteratorMode,
        database: String,
        collection: String,
        index_id: usize,
        field_type: Type,
    ) -> Self {
        let index_start = index_key_prefix(&database, &collection, index_id);
        let (start, end) = delimiter_prefix_bound(index_start);

        let start= match mode.from {
            Some(value) => index_object_key_prefix(&database, &collection, index_id, &value).into_bytes(),
            None => start
        };




//        let (start, end) = delimiter_prefix_bound(start);

        let _start = String::from_utf8(start.clone()).unwrap();
        let _end = String::from_utf8(end.clone()).unwrap();

        KVIndexAccessMethodIterator {
            inner: kv_store.bounded_prefix_iterator(&start, &end),

            database,
            collection,
            index_id,
            field_type,
        }
    }
}

impl Iterator for KVIndexAccessMethodIterator {
    type Item = (FieldValue, u64);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let (key, _) = self.inner.next()?;
        Some(value_and_id_from_index_key(
            &self.database,
            &self.collection,
            self.index_id,
            self.field_type,
            &String::from_utf8_lossy(&key),
        ))
    }
}

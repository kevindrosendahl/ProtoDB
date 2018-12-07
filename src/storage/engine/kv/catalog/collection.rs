use std::{cell::RefCell, collections::HashSet, io::Cursor, sync::Arc};

use super::super::store::{KVStore, KVStoreBytes};
use super::{delimiter_prefix_bound, index::KVIndexCatalog, key_suffix, KEY_DELIMITER};
use crate::{
    catalog::{
        collection::CollectionCatalogEntry,
        errors::collection::{FindObjectError, InsertObjectError},
        index::IndexCatalog,
    },
    schema::{errors::SchemaError, DecodedObject, DecodedObjectBuilder, Schema},
    storage::errors::InternalStorageEngineError,
};

use prost_types::DescriptorProto;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

#[derive(Clone)]
pub struct KVCollectionCatalogEntry {
    kv_store: Arc<dyn KVStore>,
    key_generator: CollectionKeyGenerator,

    database: String,
    name: String,
    schema: Schema,
    indexes: Arc<KVIndexCatalog>,
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
            kv_store: kv_store.clone(),
            key_generator: CollectionKeyGenerator {
                database: database.clone(),
                collection: name.clone(),
            },

            database: database.clone(),
            name: name.clone(),
            schema,
            indexes: Arc::new(KVIndexCatalog::new(kv_store, database, name)),
        })
    }
}

impl CollectionCatalogEntry for KVCollectionCatalogEntry {
    fn name(&self) -> &str {
        &self.name
    }

    fn schema(&self) -> &Schema {
        &self.schema
    }

    fn indexes(&self) -> Arc<dyn IndexCatalog> {
        self.indexes.clone()
    }

    fn find_all(
        &self,
        fields: Option<HashSet<i32>>,
    ) -> Box<dyn Iterator<Item = Result<Vec<u8>, InternalStorageEngineError>>> {
        Box::new(FindAll::new(self.clone(), fields))
            as Box<dyn Iterator<Item = Result<Vec<u8>, InternalStorageEngineError>>>
    }

    fn find_object(&self, id: u64) -> Result<Option<Vec<u8>>, FindObjectError> {
        // get the key bounds for the object
        let (start, end) = delimiter_prefix_bound(self.key_generator.field_key_prefix(id));

        // allocate the buffer that we'll be encoding the message into
        let mut buf = Vec::new();

        // iterate through the key/value pairs that are within the object's key range,
        // encoding the values into the buffer if the tag for the field is found in
        // the schema
        for (key, value) in self.kv_store.clone().bounded_prefix_iterator(&start, &end) {
            // FIXME: handle this error
            let tag = self
                .key_generator
                .tag_from_key(&String::from_utf8(key.to_vec()).unwrap(), id);
            let wire_type = match self.schema.wire_type(tag) {
                Some(wire_type) => wire_type,
                // this indicates there's a field in the cache that isn't in the schema
                // this shouldn't currently be possible
                None => continue,
            };

            Schema::encode_field(tag, wire_type, &value, &mut buf);
        }

        match buf.len() {
            0 => Ok(None),
            _ => Ok(Some(buf)),
        }
    }

    fn insert_object(&self, object: &[u8]) -> Result<(), InsertObjectError> {
        // decode the object and retrieve the key for this object's id
        let decoded = self.schema.decoded_object(object)?;
        let id_key = self
            .key_generator
            .field_key(decoded.id, self.schema.id_field)
            .as_bytes()
            .to_vec();

        // check to see if an object with this id already exists in the store
        let store = self.kv_store.clone();
        let id_field = store.get(&id_key)?;
        if id_field.is_some() {
            return Err(InsertObjectError::ObjectExists);
        }

        // batch up all of the fields' writes to the store
        let mut batch = Vec::new();
        for (tag, value) in decoded.fields_iter() {
            batch.push((
                self.key_generator.field_key(decoded.id, *tag),
                Schema::encode_value(value.clone()),
            ));
        }

        // write the batch
        store
            .write(
                // FIXME: pretty silly to allocate the batch vec above then have to map it
                //        to grab the proper references
                batch
                    .iter()
                    .map(|(k, v)| (k.as_bytes(), v.as_slice()))
                    .collect(),
            )
            .map_err(|err| err.into())
    }
}

#[derive(Clone)]
struct CollectionKeyGenerator {
    database: String,
    collection: String,
}

impl CollectionKeyGenerator {
    #[inline(always)]
    fn tag_from_key(&self, key: &str, id: u64) -> i32 {
        let prefix = self.field_key_prefix(id);
        let tag = key_suffix(&prefix, &key);
        tag.parse().unwrap()
    }

    #[inline(always)]
    fn parts_from_key(&self, key: &str) -> (u64, i32) {
        let prefix = self.key_prefix();
        let parts = key_suffix(&prefix, &key);
        let parts: Vec<&str> = parts.split(KEY_DELIMITER).collect();
        if parts.len() != 2 {
            panic!("corrupted key: {}", key);
        }
        (parts[0].parse().unwrap(), parts[1].parse().unwrap())
    }

    #[inline(always)]
    fn key_prefix(&self) -> String {
        format!(
            "{database}{delimiter}{collection}{delimiter}",
            database = self.database,
            delimiter = KEY_DELIMITER,
            collection = self.collection,
        )
    }

    #[inline(always)]
    fn object_key_prefix(&self, id: u64) -> String {
        // In order to have the keys sorted in the correct order,
        // pad the left of the id with 0s up to the length of the
        // longest u64.
        format!("{prefix}{id:0>20}", prefix = self.key_prefix(), id = id,)
    }

    #[inline(always)]
    fn field_key_prefix(&self, id: u64) -> String {
        format!(
            "{prefix}{delimiter}",
            prefix = self.object_key_prefix(id),
            delimiter = KEY_DELIMITER,
        )
    }

    #[inline(always)]
    fn field_key(&self, id: u64, tag: i32) -> String {
        // In order to have the keys sorted in the correct order,
        // pad the left of the id with 0s up to the length of the
        // longest i32.
        format!(
            "{prefix}{tag:0>10}",
            prefix = self.field_key_prefix(id),
            tag = tag,
        )
    }
}

struct FindAll {
    inner: Box<dyn Iterator<Item = KVStoreBytes>>,

    // should probably take a reference to the collection,
    // but cloning the KVCollectionCatalogEntry shouldn't be _too_
    // expensive and should be fine since most of its important members are Arcs,
    // and easier for now than getting the lifetime right (if possible)
    collection: KVCollectionCatalogEntry,

    fields: Option<HashSet<i32>>,

    curr_id: u64,
    curr_object: RefCell<Vec<u8>>,
    done: bool,
}

impl FindAll {
    fn new(collection: KVCollectionCatalogEntry, fields: Option<HashSet<i32>>) -> Self {
        let start = collection.key_generator.key_prefix();
        let mut end = start.clone().into_bytes();
        let delimiter_byte = end.pop().unwrap();
        end.push(delimiter_byte + 1);

        let kv_store = collection.kv_store.clone();
        FindAll {
            inner: kv_store.bounded_prefix_iterator(&start.into_bytes(), &end),
            collection,
            fields,
            curr_id: 0,
            curr_object: Default::default(),
            done: false,
        }
    }
}

impl Iterator for FindAll {
    type Item = Result<Vec<u8>, InternalStorageEngineError>;

    fn next(&mut self) -> Option<Self::Item> {
        // If this iterator was previously marked as done, simply return None.
        if self.done {
            return None;
        }

        // Loop over key/value pairs until you see a new id or the key/value iterator
        // is extinguished, and return the built object if there was one once either
        // of those conditions is met.
        loop {
            let next = self.inner.next();
            if next.is_none() {
                self.done = true;

                // If we never started building an object, the collection is empty so return None.
                if self.curr_id == 0 {
                    return None;
                }

                // Otherwise return the object.
                let object = self.curr_object.replace(Vec::new());
                return Some(Ok(object));
            }

            // Get the key/value information from the info returned by the iterator.
            let (key, mut value) = next.unwrap();
            let (id, tag) = self
                .collection
                .key_generator
                .parts_from_key(&String::from_utf8(key).unwrap());

            // If this is the first object we're seeing, set curr_id to its id.
            if self.curr_id == 0 {
                self.curr_id = id;
            }

            // If we're looking at a new object, create a buffer for the new object,
            // and set the current object to return after updating the new builder with
            // the info we're seeing here.
            // Also update the curr_id to be the new object's id.
            let returnable = if id == self.curr_id {
                None
            } else {
                self.curr_id = id;
                let object = self.curr_object.replace(Vec::new());
                Some(object)
            };

            // Append the value to the object if either no fields were specified or if this was a specified field.
            let valid = match &self.fields {
                Some(fields) => fields.contains(&tag),
                None => true,
            };
            if valid {
                self.curr_object.borrow_mut().append(&mut value);
            }

            // If this iteration was a new id, return it.
            if let Some(object) = returnable {
                return Some(Ok(object));
            }
        }
    }
}

use std::sync::Arc;

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

    fn find_all_decoded(
        &self,
    ) -> Box<dyn Iterator<Item = Result<DecodedObject, InternalStorageEngineError>>> {
        //        let start = self.object_key_prefix(1);
        //        let end = self.object_key_prefix(u64::max_value());
        //        unimplemented!()
        Box::new(FindAllDecoded::new(self.clone()))
            as Box<dyn Iterator<Item = Result<DecodedObject, InternalStorageEngineError>>>
    }

    fn find_object(&self, id: u64) -> Result<Option<Vec<u8>>, FindObjectError> {
        // get the key bounds for the object
        let (start, end) = delimiter_prefix_bound(self.key_generator.object_key_prefix(id));

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
        format!("{prefix}{id}", prefix = self.key_prefix(), id = id,)
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
        format!(
            "{prefix}{tag}",
            prefix = self.field_key_prefix(id),
            tag = tag,
        )
    }
}

struct FindAllDecoded {
    inner: Box<dyn Iterator<Item = KVStoreBytes>>,
    // should probably take a reference to the collection,
    // but cloning the KVCollectionCatalogEntry shouldn't be _too_
    // expensive and should be fine since most of its important members are Arcs,
    // and easier for now than getting the lifetime right (if possible)
    collection: KVCollectionCatalogEntry,
}

impl FindAllDecoded {
    fn new(collection: KVCollectionCatalogEntry) -> Self {
        let start = collection.key_generator.object_key_prefix(1);
        let end = collection.key_generator.object_key_prefix(u64::max_value());
        let kv_store = collection.kv_store.clone();
        FindAllDecoded {
            inner: kv_store.bounded_prefix_iterator(&start.into_bytes(), &end.into_bytes()),
            collection,
        }
    }
}

impl Iterator for FindAllDecoded {
    type Item = Result<DecodedObject, InternalStorageEngineError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut builder = self.collection.schema.decoded_object_builder();
        loop {
            let next = self.inner.next();
            if next.is_none() {
                return None;
            }

            let (key, value) = next.unwrap();
            let (id, tag) = self
                .collection
                .key_generator
                .parts_from_key(&String::from_utf8(key).unwrap());

            let info = self.collection.schema.fields.get(&tag);
            if info.is_none() {
                continue;
            }

            let (_, type_) = info.unwrap();
            let value = Schema::decode_value(*type_, &value);

            builder.id(id);
            builder.field(tag, value).unwrap();
        }
        unimplemented!()
    }
}

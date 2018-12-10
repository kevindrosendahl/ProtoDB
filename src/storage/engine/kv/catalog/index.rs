use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
};

use super::super::{index::KVIndexAccessMethod, store::KVStore};
use crate::{
    catalog::index::{IndexCatalog, IndexCatalogEntry},
    index::IndexAccessMethod,
    storage::errors::InternalStorageEngineError,
};

use protodb_schema::Schema;

pub struct KVIndexCatalog {
    kv_store: Arc<dyn KVStore>,

    database: String,
    collection: String,
    schema: Schema,

    entry_counter: AtomicUsize,
    entries: Arc<RwLock<HashMap<usize, Arc<dyn IndexCatalogEntry>>>>,
}

impl KVIndexCatalog {
    pub fn new(
        kv_store: Arc<dyn KVStore>,
        database: String,
        collection: String,
        schema: Schema,
    ) -> KVIndexCatalog {
        KVIndexCatalog {
            kv_store,

            database,
            collection,
            schema,

            entry_counter: AtomicUsize::new(1),
            entries: Default::default(),
        }
    }
}

impl IndexCatalog for KVIndexCatalog {
    fn create_index(&self, field: i32) -> Result<usize, InternalStorageEngineError> {
        let index_id = self.entry_counter.fetch_add(1, Ordering::SeqCst) + 1;
        let index_entry = Arc::new(KVIndexCatalogEntry::new(
            self.kv_store.clone(),
            self.database.clone(),
            self.collection.clone(),
            self.schema.clone(),
            index_id,
            field,
        ));

        let entries = self.entries.clone();
        let mut entries = entries.write().unwrap();
        entries.insert(index_id, index_entry);
        Ok(index_id)
    }

    fn index_entry(&self, id: usize) -> Option<Arc<dyn IndexCatalogEntry>> {
        let entries = self.entries.clone();
        let entries = entries.read().unwrap();
        entries.get(&id).cloned()
    }
}

pub struct KVIndexCatalogEntry {
    id: usize,
    access_method: Arc<dyn IndexAccessMethod>,
}

impl KVIndexCatalogEntry {
    pub fn new(
        kv_store: Arc<dyn KVStore>,
        database: String,
        collection: String,
        schema: Schema,
        id: usize,
        field: i32,
    ) -> KVIndexCatalogEntry {
        KVIndexCatalogEntry {
            id,
            access_method: Arc::new(KVIndexAccessMethod::new(
                kv_store, database, collection, id, schema, field,
            )),
        }
    }
}

impl IndexCatalogEntry for KVIndexCatalogEntry {
    fn id(&self) -> usize {
        self.id
    }

    fn access_method(&self) -> Arc<IndexAccessMethod> {
        self.access_method.clone()
    }
}

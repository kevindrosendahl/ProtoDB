use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
};

use super::super::store::KVStore;
use crate::{
    catalog::index::{IndexCatalog, IndexCatalogEntry},
    index::IndexAccessMethod,
    storage::errors::InternalStorageEngineError,
};

pub struct KVIndexCatalog {
    kv_store: Arc<dyn KVStore>,

    database: String,
    collection: String,

    entry_counter: AtomicUsize,
    entries: Arc<RwLock<HashMap<usize, Arc<dyn IndexCatalogEntry>>>>,
}

impl KVIndexCatalog {
    pub fn new(kv_store: Arc<dyn KVStore>, database: String, collection: String) -> KVIndexCatalog {
        KVIndexCatalog {
            kv_store,

            database,
            collection,

            entry_counter: AtomicUsize::new(1),
            entries: Default::default(),
        }
    }
}

impl IndexCatalog for KVIndexCatalog {
    fn create_index(&self, field: i32) -> Result<usize, InternalStorageEngineError> {
        let index_id = self.entry_counter.fetch_add(1, Ordering::SeqCst) + 1;
        unimplemented!()
    }

    fn index_entry(&self, id: usize) -> Option<Arc<dyn IndexCatalogEntry>> {
        let entries = self.entries.clone();
        let entries = entries.read().unwrap();
        entries.get(&id).cloned()
    }
}

pub struct KVIndexCatalogEntry {
    kv_store: Arc<dyn KVStore>,

    database: String,
    collection: String,

    access_method: Arc<dyn IndexAccessMethod>,
}

impl KVIndexCatalogEntry {
    pub fn new(kv_store: Arc<dyn KVStore>, database: String, collection: String) -> KVIndexCatalogEntry {
        unimplemented!()
    }
}

impl IndexCatalogEntry for KVIndexCatalogEntry {
    fn id(&self) -> u64 {
        unimplemented!()
    }

    fn access_method(&self) -> Arc<IndexAccessMethod> {
        unimplemented!()
    }
}
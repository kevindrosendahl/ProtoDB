pub mod kv_store;

use super::kv::catalog::database::KVDatabaseCatalog;
use crate::storage;
use crate::storage::{catalog, StorageEngine};

pub struct InMemoryStorageEngine<'a> {
    catalog: KVDatabaseCatalog<'a>,
}

impl<'a> InMemoryStorageEngine<'a> {
    pub fn new() -> InMemoryStorageEngine<'a> {
        let store: kv_store::InMemoryKVStore = Default::default();
        InMemoryStorageEngine {
            catalog: KVDatabaseCatalog::new(Box::new(store)),
        }
    }
}

impl<'a> StorageEngine for InMemoryStorageEngine<'a> {
    fn catalog(&self) -> Box<dyn catalog::database::DatabaseCatalog> {
        unimplemented!()
    }
}

pub mod kv_store;

use std::sync::Arc;

use super::kv::catalog::database::KVDatabaseCatalog;
use crate::storage;
use crate::storage::{catalog, StorageEngine};

pub struct InMemoryStorageEngine {
    store: Arc<kv_store::InMemoryKVStore>,
    catalog: KVDatabaseCatalog,
}

impl InMemoryStorageEngine {
    pub fn new() -> InMemoryStorageEngine {
        let store: Arc<kv_store::InMemoryKVStore> = Arc::new(Default::default());
        InMemoryStorageEngine {
            store: store.clone(),
            catalog: KVDatabaseCatalog::new(store),
        }
    }
}

impl StorageEngine for InMemoryStorageEngine {
    fn catalog(&self) -> Arc<dyn catalog::database::DatabaseCatalog> {
        unimplemented!()
    }
}

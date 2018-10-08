pub mod kv_store;

use std::sync::Arc;

use super::kv::catalog::database::KVDatabaseCatalog;
use crate::storage;
use crate::storage::{catalog, StorageEngine};

pub struct InMemoryStorageEngine {
    catalog: Arc<KVDatabaseCatalog>,
}

impl InMemoryStorageEngine {
    pub fn new() -> InMemoryStorageEngine {
        let store: Arc<kv_store::InMemoryKVStore> = Arc::new(Default::default());
        InMemoryStorageEngine {
            catalog: Arc::new(KVDatabaseCatalog::new(store)),
        }
    }
}

impl StorageEngine for InMemoryStorageEngine {
    fn catalog(&self) -> Arc<dyn catalog::database::DatabaseCatalog> {
        self.catalog.clone() as Arc<dyn catalog::database::DatabaseCatalog>
    }
}

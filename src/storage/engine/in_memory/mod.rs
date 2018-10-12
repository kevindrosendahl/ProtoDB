pub mod kv_store;

use std::sync::Arc;

use super::kv::catalog::database::KVDatabaseCatalog;
use crate::{catalog::database::DatabaseCatalog, storage::StorageEngine};

pub struct InMemoryStorageEngine {
    catalog: Arc<KVDatabaseCatalog>,
}

impl InMemoryStorageEngine {
    pub fn new() -> InMemoryStorageEngine {
        let store = Arc::new(kv_store::InMemoryKVStore::default());
        InMemoryStorageEngine {
            catalog: Arc::new(KVDatabaseCatalog::new(store)),
        }
    }
}

impl StorageEngine for InMemoryStorageEngine {
    fn catalog(&self) -> Arc<dyn DatabaseCatalog> {
        self.catalog.clone() as Arc<dyn DatabaseCatalog>
    }
}

impl Default for InMemoryStorageEngine {
    fn default() -> Self {
        Self::new()
    }
}

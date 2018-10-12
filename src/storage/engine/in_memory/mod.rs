pub mod kv_store;

use std::sync::Arc;

use super::kv::catalog::database::KVDatabaseCatalog;
use crate::{
    catalog::database::DatabaseCatalog, storage::{errors::InternalStorageEngineError, StorageEngine}};

pub struct InMemoryStorageEngine {
    catalog: Arc<KVDatabaseCatalog>,
}

impl InMemoryStorageEngine {
    pub fn new() -> Result<InMemoryStorageEngine, InternalStorageEngineError> {
        let store = Arc::new(kv_store::InMemoryKVStore::default());
        let catalog = KVDatabaseCatalog::new(store)?;
        Ok(InMemoryStorageEngine {
            catalog: Arc::new(catalog),
        })
    }
}

impl StorageEngine for InMemoryStorageEngine {
    fn catalog(&self) -> Arc<dyn DatabaseCatalog> {
        self.catalog.clone() as Arc<dyn DatabaseCatalog>
    }
}


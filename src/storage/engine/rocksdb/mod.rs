pub mod kv_store;

use std::sync::Arc;

use super::kv::catalog::database::KVDatabaseCatalog;
use crate::{catalog::database::DatabaseCatalog, storage::StorageEngine};

pub struct RocksDBStorageEngine {
    catalog: Arc<KVDatabaseCatalog>,
}

impl RocksDBStorageEngine {
    pub fn new(path: &str) -> RocksDBStorageEngine {
        let store = Arc::new(kv_store::RocksDBKVStore::new(path));
        RocksDBStorageEngine {
            catalog: Arc::new(KVDatabaseCatalog::new(store)),
        }
    }
}

impl StorageEngine for RocksDBStorageEngine {
    fn catalog(&self) -> Arc<dyn DatabaseCatalog> {
        self.catalog.clone() as Arc<dyn DatabaseCatalog>
    }
}

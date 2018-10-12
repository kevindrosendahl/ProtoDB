pub mod kv_store;

use std::sync::Arc;

use super::kv::catalog::database::KVDatabaseCatalog;
use crate::{
    catalog::database::DatabaseCatalog,
    storage::{errors::InternalStorageEngineError, StorageEngine},
};

pub struct RocksDBStorageEngine {
    catalog: Arc<KVDatabaseCatalog>,
}

impl RocksDBStorageEngine {
    pub fn new(path: &str) -> Result<RocksDBStorageEngine, rocksdb::Error> {
        let store = Arc::new(kv_store::RocksDBKVStore::new(path)?);
        Ok(RocksDBStorageEngine {
            catalog: Arc::new(KVDatabaseCatalog::new(store)),
        })
    }
}

impl StorageEngine for RocksDBStorageEngine {
    fn catalog(&self) -> Arc<dyn DatabaseCatalog> {
        self.catalog.clone() as Arc<dyn DatabaseCatalog>
    }
}

impl Into<InternalStorageEngineError> for rocksdb::Error {
    fn into(self) -> InternalStorageEngineError {
        InternalStorageEngineError {
            message: self.to_string(),
        }
    }
}

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
    pub fn try_new(path: &str) -> Result<RocksDBStorageEngine, InternalStorageEngineError> {
        let store = kv_store::RocksDBKVStore::try_new(path).map_err(|err| err.into())?;
        let catalog = KVDatabaseCatalog::try_new(Arc::new(store))?;
        Ok(RocksDBStorageEngine {
            catalog: Arc::new(catalog),
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

use std::sync::Arc;

use crate::storage::{
    engine::kv::store::{KVStore, KVStoreBytes, KVStoreWriteBatch},
    errors::InternalStorageEngineError,
};

#[derive(Debug)]
pub struct RocksDBKVStore {
    inner: Arc<rocksdb::DB>,
}

impl RocksDBKVStore {
    pub fn try_new(path: &str) -> Result<RocksDBKVStore, rocksdb::Error> {
        Ok(RocksDBKVStore {
            inner: Arc::new(rocksdb::DB::open_default(path)?),
        })
    }
}

impl KVStore for RocksDBKVStore {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, InternalStorageEngineError> {
        self.inner
            .clone()
            .get(key)
            .map_err(|err| err.into())
            .and_then(|v| Ok(v.map(|v| v.to_vec())))
    }

    fn prefix_iterator(&self, prefix: &[u8]) -> Box<dyn Iterator<Item = KVStoreBytes>> {
        Box::new(
            self.inner
                .clone()
                .prefix_iterator(prefix)
                .map(|(k, v)| (k.to_vec(), v.to_vec())),
        ) as Box<dyn Iterator<Item = KVStoreBytes>>
    }

    fn write(&self, batch: KVStoreWriteBatch) -> Result<(), InternalStorageEngineError> {
        let mut rocksdb_batch = rocksdb::WriteBatch::default();
        for (key, value) in batch.iter() {
            rocksdb_batch.put(key, value).unwrap();
        }

        self.inner
            .clone()
            .write(rocksdb_batch)
            .map_err(|err| err.into())
    }

    fn delete(&self, key: &[u8]) -> Result<(), InternalStorageEngineError> {
        self.inner.clone().delete(key).map_err(|err| err.into())
    }
}

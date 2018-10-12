use std::sync::Arc;

use crate::storage::engine::kv::store::{KVStore, KVStoreBytes, KVStoreWriteBatch};

#[derive(Debug)]
pub struct RocksDBKVStore {
    inner: Arc<rocksdb::DB>,
}

impl RocksDBKVStore {
    pub fn new(path: &str) -> RocksDBKVStore {
        RocksDBKVStore {
            inner: Arc::new(rocksdb::DB::open_default(path).unwrap()),
        }
    }
}

impl KVStore for RocksDBKVStore {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let store = self.inner.clone();
        store.get(key).unwrap().and_then(|v| Some(v.to_vec()))
    }

    fn prefix_iterator(&self, prefix: &[u8]) -> Box<dyn Iterator<Item = KVStoreBytes>> {
        let store = self.inner.clone();
        Box::new(
            store
                .prefix_iterator(prefix)
                .map(|(k, v)| (k.to_vec(), v.to_vec())),
        ) as Box<dyn Iterator<Item = KVStoreBytes>>
    }

    fn write(&self, batch: KVStoreWriteBatch) {
        let mut rocksdb_batch = rocksdb::WriteBatch::default();
        for (key, value) in batch.iter() {
            rocksdb_batch.put(key, value).unwrap();
        }

        let store = self.inner.clone();
        store.write(rocksdb_batch).unwrap()
    }
}


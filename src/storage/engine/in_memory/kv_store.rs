use std::{
    collections::{btree_map::Range, BTreeMap},
    iter::Map,
    ops::Bound,
    sync::{Arc, RwLock},
};

use crate::storage::engine::kv::store::{KVStore, KVStoreBytes, KVStoreWriteBatch};

type Inner = BTreeMap<Vec<u8>, Vec<u8>>;

#[derive(Debug, Default)]
pub struct InMemoryKVStore {
    inner: Arc<RwLock<Inner>>,
}

impl<'a> InMemoryKVStore {
    //    #[inline(always)]
    //    fn get_locked(store: &'a Inner, key: KVStoreKey) -> Option<KVStoreValue<'a>> {
    //
    //    }

    //    #[inline(always)]
    //    fn prefix_iter_locked<'a, T>(store: &'a Inner, prefix: KVStoreKey) -> T where T: Iterator + 'a
    //    {
    //        store
    //            .range((Bound::Included(prefix), Bound::Unbounded))
    //            .map(|(k, v)| (k.clone(), v.clone()))
    //    }

    //    #[inline(always)]
    //    fn write_locked(store: &'a mut Inner, batch: KVStoreWriteBatch<'a>) {
    //
    //    }
}

//pub struct InMemoryKVStorePrefixIterator<T>
//where
//    T: Iterator<Item = KVStoreBytes>,
//{
//    inner: T,
//}
//
//impl<T> Iterator for InMemoryKVStorePrefixIterator<T>
//where
//    T: Iterator<Item = KVStoreBytes>,
//{
//    type Item = KVStoreBytes;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        self.inner.next()
//    }
//}

impl KVStore for InMemoryKVStore {
    fn get(&self, key: &[u8]) -> Option<Box<[u8]>> {
        let store = self.inner.clone();
        let store = store.read().unwrap();
        store
            .get(&key.to_vec())
            .map(|v| v.clone().into_boxed_slice())
    }

    fn prefix_iter(&self, prefix: &[u8]) -> Box<dyn Iterator<Item = KVStoreBytes>> {
        let store = self.inner.clone();
        let store = store.read().unwrap();
        let store = store.clone();
        Box::new(PrefixIterator {
            inner: store
                .range((Bound::Included(prefix.to_vec()), Bound::Unbounded))
                .map(|(k, v)| (k.clone().into_boxed_slice(), v.clone().into_boxed_slice()) )
                .collect(),
            pos: 0,
        })
    }

    fn write(&self, batch: KVStoreWriteBatch) {
        let store = self.inner.clone();
        let mut store = store.write().unwrap();

        for (key, value) in batch.iter() {
            let key = key.to_vec();
            let value = value.to_vec();
            store.insert(key, value);
        }
    }
}

pub struct PrefixIterator {
    inner: Vec<KVStoreBytes>,
    pos: usize,
}

impl Iterator for PrefixIterator {
    type Item = KVStoreBytes;

    fn next(&mut self) -> Option<KVStoreBytes> {
        if self.pos >= self.inner.len() {
            return None;
        }

        let res = self.inner[self.pos].clone();
        self.pos += 1;
        Some(res)
    }
}

use crate::storage::errors::InternalStorageEngineError;

pub trait KVStore {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, InternalStorageEngineError>;

    fn prefix_iterator(&self, prefix: &[u8]) -> Box<dyn Iterator<Item = KVStoreBytes>>;

    fn bounded_prefix_iterator(
        &self,
        prefix: &[u8],
        bound: &[u8],
    ) -> Box<dyn Iterator<Item = KVStoreBytes>> {
        Box::new(KVStoreBoundedPrefixIterator {
            bound: bound.to_vec().into_boxed_slice(),
            inner: self.prefix_iterator(prefix),
        })
    }

    fn write(&self, batch: KVStoreWriteBatch) -> Result<(), InternalStorageEngineError>;

    fn delete(&self, key: &[u8]) -> Result<(), InternalStorageEngineError>;
}

pub type KVStoreBytes = (Vec<u8>, Vec<u8>);
pub type KVStoreWriteBatch<'a> = Vec<(&'a [u8], &'a [u8])>;

pub struct KVStoreBoundedPrefixIterator {
    bound: Box<[u8]>,
    inner: Box<dyn Iterator<Item = KVStoreBytes>>,
}

impl<'a> Iterator for KVStoreBoundedPrefixIterator {
    type Item = KVStoreBytes;

    fn next(&mut self) -> Option<KVStoreBytes> {
        let (key, value) = self.inner.next()?;
        if *key > *self.bound {
            return None;
        }

        Some((key, value))
    }
}

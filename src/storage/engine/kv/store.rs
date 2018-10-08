use std::ops::Deref;

pub trait KVStore {
    fn get(&self, key: &[u8]) -> Option<Box<[u8]>>;

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

    // TODO: return Result
    fn write(&self, batch: KVStoreWriteBatch);
}

pub type KVStoreBytes = (Box<[u8]>, Box<[u8]>);
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

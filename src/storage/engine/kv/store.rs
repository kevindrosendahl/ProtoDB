use std::ops::Deref;

pub trait KVStore {
    fn get(&self, key: &[u8]) -> Option<Box<[u8]>>;
    fn prefix_iter(&self, prefix: &[u8]) -> Box<dyn Iterator<Item = KVStoreBytes>>;
    // TODO: return Result
    fn write(&self, batch: KVStoreWriteBatch);
}

pub type KVStoreBytes = (Box<[u8]>, Box<[u8]>);

pub type KVStoreWriteBatch<'a> = Vec<(&'a [u8], &'a [u8])>;

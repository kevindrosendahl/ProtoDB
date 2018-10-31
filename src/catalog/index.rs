use std::sync::Arc;

use crate::{index::IndexAccessMethod, storage::errors::InternalStorageEngineError};

pub trait IndexCatalog {
    fn create_index(&self, field: i32) -> Result<usize, InternalStorageEngineError>;

    fn index_entry(&self, id: usize) -> Option<Arc<dyn IndexCatalogEntry>>;
}

pub trait IndexCatalogEntry {
    fn id(&self) -> u64;

    fn access_method(&self) -> Arc<dyn IndexAccessMethod>;
}

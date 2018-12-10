use std::{collections::HashSet, sync::Arc};

use super::errors::collection::{FindObjectError, InsertObjectError};
use crate::{
    index::IndexAccessMethod, schema::Schema, storage::errors::InternalStorageEngineError,
};

pub trait CollectionCatalogEntry {
    fn name(&self) -> &str;

    fn schema(&self) -> &Schema;

    fn create_index(&self, field: i32) -> Result<usize, InternalStorageEngineError>;

    fn index(&self, id: usize) -> Option<Arc<dyn IndexAccessMethod>>;

    fn find_all(
        &self,
        fields: Option<HashSet<i32>>,
    ) -> Box<dyn Iterator<Item = Result<Vec<u8>, InternalStorageEngineError>>>;

    fn find_object(&self, id: u64) -> Result<Option<Vec<u8>>, FindObjectError>;

    fn insert_object(&self, object: &[u8]) -> Result<(), InsertObjectError>;
}

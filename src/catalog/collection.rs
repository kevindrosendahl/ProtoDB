use std::sync::Arc;

use super::{
    errors::collection::{FindObjectError, InsertObjectError},
    index::IndexCatalog,
};
use crate::{
    schema::{DecodedObject, Schema},
    storage::errors::InternalStorageEngineError,
};

pub trait CollectionCatalogEntry {
    fn name(&self) -> &str;

    fn schema(&self) -> &Schema;

    fn indexes(&self) -> Arc<dyn IndexCatalog>;

    fn find_all_decoded(
        &self,
    ) -> Box<dyn Iterator<Item = Result<DecodedObject, InternalStorageEngineError>>>;

    fn find_object(&self, id: u64) -> Result<Option<Vec<u8>>, FindObjectError>;

    fn insert_object(&self, object: &[u8]) -> Result<(), InsertObjectError>;
}

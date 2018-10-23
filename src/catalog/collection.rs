use super::{
    errors::collection::{FindObjectError, InsertObjectError},
    index::IndexCatalog,
};
use crate::schema::Schema;

pub trait CollectionCatalogEntry {
    fn name(&self) -> &str;

    fn schema(&self) -> &Schema;

    fn indexes(&self) -> &IndexCatalog;

    fn find_object(&self, id: u64) -> Result<Option<Vec<u8>>, FindObjectError>;

    fn insert_object(&self, object: &[u8]) -> Result<(), InsertObjectError>;
}

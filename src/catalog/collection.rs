use super::errors::collection::{FindObjectError, InsertObjectError};
use crate::schema::Schema;

pub trait CollectionCatalogEntry {
    fn name(&self) -> &str;

    fn schema(&self) -> &Schema;

    fn find_object(&self, id: u64) -> Result<Option<Vec<u8>>, FindObjectError>;

    fn insert_object(&self, object: &[u8]) -> Result<(), InsertObjectError>;
}

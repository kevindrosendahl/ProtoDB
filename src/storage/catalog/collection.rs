use crate::storage::{errors, schema::Schema};

pub trait CollectionCatalogEntry {
    fn name(&self) -> &str;

    fn schema(&self) -> &Schema;

    fn find_object(&self, id: u64) -> Result<Vec<u8>, errors::collection::FindObjectError>;

    fn insert_object(&self, object: &[u8]) -> Result<(), errors::collection::InsertObjectError>;
}

pub mod engine;
pub mod errors;
pub mod schema;

use prost_types::DescriptorProto;

pub trait StorageEngine {
    fn create_database(&self, name: &str) -> Result<(), errors::database::CreateDatabaseError>;
    fn list_databases(&self) -> Vec<String>;

    fn create_collection(
        &self,
        database: &str,
        name: &str,
        schema: &DescriptorProto,
    ) -> Result<(), errors::collection::CreateCollectionError>;
    fn list_collections(
        &self,
        database: &str,
    ) -> Result<Vec<String>, errors::collection::ListCollectionsError>;

    fn insert_object(
        &self,
        database: &str,
        collection: &str,
        object: &[u8],
    ) -> Result<(), errors::collection::InsertObjectError>;
    fn find_object(
        &self,
        database: &str,
        collection: &str,
        id: u64,
    ) -> Result<Vec<u8>, errors::collection::FindObjectError>;
}

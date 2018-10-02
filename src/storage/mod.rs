pub mod engines;
pub mod errors;
pub mod schema;

use prost_types::DescriptorProto;

pub trait StorageEngine {
    fn create_database(&self, name: &str) -> Result<(), errors::CreateDatabaseError>;
    fn list_databases(&self) -> Vec<String>;

    fn create_collection(
        &self,
        database: &str,
        name: &str,
        schema: &DescriptorProto,
    ) -> Result<(), errors::CreateCollectionError>;
    fn list_collections(&self, database: &str)
        -> Result<Vec<String>, errors::ListCollectionsError>;

    //    fn insert_object(
    //        &self,
    //        database: &str,
    //        collection: &str,
    //        object: &[u8],
    //    ) -> Result<(), errors::InsertError>;
}

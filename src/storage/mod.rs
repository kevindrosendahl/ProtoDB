pub mod errors;
pub mod in_memory;

use prost_types::DescriptorProto;

pub trait StorageEngine {
    fn create_database(&self, name: &str) -> Result<(), errors::StorageError>;
    fn list_databases(&self) -> Vec<String>;

    fn create_collection(
        &self,
        database: &str,
        name: &str,
        schema: &DescriptorProto,
    ) -> Result<(), errors::StorageError>;
    fn list_collections(&self, database: &str) -> Result<Vec<String>, errors::StorageError>;
}

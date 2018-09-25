use prost_types::{DescriptorProto};

pub trait StorageEngine {
    fn list_databases(&self) -> Vec<String>;
    fn create_database(&self, name: &str);

    fn list_collections(&self, database: &str) -> Option<Vec<String>>;
    fn create_collection(&self, database: &str, name: &str, schema: &DescriptorProto);
}

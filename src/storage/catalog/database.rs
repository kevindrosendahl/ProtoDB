use std::clone::Clone;

use super::collection::CollectionCatalogEntry;
use crate::storage::errors;

use prost_types::DescriptorProto;

pub trait DatabaseCatalog {
    fn create_database(&self, name: &str) -> Result<(), errors::database::CreateDatabaseError>;
    fn list_databases(&self) -> Vec<String>;
    fn get_database(&self, name: &str) -> Option<Box<dyn DatabaseCatalogEntry>>;
}

pub trait DatabaseCatalogEntry {
    fn name(&self) -> &str;

    fn list_collections(&self) -> Vec<String>;
    fn create_collection(
        &self,
        name: &str,
        schema: &DescriptorProto,
    ) -> Result<(), errors::collection::CreateCollectionError>;
    fn get_collection(&self, name: &str) -> Option<Box<dyn CollectionCatalogEntry>>;
}

use std::sync::Arc;

use super::collection::CollectionCatalogEntry;
use crate::storage::errors;

use prost_types::DescriptorProto;

pub trait DatabaseCatalog<'a> {
    fn create_database(&self, name: &str) -> Result<(), errors::database::CreateDatabaseError>;

    fn list_databases(&self) -> Vec<String>;

    fn get_database_entry(&self, name: &str) -> Option<Arc<dyn DatabaseCatalogEntry<'a> + 'a>>;
}

pub trait DatabaseCatalogEntry<'a> {
    fn name(&self) -> &str;

    fn list_collections(&self) -> Vec<String>;

    fn create_collection(
        &self,
        name: &str,
        schema: &DescriptorProto,
    ) -> Result<(), errors::collection::CreateCollectionError>;

    fn get_collection_entry(&self, name: &str) -> Option<Arc<dyn CollectionCatalogEntry + 'a>>;
}

use std::sync::Arc;

use super::collection::CollectionCatalogEntry;
use super::errors::database::{CreateCollectionError, CreateDatabaseError};

use crate::wasm::ProtoDBModule;

use prost_types::DescriptorProto;

pub trait DatabaseCatalog {
    fn create_database(&self, name: &str) -> Result<(), CreateDatabaseError>;

    fn list_databases(&self) -> Vec<String>;

    fn get_database_entry(&self, name: &str) -> Option<Arc<dyn DatabaseCatalogEntry>>;
}

pub trait DatabaseCatalogEntry {
    fn name(&self) -> &str;

    fn list_collections(&self) -> Vec<String>;

    fn create_collection(
        &self,
        name: &str,
        descriptor: &DescriptorProto,
    ) -> Result<(), CreateCollectionError>;

    fn get_collection_entry(&self, name: &str) -> Option<Arc<dyn CollectionCatalogEntry>>;

    fn list_wasm_modules(&self) -> Vec<String>;

    fn create_wasm_module(&self, name: &str, module: ProtoDBModule);

    fn get_wasm_module(&self, name: &str) -> Option<Arc<ProtoDBModule>>;
}

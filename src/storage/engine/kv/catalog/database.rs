use std::{
    clone::Clone,
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use super::super::store::KVStore;
use super::collection::KVCollectionCatalogEntry;
use crate::storage::{
    catalog::collection::CollectionCatalogEntry,
    catalog::database::{DatabaseCatalog, DatabaseCatalogEntry},
    errors,
};

use prost_types::DescriptorProto;

pub struct KVDatabaseCatalog<'a> {
    databases: Arc<RwLock<BTreeMap<String, KVCollectionCatalogEntry<'a>>>>,
    kv_store: Box<dyn KVStore>,
}

impl<'a> KVDatabaseCatalog<'a> {
    pub fn new(kv_store: Box<dyn KVStore>) -> KVDatabaseCatalog<'a> {
        KVDatabaseCatalog {
            databases: Default::default(),
            kv_store,
        }
    }
}

impl<'a> DatabaseCatalog for KVDatabaseCatalog<'a> {
    fn create_database(&self, name: &str) -> Result<(), errors::database::CreateDatabaseError> {
        unimplemented!()
    }

    fn list_databases(&self) -> Vec<String> {
        unimplemented!()
    }

    fn get_database(&self, name: &str) -> Option<Box<dyn DatabaseCatalogEntry>> {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct KVDatabaseCatalogEntry<'a> {
    name: String,
    collections: Arc<RwLock<BTreeMap<String, KVCollectionCatalogEntry<'a>>>>,
}

impl<'a> KVDatabaseCatalogEntry<'a> {
    pub fn new(name: String) -> KVDatabaseCatalogEntry<'a> {
        KVDatabaseCatalogEntry {
            name,
            collections: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
}

impl<'a> DatabaseCatalogEntry for KVDatabaseCatalogEntry<'a> {
    fn name(&self) -> &str {
        unimplemented!()
    }

    fn list_collections(&self) -> Vec<String> {
        unimplemented!()
    }

    fn create_collection(
        &self,
        name: &str,
        schema: &DescriptorProto,
    ) -> Result<(), errors::collection::CreateCollectionError> {
        unimplemented!()
    }

    fn get_collection(&self, name: &str) -> Option<Box<dyn CollectionCatalogEntry>> {
        unimplemented!()
    }
}

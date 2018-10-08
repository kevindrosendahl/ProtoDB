use std::{
    //    clone::Clone,
    collections::BTreeMap,
    sync::{Arc, RwLock},
    ops::Deref,
};

use super::super::store::KVStore;
use super::collection::KVCollectionCatalogEntry;
use crate::storage::{
    catalog::collection::CollectionCatalogEntry,
    catalog::database::{DatabaseCatalog, DatabaseCatalogEntry},
    errors,
};

use prost_types::DescriptorProto;

pub struct KVDatabaseCatalog {
    kv_store: Arc<dyn KVStore>,

    databases: Arc<RwLock<BTreeMap<String, Arc<KVDatabaseCatalogEntry>>>>,
}

impl KVDatabaseCatalog {
    pub fn new(kv_store: Arc<dyn KVStore>) -> KVDatabaseCatalog {
        KVDatabaseCatalog {
            kv_store,
            databases: Default::default(),
        }
    }
}

impl<'a> DatabaseCatalog<'a> for KVDatabaseCatalog {
    fn create_database(&self, name: &str) -> Result<(), errors::database::CreateDatabaseError> {
        let dbs = self.databases.clone();
        let mut dbs = dbs.write().unwrap();
        if dbs.contains_key(name) {
            return Err(errors::database::CreateDatabaseError::DatabaseExists);
        }

        dbs.insert(
            name.to_string(),
            Arc::new(KVDatabaseCatalogEntry::new(name.to_string(), self.kv_store.clone())),
        );
        Ok(())
    }

    fn list_databases(&self) -> Vec<String> {
        let dbs = self.databases.clone();
        let dbs = dbs.read().unwrap();
        dbs.keys().cloned().collect()
    }

    fn get_database_entry(&self, name: &str) -> Option<Arc<dyn DatabaseCatalogEntry<'a> + 'a>> {
        let dbs = self.databases.clone();
        let dbs = dbs.read().unwrap();
        dbs.get(name)
            .cloned()
            .map(|e| e as Arc<dyn DatabaseCatalogEntry<'a> + 'a>)
    }
}

#[derive(Clone)]
pub struct KVDatabaseCatalogEntry {
    kv_store: Arc<dyn KVStore>,

    name: String,
    collections: Arc<RwLock<BTreeMap<String, KVCollectionCatalogEntry>>>,
}

impl<'a> KVDatabaseCatalogEntry {
    pub fn new(name: String, kv_store: Arc<dyn KVStore>) -> KVDatabaseCatalogEntry {
        KVDatabaseCatalogEntry {
            kv_store,

            name,
            collections: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
}

impl<'a> DatabaseCatalogEntry<'a> for KVDatabaseCatalogEntry {
    fn name(&self) -> &str {
        &self.name
    }

    fn list_collections(&self) -> Vec<String> {
        let collections = self.collections.clone();
        let collections = collections.read().unwrap();
        collections.keys().cloned().collect()
    }

    fn create_collection(
        &self,
        name: &str,
        descriptor: &DescriptorProto,
    ) -> Result<(), errors::collection::CreateCollectionError> {
        let collections = self.collections.clone();
        let mut collections = collections.write().unwrap();
        if collections.contains_key(name) {
            return Err(errors::collection::CreateCollectionError::CollectionExists);
        }

        let collection = KVCollectionCatalogEntry::new(
            self.kv_store.clone(),
            self.name.clone(),
            name.to_string(),
            descriptor,
        )?;
        collections.insert(name.to_string(), collection);
        Ok(())
    }

    fn get_collection_entry(&self, name: &str) -> Option<Arc<dyn CollectionCatalogEntry + 'a>> {
        unimplemented!()
    }
}

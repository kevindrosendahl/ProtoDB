use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

mod cache;
mod collection;
mod database;
use self::{collection::Collection, database::Database};

use crate::storage::{errors, StorageEngine};

use prost_types::DescriptorProto;

#[derive(Default)]
pub struct InMemoryStorageEngine {
    databases: Arc<RwLock<BTreeMap<String, Database>>>,
}

impl InMemoryStorageEngine {
    pub fn new() -> InMemoryStorageEngine {
        InMemoryStorageEngine {
            databases: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    fn list_databases(&self) -> Vec<String> {
        let dbs = self.databases.clone();
        let dbs = dbs.read().unwrap();
        dbs.keys().cloned().collect()
    }

    fn create_database(&self, name: &str) -> Result<(), errors::CreateDatabaseError> {
        let dbs = self.databases.clone();
        let mut dbs = dbs.write().unwrap();
        if dbs.contains_key(name) {
            return Err(errors::CreateDatabaseError::DatabaseExists);
        }

        dbs.insert(name.to_string(), Default::default());
        Ok(())
    }

    fn create_collection(
        &self,
        database: &str,
        name: &str,
        descriptor: &DescriptorProto,
    ) -> Result<(), errors::CreateCollectionError> {
        let dbs = self.databases.clone();
        let dbs = dbs.read().unwrap();
        let db = dbs
            .get(database)
            .ok_or_else(|| errors::CreateCollectionError::InvalidDatabase)?;

        let collections = db.collections.clone();
        let mut collections = collections.write().unwrap();
        if collections.contains_key(name) {
            return Err(errors::CreateCollectionError::CollectionExists);
        }

        let collection = Collection::new(descriptor)?;
        collections.insert(name.to_string(), collection);
        Ok(())
    }

    fn list_collections(
        &self,
        database: &str,
    ) -> Result<Vec<String>, errors::ListCollectionsError> {
        let dbs = self.databases.clone();
        let dbs = dbs.read().unwrap();
        let db = dbs
            .get(database)
            .ok_or_else(|| errors::ListCollectionsError::InvalidDatabase)?;

        let colls = db.collections.clone();
        let colls = colls.read().unwrap();
        Ok(colls.keys().cloned().collect())
    }
}

impl StorageEngine for InMemoryStorageEngine {
    fn list_databases(&self) -> Vec<String> {
        self.list_databases()
    }

    fn create_database(&self, name: &str) -> Result<(), errors::CreateDatabaseError> {
        self.create_database(name)
    }

    fn list_collections(
        &self,
        database: &str,
    ) -> Result<Vec<String>, errors::ListCollectionsError> {
        self.list_collections(database)
    }

    fn create_collection(
        &self,
        database: &str,
        name: &str,
        schema: &DescriptorProto,
    ) -> Result<(), errors::CreateCollectionError> {
        self.create_collection(database, name, schema)
    }
}

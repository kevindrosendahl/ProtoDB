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

    fn create_database(&self, name: &str) -> Result<(), errors::database::CreateDatabaseError> {
        let dbs = self.databases.clone();
        let mut dbs = dbs.write().unwrap();
        if dbs.contains_key(name) {
            return Err(errors::database::CreateDatabaseError::DatabaseExists);
        }

        dbs.insert(name.to_string(), Default::default());
        Ok(())
    }

    fn create_collection(
        &self,
        database: &str,
        name: &str,
        descriptor: &DescriptorProto,
    ) -> Result<(), errors::collection::CreateCollectionError> {
        let dbs = self.databases.clone();
        let dbs = dbs.read().unwrap();
        let db = dbs
            .get(database)
            .ok_or_else(|| errors::collection::CreateCollectionError::InvalidDatabase)?;

        let collections = db.collections.clone();
        let mut collections = collections.write().unwrap();
        if collections.contains_key(name) {
            return Err(errors::collection::CreateCollectionError::CollectionExists);
        }

        let collection = Collection::new(db.name.clone(), name.to_string(), descriptor)?;
        collections.insert(name.to_string(), collection);
        Ok(())
    }

    fn list_collections(
        &self,
        database: &str,
    ) -> Result<Vec<String>, errors::collection::ListCollectionsError> {
        let dbs = self.databases.clone();
        let dbs = dbs.read().unwrap();
        let db = dbs
            .get(database)
            .ok_or_else(|| errors::collection::ListCollectionsError::InvalidDatabase)?;

        let collections = db.collections.clone();
        let collections = collections.read().unwrap();
        Ok(collections.keys().cloned().collect())
    }

    fn insert_object(
        &self,
        database: &str,
        collection: &str,
        object: &[u8],
    ) -> Result<(), errors::collection::InsertObjectError> {
        let dbs = self.databases.clone();
        let dbs = dbs.read().unwrap();
        let db = dbs
            .get(database)
            .ok_or_else(|| errors::collection::InsertObjectError::InvalidDatabase)?;

        let collections = db.collections.clone();
        let collections = collections.write().unwrap();
        let collection = collections
            .get(collection)
            .ok_or_else(|| errors::collection::InsertObjectError::InvalidCollection)?;
        collection.insert_object(object)
    }
}

impl StorageEngine for InMemoryStorageEngine {
    fn create_database(&self, name: &str) -> Result<(), errors::database::CreateDatabaseError> {
        self.create_database(name)
    }

    fn list_databases(&self) -> Vec<String> {
        self.list_databases()
    }

    fn create_collection(
        &self,
        database: &str,
        name: &str,
        schema: &DescriptorProto,
    ) -> Result<(), errors::collection::CreateCollectionError> {
        self.create_collection(database, name, schema)
    }

    fn list_collections(
        &self,
        database: &str,
    ) -> Result<Vec<String>, errors::collection::ListCollectionsError> {
        self.list_collections(database)
    }

    fn insert_object(
        &self,
        database: &str,
        collection: &str,
        object: &[u8],
    ) -> Result<(), errors::collection::InsertObjectError> {
        self.insert_object(database, collection, object)
    }
}

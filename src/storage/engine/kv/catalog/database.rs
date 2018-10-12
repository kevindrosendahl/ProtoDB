use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use super::{
    KEY_DELIMITER,
    collection::KVCollectionCatalogEntry,
};
use super::super::store::KVStore;
use crate::{
    catalog::{
        collection::CollectionCatalogEntry,
        database::{DatabaseCatalog, DatabaseCatalogEntry},
        errors::database::{CreateCollectionError, CreateDatabaseError},
    },
    storage::errors::InternalStorageEngineError,
};

use prost_types::DescriptorProto;

const SYSTEM_KEY_PREFIX: &str = "__system";
// FIXME: make this const or lazy_static
//const DATABASES_PREFIX: &str = SYSTEM_PREFIX + KEY_DELIMITER + DATABASES_DELIMITER + KEY_DELIMITER;

pub struct KVDatabaseCatalog {
    kv_store: Arc<dyn KVStore>,

    databases: Arc<RwLock<BTreeMap<String, Arc<KVDatabaseCatalogEntry>>>>,
}

impl KVDatabaseCatalog {
    pub fn new(kv_store: Arc<dyn KVStore>) -> Result<KVDatabaseCatalog, InternalStorageEngineError> {
        let catalog = KVDatabaseCatalog {
            kv_store,
            databases: Default::default(),
        };
        catalog.init().and(Ok(catalog))
    }

    fn init(&self) -> Result<(), InternalStorageEngineError> {
        Ok(())
    }
}

impl DatabaseCatalog for KVDatabaseCatalog {
    fn create_database(&self, name: &str) -> Result<(), CreateDatabaseError> {
        let dbs = self.databases.clone();
        let mut dbs = dbs.write().unwrap();
        if dbs.contains_key(name) {
            return Err(CreateDatabaseError::DatabaseExists);
        }

        dbs.insert(
            name.to_string(),
            Arc::new(KVDatabaseCatalogEntry::new(
                name.to_string(),
                self.kv_store.clone(),
            )),
        );
        Ok(())
    }

    fn list_databases(&self) -> Vec<String> {
        let dbs = self.databases.clone();
        let dbs = dbs.read().unwrap();
        dbs.keys().cloned().collect()
    }

    fn get_database_entry(&self, name: &str) -> Option<Arc<dyn DatabaseCatalogEntry>> {
        let dbs = self.databases.clone();
        let dbs = dbs.read().unwrap();
        dbs.get(name)
            .cloned()
            .map(|e| e as Arc<dyn DatabaseCatalogEntry>)
    }
}

#[derive(Clone)]
pub struct KVDatabaseCatalogEntry {
    kv_store: Arc<dyn KVStore>,

    name: String,
    collections: Arc<RwLock<BTreeMap<String, Arc<KVCollectionCatalogEntry>>>>,
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

impl DatabaseCatalogEntry for KVDatabaseCatalogEntry {
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
    ) -> Result<(), CreateCollectionError> {
        let collections = self.collections.clone();
        let mut collections = collections.write().unwrap();
        if collections.contains_key(name) {
            return Err(CreateCollectionError::CollectionExists);
        }

        let collection = KVCollectionCatalogEntry::new(
            self.kv_store.clone(),
            self.name.clone(),
            name.to_string(),
            descriptor,
        )?;
        let collection = Arc::new(collection);
        collections.insert(name.to_string(), collection);
        Ok(())
    }

    fn get_collection_entry(&self, name: &str) -> Option<Arc<dyn CollectionCatalogEntry>> {
        let collections = self.collections.clone();
        let collections = collections.read().unwrap();
        collections
            .get(name)
            .cloned()
            .map(|e| e as Arc<dyn CollectionCatalogEntry>)
    }
}

#[inline(always)]
// FIXME: make this const or lazy_static
fn databases_key_prefix() -> String {
    format!(
        "{system_prefix}{delimiter}databases{delimiter}",
        system_prefix = SYSTEM_KEY_PREFIX,
        delimiter = KEY_DELIMITER,
    )
}

#[inline(always)]
fn database_key(name: &str) -> String {
    format!(
        "{databases_prefix}{name}",
        databases_prefix = databases_key_prefix(),
        name = name,
    )
}

#[inline(always)]
fn database_key_prefix(name: &str) -> String {
    format!(
        "{system_prefix}{delimiter}database{delimiter}{name}{delimiter}",
        system_prefix = SYSTEM_KEY_PREFIX,
        delimiter = KEY_DELIMITER,
        name = name,
    )
}

#[inline(always)]
fn collections_key_prefix(database: &str) -> String {
    format!(
        "{database_prefix}collections",
        database_prefix = database_key_prefix(database),
    )
}

#[inline(always)]
fn collection_key(database: &str, name: &str) -> String {
    format!(
        "{collections_prefix}{delimiter}{name}",
        collections_prefix = collections_key_prefix(database),
        delimiter = KEY_DELIMITER,
        name = name,
    )
}

#[inline(always)]
fn collection_key_prefix(database: &str, name: &str) -> String {
    format!(
        "{database_prefix}collection{delimiter}{name}{delimiter}",
        database_prefix = database_key_prefix(database),
        delimiter = KEY_DELIMITER,
        name = name,
    )
}

#[inline(always)]
fn descriptor_key(database: &str, collection: &str) -> String {
    format!(
        "{collection_prefix}descriptor",
        collection_prefix = collection_key_prefix(database, collection),
    )
}
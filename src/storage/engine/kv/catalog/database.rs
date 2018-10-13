use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use super::super::store::{KVStore, KVStoreWriteBatch};
use super::{
    collection::KVCollectionCatalogEntry, delimiter_prefix_bound, key_suffix, KEY_DELIMITER,
};
use crate::{
    catalog::{
        collection::CollectionCatalogEntry,
        database::{DatabaseCatalog, DatabaseCatalogEntry},
        errors::database::{CreateCollectionError, CreateDatabaseError},
    },
    schema::errors::SchemaError,
    storage::errors::InternalStorageEngineError,
};

use prost::Message;
use prost_types::DescriptorProto;

const SYSTEM_KEY_PREFIX: &str = "__system";
// FIXME: make this const or lazy_static
//const DATABASES_PREFIX: &str = SYSTEM_PREFIX + KEY_DELIMITER + DATABASES_DELIMITER + KEY_DELIMITER;

pub struct KVDatabaseCatalog {
    kv_store: Arc<dyn KVStore>,

    databases: Arc<RwLock<BTreeMap<String, Arc<KVDatabaseCatalogEntry>>>>,
}

impl KVDatabaseCatalog {
    pub fn new(
        kv_store: Arc<dyn KVStore>,
    ) -> Result<KVDatabaseCatalog, InternalStorageEngineError> {
        let catalog = KVDatabaseCatalog {
            kv_store,
            databases: Default::default(),
        };

        catalog.init().and(Ok(catalog))
    }

    fn init(&self) -> Result<(), InternalStorageEngineError> {
        // iterate over the database keyspace and initialize a database catalog entry for each
        let (start, end) = delimiter_prefix_bound(databases_key_prefix());
        for (key, _) in self.kv_store.clone().bounded_prefix_iterator(&start, &end) {
            // FIXME: handle this error
            let database = database_from_key(&String::from_utf8(key.to_vec()).unwrap());
            self.init_database(database)?;
        }

        Ok(())
    }

    fn init_database(&self, name: String) -> Result<(), InternalStorageEngineError> {
        let entry = KVDatabaseCatalogEntry::new(name.clone(), self.kv_store.clone());
        entry.init()?;

        let databases = self.databases.clone();
        let mut databases = databases.write().unwrap();
        databases.insert(
            name.clone(),
            Arc::new(entry),
        );
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

        // write the database into the kv store
        let database_key = database_key(&name);
        let mut batch = KVStoreWriteBatch::new();
        let empty = vec![];
        batch.push((database_key.as_bytes(), &empty));
        self.kv_store.clone().write(batch).map_err(|err| {
            // not really sure what can be done if this fails besides log it
            // return the original error that made this fail
            let _ = self.kv_store.clone().delete(database_key.as_bytes());
            err
        })?;

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

    fn init(&self) -> Result<(), InternalStorageEngineError> {
        // iterate over the collection keyspace and initialize a collection catalog entry for each
        let (start, end) = delimiter_prefix_bound(collections_key_prefix(&self.name));
        for (key, _) in self.kv_store.clone().bounded_prefix_iterator(&start, &end) {
            // FIXME: handle this error
            let collection =
                collection_from_key(&self.name, &String::from_utf8(key.to_vec()).unwrap());
            self.init_collection(collection)?;
        }

        Ok(())
    }

    fn init_collection(&self, name: String) -> Result<(), InternalStorageEngineError> {
        let descriptor_key = descriptor_key(&self.name, &name);
        let descriptor_bytes = self.kv_store.clone().get(descriptor_key.as_bytes())?;
        let descriptor_bytes = descriptor_bytes.unwrap_or_else(|| {
            panic!(format!(
                "missing schema descriptor for {}/{}",
                &self.name, &name
            ))
        });

        let descriptor = DescriptorProto::decode(&descriptor_bytes).unwrap_or_else(|err| {
            panic!(format!(
                "error decoding schema descriptor for {}/{}: {}",
                &self.name, &name, err,
            ))
        });

        let entry = KVCollectionCatalogEntry::new(
            self.kv_store.clone(),
            self.name.clone(),
            name.clone(),
            &descriptor,
        ).unwrap_or_else(|err| {
            panic!(format!(
                "error creating collection catalog entry for {}/{}: {}",
                &self.name, &name, err,
            ))
        });

        let collections = self.collections.clone();
        let mut collections = collections.write().unwrap();
        collections.insert(name.to_string(), Arc::new(entry));
        Ok(())
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

        let entry = KVCollectionCatalogEntry::new(
            self.kv_store.clone(),
            self.name.clone(),
            name.to_string(),
            descriptor,
        )?;

        // write the collection into the kv store
        let collection_key = collection_key(&self.name, &name);
        let mut batch = KVStoreWriteBatch::new();
        let empty = vec![];
        batch.push((collection_key.as_bytes(), &empty));
        self.kv_store.clone().write(batch).map_err(|err| {
            // not really sure what can be done if this fails besides log it
            // return the original error that made this fail
            let _ = self.kv_store.clone().delete(collection_key.as_bytes());
            err
        })?;

        // encode the descriptor into a buffer
        let mut buf = Vec::new();
        descriptor
            .encode(&mut buf)
            .map_err(|err| SchemaError::EncodingError(err.to_string()))?;


        // write the descriptor buffer into the catalog
        let descriptor_key = descriptor_key(&self.name, &name);
        let mut batch = KVStoreWriteBatch::new();
        batch.push((descriptor_key.as_bytes(), &buf));
        self.kv_store.clone().write(batch).map_err(|err| {
            // not really sure what can be done if these fail besides log it
            // return the original error that made this fail
            let _ = self.kv_store.clone().delete(collection_key.as_bytes());
            let _ = self.kv_store.clone().delete(descriptor_key.as_bytes());
            err
        })?;

        collections.insert(name.to_string(), Arc::new(entry));
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
fn database_from_key(key: &str) -> String {
    let prefix = databases_key_prefix();
    key_suffix(&prefix, key)
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
        "{database_prefix}collections{delimiter}",
        database_prefix = database_key_prefix(database),
        delimiter = KEY_DELIMITER,
    )
}

#[inline(always)]
fn collection_key(database: &str, name: &str) -> String {
    format!(
        "{collections_prefix}{name}",
        collections_prefix = collections_key_prefix(database),
        name = name,
    )
}

#[inline(always)]
fn collection_from_key(database: &str, key: &str) -> String {
    let prefix = collections_key_prefix(database);
    key_suffix(&prefix, key)
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

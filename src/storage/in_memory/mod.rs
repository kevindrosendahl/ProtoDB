use std::{
    collections::BTreeMap,
    io::Cursor,
    ops::Bound,
    sync::{Arc, Mutex, RwLock},
};

use prost_types::{DescriptorProto};

mod database;
use self::database::Database;

use crate::storage::storage_engine::StorageEngine;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

const DATABASE_PREFIX: &'static str = "database";
const DATABASES_PREFIX: &'static str = "databases";
const DELIMITER: &'static str = "/";

pub struct InMemoryStorageEngine {
    cache: Arc<RwLock<BTreeMap<Vec<u8>, Vec<u8>>>>,
    database_id_counter: Arc<Mutex<u64>>,
}

impl InMemoryStorageEngine {
    pub fn new() -> InMemoryStorageEngine {
        InMemoryStorageEngine {
            cache: Arc::new(RwLock::new(BTreeMap::new())),
            database_id_counter: Arc::new(Mutex::new(1)),
        }
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let cache = self.cache.clone();
        let cache = cache.read().unwrap();
        cache.get(key).map(|v| v.clone())
    }

    fn get_range(&self, lower: &[u8], upper: &[u8]) -> Option<Vec<(Vec<u8>, Vec<u8>)>> {
        let cache = self.cache.clone();
        let cache = cache.read().unwrap();
        Some(
            cache
                .range((
                    Bound::Included(lower.to_vec()),
                    Bound::Included(upper.to_vec()),
                ))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        )
    }

    fn put(&self, key: &[u8], value: &[u8]) {
        let cache = self.cache.clone();
        cache.write().unwrap().insert(key.to_vec(), value.to_vec());
    }

    fn get_databases(&self) -> Vec<(String, u64)> {
        let lower = format!("{}{}", DATABASES_PREFIX, DELIMITER);
        let delimiter_plus_1 = DELIMITER.as_bytes()[0] + 1;
        let upper = format!("{}{}", DATABASES_PREFIX, delimiter_plus_1);

        self.get_range(lower.as_bytes(), upper.as_bytes())
            .unwrap()
            .iter()
            .map(|(k, v)| {
                let name = InMemoryStorageEngine::database_name(
                    String::from_utf8(k.clone()).unwrap().as_ref(),
                );
                let id = InMemoryStorageEngine::database_id(&v);
                (name, id)
            })
            .collect()
    }

    fn create_database(&self, name: &str) {
        let mut count = self.database_id_counter.lock().unwrap();
        let curr = *count;
        *count += 1;

        let key = InMemoryStorageEngine::database_entry_key(name);
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(curr).unwrap();
        self.put(key.as_bytes(), &wtr);
    }

    fn database_entry_key(name: &str) -> String {
        format!("{}{}{}", DATABASES_PREFIX, DELIMITER, name)
    }

    fn database_name(key: &str) -> String {
        let v: Vec<&str> = key.split(DELIMITER).collect();
        v.last().unwrap().to_string()
    }

    fn database_id(v: &Vec<u8>) -> u64 {
        Cursor::new(v).read_u64::<LittleEndian>().unwrap()
    }

    fn get_database_id(&self, name: &str) -> Option<u64> {
        let key = InMemoryStorageEngine::database_entry_key(name);
        self.get(key.as_ref())
            .map(|v| InMemoryStorageEngine::database_id(&v))
    }
}

impl StorageEngine for InMemoryStorageEngine {
    fn list_databases(&self) -> Vec<String> {
        self.get_databases()
            .iter()
            .map(|(name, id)| {
                println!("{}: {}", name, id);
                name.clone()
            })
            .collect()
    }

    fn create_database(&self, name: &str) {
        self.create_database(name)
    }

    fn list_collections(&self, database: &str) -> Option<Vec<String>> {
        None
    }
    fn create_collection(&self, database: &str, name: &str, schema: &DescriptorProto) {
        println!("{}", schema.name());
        println!("{:?}", schema);
        println!("{:?}", schema.extension);
        println!("{:?}", schema.field[0]);
        println!("{:?}", schema.field[0].options.clone().unwrap());
    }
}

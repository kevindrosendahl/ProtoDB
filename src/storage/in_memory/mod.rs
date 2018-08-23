use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use super::storage_engine::StorageEngine;
use super::super::database;

pub struct InMemoryStorageEngine {
    databases: Arc<RwLock<HashSet<String>>>
}

impl InMemoryStorageEngine {
    pub fn new() -> InMemoryStorageEngine {
        InMemoryStorageEngine { databases: Arc::new(RwLock::new(HashSet::new())) }
    }
}

impl StorageEngine for InMemoryStorageEngine {
    fn list_databases(&self) -> Vec<String> {
        let databases = self.databases.clone();
        let databases = databases.read().unwrap();
        databases.iter().map(|s| s.clone()).collect()
    }

    fn create_database(&mut self, name: &str) {
        let databases = self.databases.clone();
        databases.write().unwrap().insert(String::from(name));
    }

    fn get_database(&self, name: &str) -> Option<Box<database::Database>> {
        let databases = self.databases.clone();
        if !databases.read().unwrap().contains(name) {
            return None
        }

        Some(Box::new(Database { name: name.to_string() }))
    }
}

struct Database {
    name: String,
}

impl database::Database for Database {
    fn name(&self) -> String {
        self.name.clone()
    }
}
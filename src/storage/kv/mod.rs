use storage::storage_engine::StorageEngine;

const DATABASES_PREFIX: &'static str = "databases";
const DELIMITER: &'static str = "/";

pub trait KVStorageEngine {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
    fn get_range(&self, lower: &[u8], upper: &[u8]) -> Option<Vec<Vec<u8>>>;

    fn put(&self, key: &[u8], value: &[u8]);
}

impl<T> StorageEngine for T
    where T: KVStorageEngine {
    fn list_databases(&self) -> Vec<String> {
        let lower = format!("{}{}", DATABASES_PREFIX, DELIMITER);
        let delimiter_plus_1 = DELIMITER.as_bytes()[0] + 1;
        let upper = format!("{}{}", DATABASES_PREFIX, delimiter_plus_1);

        let databases = self.get_range(lower.as_bytes(), upper.as_bytes()).unwrap();
        databases.iter().map(|b| String::from_utf8(b.clone()).unwrap()).collect()
    }

    fn create_database(&self, name: &str) {
        let key = format!("{}{}{}", DATABASES_PREFIX, DELIMITER, name);
        self.put(key.as_bytes(), name.as_bytes());
    }
}
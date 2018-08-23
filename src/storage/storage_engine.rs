use super::super::database::{Database};

pub trait StorageEngine {
    //fn list_databases(&self) -> Result<Vec<String>>;
    fn list_databases(&self) -> Vec<String>;
    //fn create_database(&mut self, name: &str) -> Result<()>;
    fn create_database(&mut self, name: &str);
    //fn get_database(&self, name: &str) -> Result<Option<Database>>;
    fn get_database(&self, name: &str) -> Option<Box<Database>>;
}
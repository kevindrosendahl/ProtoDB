pub trait StorageEngine {
    //fn list_databases(&self) -> Result<Vec<String>>;
    fn list_databases(&self) -> Vec<String>;
    //fn create_database(&mut self, name: &str) -> Result<()>;
    fn create_database(&self, name: &str);
}

pub mod catalog;
pub mod engine;
pub mod errors;
pub mod schema;

use std::clone::Clone;

pub trait StorageEngine {
    fn catalog(&self) -> Box<dyn catalog::database::DatabaseCatalog>;
}

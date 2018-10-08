pub mod catalog;
pub mod engine;
pub mod errors;
pub mod schema;

use std::sync::Arc;

pub trait StorageEngine {
    fn catalog(&self) -> Arc<dyn catalog::database::DatabaseCatalog>;
}

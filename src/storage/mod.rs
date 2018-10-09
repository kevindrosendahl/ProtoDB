pub mod engine;

use std::sync::Arc;

use crate::catalog;

pub trait StorageEngine {
    fn catalog(&self) -> Arc<dyn catalog::database::DatabaseCatalog>;
}

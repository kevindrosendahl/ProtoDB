pub mod engine;

use std::sync::Arc;

use crate::catalog::database::DatabaseCatalog;

pub trait StorageEngine {
    fn catalog(&self) -> Arc<dyn DatabaseCatalog>;
}

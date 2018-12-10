use std::sync::Arc;

use super::super::generated::protodb::index;
use super::Handler;

use crate::catalog::{collection::CollectionCatalogEntry, database::DatabaseCatalogEntry};

use tower_grpc::Request;

impl Handler {
    pub(super) fn handle_create_index(
        &mut self,
        request: &Request<index::CreateIndexRequest>,
    ) -> index::CreateIndexResponse {
        self.storage_engine
            .clone()
            .catalog()
            .get_database_entry(&request.get_ref().database)
            .ok_or(index::create_index_response::ErrorCode::InvalidDatabase)
            .and_then(|db: Arc<dyn DatabaseCatalogEntry>| {
                db.get_collection_entry(&request.get_ref().collection)
                    .ok_or(index::create_index_response::ErrorCode::InvalidCollection)
            })
            .and_then(|collection: Arc<dyn CollectionCatalogEntry>| {
                collection
                    .create_index(request.get_ref().field)
                    .map_err(|_| index::create_index_response::ErrorCode::InternalError)
            })
            .and_then(|index_id| {
                Ok(index::CreateIndexResponse {
                    error_code: index::create_index_response::ErrorCode::NoError as i32,
                    index_id: index_id as u64,
                })
            })
            .unwrap_or_else(|error_code| index::CreateIndexResponse {
                error_code: error_code as i32,
                index_id: 0,
            })
    }
}

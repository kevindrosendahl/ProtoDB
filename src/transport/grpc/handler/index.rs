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
            .and_then(|id| {
                Ok(index::CreateIndexResponse {
                    error_code: index::create_index_response::ErrorCode::NoError as i32,
                    id: id as u64,
                })
            })
            .unwrap_or_else(|error_code| index::CreateIndexResponse {
                error_code: error_code as i32,
                id: 0,
            })
    }

    pub(super) fn handle_get_index(
        &mut self,
        request: &Request<index::GetIndexRequest>,
    ) -> index::GetIndexResponse {
        self.storage_engine
            .clone()
            .catalog()
            .get_database_entry(&request.get_ref().database)
            .ok_or(index::get_index_response::ErrorCode::InvalidDatabase)
            .and_then(|db: Arc<dyn DatabaseCatalogEntry>| {
                db.get_collection_entry(&request.get_ref().collection)
                    .ok_or(index::get_index_response::ErrorCode::InvalidCollection)
            })
            .and_then(|collection: Arc<dyn CollectionCatalogEntry>| {
                collection
                    .index(request.get_ref().id as usize)
                    .ok_or(index::get_index_response::ErrorCode::InvalidId)
            })
            .and_then(|index| {
                Ok(index::GetIndexResponse {
                    error_code: index::create_index_response::ErrorCode::NoError as i32,
                    index: Some(index::Index {
                        database: request.get_ref().database.clone(),
                        collection: request.get_ref().collection.clone(),
                        id: request.get_ref().id,
                        field: index.field(),
                    }),
                })
            })
            .unwrap_or_else(|error_code| index::GetIndexResponse {
                error_code: error_code as i32,
                index: None,
            })
    }

    pub(super) fn handle_list_indexes(
        &mut self,
        _request: &Request<index::ListIndexesRequest>,
    ) -> index::ListIndexesResponse {
        unimplemented!()
    }
}

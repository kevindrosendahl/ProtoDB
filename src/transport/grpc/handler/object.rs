use std::{error::Error, sync::Arc};

use super::super::generated::protodb::object;
use super::Handler;

use crate::catalog::{collection::CollectionCatalogEntry, database::DatabaseCatalogEntry, errors};

use tower_grpc::Request;

impl Handler {
    pub(super) fn handle_insert_object(
        &mut self,
        request: &Request<object::InsertObjectRequest>,
    ) -> object::InsertObjectResponse {
        self.storage_engine
            .clone()
            .catalog()
            .get_database_entry(&request.get_ref().database)
            .ok_or((object::insert_object_response::ErrorCode::InvalidDatabase, None))
            .and_then(|db: Arc<dyn DatabaseCatalogEntry>| {
                db
                    .get_collection_entry(&request.get_ref().collection)
                    .ok_or((object::insert_object_response::ErrorCode::InvalidCollection, None))
            })
            .and_then(|collection: Arc<dyn CollectionCatalogEntry>| {
                collection.insert_object(&request.get_ref().object)
                    .map_err(|err| match err {
                        errors::collection::InsertObjectError::InternalStorageEngineError(_) => {
                            (object::insert_object_response::ErrorCode::InternalError, None)
                        }
                        errors::collection::InsertObjectError::ObjectExists => {
                            (object::insert_object_response::ErrorCode::ObjectExists, None)
                        }
                        errors::collection::InsertObjectError::ObjectError(err) => {
                            let object_error = object::insert_object_response::ObjectError {
                                // FIXME: add match
                                code: object::insert_object_response::object_error::ObjectErrorCode::DecodeError as i32,
                                message: err.description().into(),
                            };
                            (object::insert_object_response::ErrorCode::InvalidDatabase, Some(object_error))
                        }
                    })
            })
            .and(
                Ok(object::InsertObjectResponse {
                    error_code: object::insert_object_response::ErrorCode::NoError
                        as i32,
                    object_error: None,
                })
            )
            .unwrap_or_else(|(error_code, object_error)| {
                object::InsertObjectResponse {
                    error_code: error_code as i32,
                    object_error,
                }
            })
    }

    pub(super) fn handle_find_object(
        &mut self,
        request: &Request<object::FindObjectRequest>,
    ) -> object::FindObjectResponse {
        self.storage_engine
            .clone()
            .catalog()
            .get_database_entry(&request.get_ref().database)
            .ok_or(object::find_object_response::ErrorCode::InvalidDatabase)
            .and_then(|db: Arc<dyn DatabaseCatalogEntry>| {
                db.get_collection_entry(&request.get_ref().collection)
                    .ok_or(object::find_object_response::ErrorCode::InvalidCollection)
            })
            .and_then(|collection: Arc<dyn CollectionCatalogEntry>| {
                // FIXME: handle object error
                Ok(collection.find_object(request.get_ref().id).unwrap())
            })
            .and_then(|object: Option<Vec<u8>>| {
                Ok(match object {
                    Some(object) => object::FindObjectResponse {
                        error_code: object::find_object_response::ErrorCode::NoError as i32,
                        object,
                    },
                    None => object::FindObjectResponse {
                        error_code: object::find_object_response::ErrorCode::InvalidId as i32,
                        object: vec![],
                    },
                })
            })
            .unwrap_or_else(|error_code| object::FindObjectResponse {
                error_code: error_code as i32,
                object: vec![],
            })
    }
}

use std::{error::Error, sync::Arc};

use super::super::generated::protodb::collection;
use super::Handler;

use crate::{
    catalog::{collection::CollectionCatalogEntry, database::DatabaseCatalogEntry, errors},
    schema::errors::SchemaError,
};

use tower_grpc::Request;

macro_rules! create_collection_err {
    ( $error_code:ident, $schema_error:expr ) => {
        (
            collection::create_collection_response::ErrorCode::$error_code,
            $schema_error,
        )
    };
}

macro_rules! create_collection_schema_err {
    ( $code:ident, $message:expr ) => {
        create_collection_err!(
            SchemaError,
            Some(collection::create_collection_response::SchemaError {
                code: collection::create_collection_response::schema_error::SchemaErrorCode::$code
                    as i32,
                message: $message,
            })
        )
    };
}

impl Handler {
    pub(super) fn handle_create_collection(
        &mut self,
        request: &Request<collection::CreateCollectionRequest>,
    ) -> collection::CreateCollectionResponse {
        self.storage_engine
            .clone()
            .catalog()
            .get_database_entry(&request.get_ref().database)
            .ok_or(create_collection_err!(InvalidDatabase, None))
            .and_then(|db: Arc<dyn DatabaseCatalogEntry>| {
                db.create_collection(
                    &request.get_ref().name,
                    &request.get_ref().schema.clone().unwrap(),
                )
                .map_err(|err| match err {
                    errors::database::CreateCollectionError::CollectionExists => {
                        create_collection_err!(CollectionExists, None)
                    }
                    errors::database::CreateCollectionError::InternalStorageEngineError(_) => {
                        create_collection_err!(InternalError, None)
                    }
                    errors::database::CreateCollectionError::SchemaError(err) => match err {
                        SchemaError::EncodingError(msg) => {
                            create_collection_schema_err!(EncodingError, msg)
                        }
                        SchemaError::InvalidFieldType((field, label, type_)) => {
                            create_collection_schema_err!(
                                InvalidFieldType,
                                format!(
                                    "invalid field type (field {}, type {:?} {:?})",
                                    field, label, type_
                                )
                            )
                        }
                        SchemaError::InvalidIdType(type_) => create_collection_schema_err!(
                            InvalidIdType,
                            format!("invalid id type ({})", type_)
                        ),
                        SchemaError::MissingIdField => {
                            create_collection_schema_err!(MissingIdField, err.description().into())
                        }
                    },
                })
            })
            .and(Ok(collection::CreateCollectionResponse {
                error_code: collection::create_collection_response::ErrorCode::NoError as i32,
                schema_error: None,
            }))
            .unwrap_or_else(
                |(error_code, schema_error)| collection::CreateCollectionResponse {
                    error_code: error_code as i32,
                    schema_error,
                },
            )
    }

    pub(super) fn handle_list_collections(
        &mut self,
        request: &Request<collection::ListCollectionsRequest>,
    ) -> collection::ListCollectionsResponse {
        self.storage_engine
            .clone()
            .catalog()
            .get_database_entry(&request.get_ref().database)
            .ok_or(collection::list_collections_response::ErrorCode::InvalidDatabase)
            .and_then(|db: Arc<dyn DatabaseCatalogEntry>| {
                Ok(collection::ListCollectionsResponse {
                    error_code: collection::list_collections_response::ErrorCode::NoError as i32,
                    collections: db.list_collections(),
                })
            })
            .unwrap_or_else(|error_code| collection::ListCollectionsResponse {
                error_code: error_code as i32,
                collections: Vec::new(),
            })
    }

    pub(super) fn handle_insert_object(
        &mut self,
        request: &Request<collection::InsertObjectRequest>,
    ) -> collection::InsertObjectResponse {
        self.storage_engine
            .clone()
            .catalog()
            .get_database_entry(&request.get_ref().database)
            .ok_or((collection::insert_object_response::ErrorCode::InvalidDatabase, None))
            .and_then(|db: Arc<dyn DatabaseCatalogEntry>| {
                db
                    .get_collection_entry(&request.get_ref().collection)
                    .ok_or((collection::insert_object_response::ErrorCode::InvalidCollection, None))
            })
            .and_then(|collection: Arc<dyn CollectionCatalogEntry>| {
                collection.insert_object(&request.get_ref().object)
                    .map_err(|err| match err {
                        errors::collection::InsertObjectError::InternalStorageEngineError(_) => {
                            (collection::insert_object_response::ErrorCode::InternalError, None)
                        }
                        errors::collection::InsertObjectError::ObjectExists => {
                            (collection::insert_object_response::ErrorCode::ObjectExists, None)
                        }
                        errors::collection::InsertObjectError::ObjectError(err) => {
                            let object_error = collection::insert_object_response::ObjectError {
                                // FIXME: add match
                                code: collection::insert_object_response::object_error::ObjectErrorCode::DecodeError as i32,
                                message: err.description().into(),
                            };
                            (collection::insert_object_response::ErrorCode::InvalidDatabase, Some(object_error))
                        }
                    })
            })
            .and(
                Ok(collection::InsertObjectResponse {
                    error_code: collection::insert_object_response::ErrorCode::NoError
                        as i32,
                    object_error: None,
                })
            )
            .unwrap_or_else(|(error_code, object_error)| {
                collection::InsertObjectResponse {
                    error_code: error_code as i32,
                    object_error,
                }
            })
    }

    pub(super) fn handle_find_object(
        &mut self,
        request: &Request<collection::FindObjectRequest>,
    ) -> collection::FindObjectResponse {
        self.storage_engine
            .clone()
            .catalog()
            .get_database_entry(&request.get_ref().database)
            .ok_or(collection::find_object_response::ErrorCode::InvalidDatabase)
            .and_then(|db: Arc<dyn DatabaseCatalogEntry>| {
                db.get_collection_entry(&request.get_ref().collection)
                    .ok_or(collection::find_object_response::ErrorCode::InvalidCollection)
            })
            .and_then(|collection: Arc<dyn CollectionCatalogEntry>| {
                // FIXME: handle object error
                Ok(collection.find_object(request.get_ref().id).unwrap())
            })
            .and_then(|object: Option<Vec<u8>>| {
                Ok(match object {
                    Some(object) => collection::FindObjectResponse {
                        error_code: collection::find_object_response::ErrorCode::NoError as i32,
                        object,
                    },
                    None => collection::FindObjectResponse {
                        error_code: collection::find_object_response::ErrorCode::InvalidId as i32,
                        object: vec![],
                    },
                })
            })
            .unwrap_or_else(|error_code| collection::FindObjectResponse {
                error_code: error_code as i32,
                object: vec![],
            })
    }
}

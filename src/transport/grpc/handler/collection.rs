use std::{error::Error, sync::Arc};

use super::super::generated::protodb::collection;
use super::Handler;

use crate::{
    catalog::{collection::CollectionCatalogEntry, database::DatabaseCatalogEntry},
    schema::errors::SchemaError,
    storage::errors,
};

use tower_grpc::Request;

macro_rules! create_collection_err {
    ( $failure_code:ident, $schema_error:expr ) => {
        (
            collection::create_collection_response::FailureCode::$failure_code,
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
                ).map_err(|err| match err {
                    errors::collection::CreateCollectionError::CollectionExists => {
                        create_collection_err!(CollectionExists, None)
                    }
                    errors::collection::CreateCollectionError::SchemaError(err) => match err {
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
            }).and(Ok(collection::CreateCollectionResponse {
                success: true,
                failure_code: collection::create_collection_response::FailureCode::NoFailure as i32,
                schema_error: None,
            })).unwrap_or_else(
                |(failure_code, schema_error)| collection::CreateCollectionResponse {
                    success: false,
                    failure_code: failure_code as i32,
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
            .ok_or(collection::list_collections_response::FailureCode::InvalidDatabase)
            .and_then(|db: Arc<dyn DatabaseCatalogEntry>| {
                Ok(collection::ListCollectionsResponse {
                    success: true,
                    failure_code: collection::list_collections_response::FailureCode::NoError
                        as i32,
                    collections: db.list_collections(),
                })
            }).unwrap_or_else(|failure_code| collection::ListCollectionsResponse {
                success: false,
                failure_code: failure_code as i32,
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
            .ok_or((collection::insert_object_response::FailureCode::InvalidDatabase, None))
            .and_then(|db: Arc<dyn DatabaseCatalogEntry>| {
                db
                    .get_collection_entry(&request.get_ref().collection)
                    .ok_or((collection::insert_object_response::FailureCode::InvalidCollection, None))
            })
            .and_then(|collection: Arc<dyn CollectionCatalogEntry>| {
                collection.insert_object(&request.get_ref().object)
                    .map_err(|err| match err {
                        errors::collection::InsertObjectError::ObjectExists => {
                            (collection::insert_object_response::FailureCode::ObjectExists, None)
                        }
                        errors::collection::InsertObjectError::ObjectError(err) => {
                            let object_error = collection::insert_object_response::ObjectError {
                                // FIXME: add match
                                code: collection::insert_object_response::object_error::ObjectErrorCode::DecodeError as i32,
                                message: err.description().into(),
                            };
                            (collection::insert_object_response::FailureCode::InvalidDatabase, Some(object_error))
                        }
                    })
            })
            .and(
                Ok(collection::InsertObjectResponse {
                    success: true,
                    failure_code: collection::insert_object_response::FailureCode::NoFailure
                        as i32,
                    object_error: None,
                })
            )
            .unwrap_or_else(|(failure_code, object_error)| {
                collection::InsertObjectResponse {
                    success: false,
                    failure_code: failure_code as i32,
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
            .ok_or(collection::find_object_response::FailureCode::InvalidDatabase)
            .and_then(|db: Arc<dyn DatabaseCatalogEntry>| {
                db.get_collection_entry(&request.get_ref().collection)
                    .ok_or(collection::find_object_response::FailureCode::InvalidCollection)
            }).and_then(|collection: Arc<dyn CollectionCatalogEntry>| {
                // FIXME: handle object error
                Ok(collection.find_object(request.get_ref().id).unwrap())
            }).and_then(|object: Option<Vec<u8>>| {
                Ok(match object {
                    Some(object) => collection::FindObjectResponse {
                        success: true,
                        failure_code: collection::find_object_response::FailureCode::NoFailure
                            as i32,
                        object,
                    },
                    None => collection::FindObjectResponse {
                        success: false,
                        failure_code: collection::find_object_response::FailureCode::InvalidId
                            as i32,
                        object: vec![],
                    },
                })
            }).unwrap_or_else(|failure_code| collection::FindObjectResponse {
                success: false,
                failure_code: failure_code as i32,
                object: vec![],
            })
    }
}

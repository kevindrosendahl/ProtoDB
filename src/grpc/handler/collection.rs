use std::error::Error;

use super::super::generated::protodb::collection;
use super::Handler;

use crate::storage::{errors, schema::errors::SchemaError};

use tower_grpc::Request;

impl Handler {
    pub(crate) fn handle_create_collection(
        &mut self,
        request: &Request<collection::CreateCollectionRequest>,
    ) -> collection::CreateCollectionResponse {
        self.storage_engine
            .clone()
            .create_collection(
                &request.get_ref().database,
                &request.get_ref().name,
                &request.get_ref().schema.clone().unwrap(),
            ).and(Ok(collection::CreateCollectionResponse {
                success: true,
                failure_code: collection::create_collection_response::FailureCode::NoFailure as i32,
                schema_error: None,
            })).unwrap_or_else(|err| {
                let (failure_code, schema_error) = match err {
                    errors::collection::CreateCollectionError::InvalidDatabase => (
                        collection::create_collection_response::FailureCode::InvalidDatabase,
                        None,
                    ),
                    errors::collection::CreateCollectionError::CollectionExists => (
                        collection::create_collection_response::FailureCode::CollectionExists,
                        None,
                    ),
                    errors::collection::CreateCollectionError::SchemaError(err) => {
                        let (code, message) = match err {
                            SchemaError::InvalidFieldType((field, label, type_)) => {
                                (
                                    collection::create_collection_response::schema_error::SchemaErrorCode::InvalidFieldType,
                                    format!(
                                        "invalid field type (field {}, type {:?} {:?})",
                                        field, label, type_
                                    )
                                )
                            }
                            SchemaError::InvalidIdType(type_) => {
                                (
                                    collection::create_collection_response::schema_error::SchemaErrorCode::InvalidIdType,
                                    format!("invalid id type ({})", type_)
                                )
                            }
                            SchemaError::MissingIdField => {
                                (
                                    collection::create_collection_response::schema_error::SchemaErrorCode::MissingIdField,
                                    err.description().into()
                                )
                            }
                        };
                        (
                            collection::create_collection_response::FailureCode::SchemaError,
                            Some(collection::create_collection_response::SchemaError{
                                code: code as i32,
                                message,
                            })
                        )
                    }
                };
                collection::CreateCollectionResponse {
                    success: false,
                    failure_code: failure_code as i32,
                    schema_error,
                }
            })
    }

    pub(crate) fn handle_list_collections(
        &mut self,
        request: &Request<collection::ListCollectionsRequest>,
    ) -> collection::ListCollectionsResponse {
        self.storage_engine
            .clone()
            .list_collections(&request.get_ref().database)
            .and_then(|collections| {
                Ok(collection::ListCollectionsResponse {
                    success: true,
                    failure_code: collection::list_collections_response::FailureCode::NoError
                        as i32,
                    collections,
                })
            }).unwrap_or_else(|err| {
                let failure_code = match err {
                    errors::collection::ListCollectionsError::InvalidDatabase => {
                        collection::list_collections_response::FailureCode::InvalidDatabase
                    }
                };
                collection::ListCollectionsResponse {
                    success: false,
                    failure_code: failure_code as i32,
                    collections: Vec::new(),
                }
            })
    }

    pub(crate) fn handle_insert_object(
        &mut self,
        request: &Request<collection::InsertObjectRequest>,
    ) -> collection::InsertObjectResponse {
        self
            .storage_engine
            .clone()
            .insert_object(
                &request.get_ref().database,
                &request.get_ref().collection,
                &request.get_ref().object,
            )
            .and_then(|_| {
                Ok(collection::InsertObjectResponse {
                    success: true,
                    failure_code: collection::insert_object_response::FailureCode::NoFailure
                        as i32,
                    object_error: None,
                })
            }).unwrap_or_else(|err| {
            let (failure_code, object_error) = match err {
                errors::collection::InsertObjectError::InvalidDatabase => {
                    (collection::insert_object_response::FailureCode::InvalidDatabase, None)
                }
                errors::collection::InsertObjectError::InvalidCollection => {
                    (collection::insert_object_response::FailureCode::InvalidCollection, None)
                }
                errors::collection::InsertObjectError::ObjectError(err)=> {
                    let object_error = collection::insert_object_response::ObjectError{
                        // FIXME: add match
                        code: collection::insert_object_response::object_error::ObjectErrorCode::DecodeError as i32,
                        message: err.description().into(),
                    };
                    (collection::insert_object_response::FailureCode::InvalidDatabase, Some(object_error))
                }
            };
            collection::InsertObjectResponse {
                success: false,
                failure_code: failure_code as i32,
                object_error,
            }
        })
    }
}

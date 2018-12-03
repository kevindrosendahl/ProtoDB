use std::{error::Error, sync::Arc};

use super::super::generated::protodb::collection;
use super::Handler;

use crate::{
    catalog::{database::DatabaseCatalogEntry, errors},
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
}

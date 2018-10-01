use std::sync::Arc;

use crate::storage::{errors, StorageEngine};

use super::generated::protodb;
use super::generated::protodb::collection;
use super::generated::protodb::database;

use futures::future;
use tower_grpc;
use tower_grpc::{Request, Response};

#[derive(Clone)]
pub struct Handler {
    storage_engine: Arc<Box<dyn StorageEngine>>,
}

impl Handler {
    pub fn new(storage_engine: Box<dyn StorageEngine>) -> Handler {
        Handler {
            storage_engine: Arc::new(storage_engine),
        }
    }
}

impl protodb::server::ProtoDb for Handler {
    type CreateDatabaseFuture =
        future::FutureResult<Response<database::CreateDatabaseResponse>, tower_grpc::Error>;

    fn create_database(
        &mut self,
        request: Request<database::CreateDatabaseRequest>,
    ) -> Self::CreateDatabaseFuture {
        let response = self
            .storage_engine
            .clone()
            .create_database(&request.get_ref().name)
            .and(Ok(database::CreateDatabaseResponse{
                success: true,
                failure_code: database::create_database_response::FailureCode::NoError as i32}))
            .unwrap_or_else(|err| {
                let failure_code = match err {
                    errors::StorageError::DatabaseError(err) => {
                        match err {
                            errors::DatabaseError::DatabaseAlreadyExists => {
                                database::create_database_response::FailureCode::DatabaseExists
                            }
                            // FIXME: replace this by specifying possible errors for each action
                            _ => panic!(format!("unexpected create database error: {}", err)),
                        }
                    }
                    // FIXME: replace this by specifying possible errors for each action
                    _ => panic!(format!("unexpected create database error: {}", err)),
                };
                database::CreateDatabaseResponse{
                    success: false,
                    failure_code: failure_code as i32,
                }
            });

        future::ok(Response::new(response))
    }

    type ListDatabasesFuture =
        future::FutureResult<Response<database::ListDatabasesResponse>, tower_grpc::Error>;

    fn list_databases(
        &mut self,
        _request: Request<database::ListDatabasesRequest>,
    ) -> Self::ListDatabasesFuture {
        let response = Response::new(database::ListDatabasesResponse {
            databases: self.storage_engine.clone().list_databases(),
        });

        future::ok(response)
    }

    type CreateCollectionFuture =
        future::FutureResult<Response<collection::CreateCollectionResponse>, tower_grpc::Error>;

    fn create_collection(
        &mut self,
        request: Request<collection::CreateCollectionRequest>,
    ) -> Self::CreateCollectionFuture {
        let response = self
            .storage_engine
            .clone()
            .create_collection(
                &request.get_ref().database,
                &request.get_ref().name,
                &request.get_ref().schema.clone().unwrap(),
            ).and(Ok(collection::CreateCollectionResponse {
                success: true,
                failure_code: collection::create_collection_response::FailureCode::NoError as i32,
            })).unwrap_or_else(|err| {
                let failure_code = match err {
                    errors::StorageError::DatabaseError(err) => {
                        match err {
                            errors::DatabaseError::InvalidDatabase => {
                                collection::create_collection_response::FailureCode::InvalidDatabase
                            }
                            // FIXME: replace this by specifying possible errors for each action
                            _ => panic!(format!("unexpected create collection error: {}", err)),
                        }
                    }
                    errors::StorageError::CollectionError(_) => {
                        collection::create_collection_response::FailureCode::CollectionExists
                    }
                };
                collection::CreateCollectionResponse {
                    success: false,
                    failure_code: failure_code as i32,
                }
            });

        future::ok(Response::new(response))
    }

    type ListCollectionsFuture =
        future::FutureResult<Response<collection::ListCollectionsResponse>, tower_grpc::Error>;

    fn list_collections(
        &mut self,
        request: Request<collection::ListCollectionsRequest>,
    ) -> Self::ListCollectionsFuture {
        let response = self
            .storage_engine
            .clone()
            .list_collections(&request.get_ref().database)
            .and_then(|collections| {
                Ok(collection::ListCollectionsResponse {
                    success: true,
                    failure_code: collection::create_collection_response::FailureCode::NoError
                        as i32,

                    collections,
                })
            }).unwrap_or_else(|err| {
                let failure_code = match err {
                    errors::StorageError::DatabaseError(err) => {
                        match err {
                            errors::DatabaseError::InvalidDatabase => {
                                collection::create_collection_response::FailureCode::InvalidDatabase
                            }
                            // FIXME: replace this by specifying possible errors for each action
                            _ => panic!(format!("unexpected create collection error: {}", err)),
                        }
                    }
                    errors::StorageError::CollectionError(_) => {
                        collection::create_collection_response::FailureCode::CollectionExists
                    }
                };
                collection::ListCollectionsResponse {
                    success: false,
                    failure_code: failure_code as i32,

                    collections: Vec::new(),
                }
            });

        future::ok(Response::new(response))
    }
}

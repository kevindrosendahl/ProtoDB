mod collection;
mod database;

use std::sync::Arc;

use super::generated::{
    protodb,
    protodb::{collection as protodb_collection, database as protodb_database},
};

use crate::storage::StorageEngine;

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

macro_rules! protodb_handler {
    ( $( ($method:ident, $handler:ident, $future:ident, $request:path, $response:path) ),* ) => {
            impl protodb::server::ProtoDb for Handler {
                $(
                    type $future = future::FutureResult<Response<$response>, tower_grpc::Error>;

                    fn $method(&mut self, request: Request<$request>) -> Self::$future {
                        future::ok(Response::new(self.$handler(&request)))
                    }
                )*
        }
    };
}

protodb_handler![
    (
        create_database,
        handle_create_database,
        CreateDatabaseFuture,
        protodb_database::CreateDatabaseRequest,
        protodb_database::CreateDatabaseResponse
    ),
    (
        list_databases,
        handle_list_databases,
        ListDatabasesFuture,
        protodb_database::ListDatabasesRequest,
        protodb_database::ListDatabasesResponse
    ),
    (
        create_collection,
        handle_create_collection,
        CreateCollectionFuture,
        protodb_collection::CreateCollectionRequest,
        protodb_collection::CreateCollectionResponse
    ),
    (
        list_collections,
        handle_list_collections,
        ListCollectionsFuture,
        protodb_collection::ListCollectionsRequest,
        protodb_collection::ListCollectionsResponse
    ),
    (
        insert_object,
        handle_insert_object,
        InsertObjectFuture,
        protodb_collection::InsertObjectRequest,
        protodb_collection::InsertObjectResponse
    )
];

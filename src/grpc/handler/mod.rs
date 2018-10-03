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

// TODO: can probably refactor this into a macro
impl protodb::server::ProtoDb for Handler {
    type CreateDatabaseFuture =
        future::FutureResult<Response<protodb_database::CreateDatabaseResponse>, tower_grpc::Error>;

    fn create_database(
        &mut self,
        request: Request<protodb_database::CreateDatabaseRequest>,
    ) -> Self::CreateDatabaseFuture {
        future::ok(Response::new(self.handle_create_database(&request)))
    }

    type ListDatabasesFuture =
        future::FutureResult<Response<protodb_database::ListDatabasesResponse>, tower_grpc::Error>;

    fn list_databases(
        &mut self,
        _request: Request<protodb_database::ListDatabasesRequest>,
    ) -> Self::ListDatabasesFuture {
        future::ok(Response::new(self.handle_list_databases()))
    }

    type CreateCollectionFuture = future::FutureResult<
        Response<protodb_collection::CreateCollectionResponse>,
        tower_grpc::Error,
    >;

    fn create_collection(
        &mut self,
        request: Request<protodb_collection::CreateCollectionRequest>,
    ) -> Self::CreateCollectionFuture {
        future::ok(Response::new(self.handle_create_collection(&request)))
    }

    type ListCollectionsFuture = future::FutureResult<
        Response<protodb_collection::ListCollectionsResponse>,
        tower_grpc::Error,
    >;

    fn list_collections(
        &mut self,
        request: Request<protodb_collection::ListCollectionsRequest>,
    ) -> Self::ListCollectionsFuture {
        future::ok(Response::new(self.handle_list_collections(&request)))
    }

    type InsertObjectFuture =
        future::FutureResult<Response<protodb_collection::InsertObjectResponse>, tower_grpc::Error>;

    fn insert_object(
        &mut self,
        request: Request<protodb_collection::InsertObjectRequest>,
    ) -> Self::InsertObjectFuture {
        future::ok(Response::new(self.handle_insert_object(&request)))
    }
}

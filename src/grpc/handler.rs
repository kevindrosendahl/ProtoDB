use std::sync::Arc;

use crate::storage::storage_engine::StorageEngine;

use super::generated;
use super::generated::{request, response};

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

impl generated::server::ProtoDb for Handler {
    type CreateDatabaseFuture =
        future::FutureResult<Response<response::CreateDatabase>, tower_grpc::Error>;

    fn create_database(
        &mut self,
        request: Request<request::CreateDatabase>,
    ) -> Self::CreateDatabaseFuture {
        println!("got request to create {}", request.get_ref().name);

        self.storage_engine
            .clone()
            .create_database(&request.get_ref().name);

        let response = Response::new(response::CreateDatabase {
            success: true,
            failure_code: generated::create_database_response::FailureCode::NoError as i32,
        });

        future::ok(response)
    }

    type ListDatabasesFuture =
        future::FutureResult<Response<response::ListDatabases>, tower_grpc::Error>;

    fn list_databases(
        &mut self,
        _request: Request<request::ListDatabases>,
    ) -> Self::ListDatabasesFuture {
        println!("got request to list databases");

        let response = Response::new(response::ListDatabases {
            database: self.storage_engine.clone().list_databases(),
        });

        future::ok(response)
    }

    type CreateCollectionFuture =
    future::FutureResult<Response<response::CreateCollection>, tower_grpc::Error>;

    fn create_collection(
        &mut self,
        request: Request<request::CreateCollection>,
    ) -> Self::CreateCollectionFuture {
        println!("got request to create {}", request.get_ref().name);

        self.storage_engine
            .clone()
            .create_collection(&request.get_ref().database, &request.get_ref().name, &request.get_ref().schema.clone().unwrap());

        let response = Response::new(response::CreateCollection {
            success: true,
            failure_code: generated::create_collection_response::FailureCode::NoError as i32,
        });

        future::ok(response)
    }
}

use std::sync::Arc;

use ::storage::storage_engine::StorageEngine;

use super::generated;
use super::generated::{CreateDatabaseRequest, CreateDatabaseResponse};

use futures::{future};
use tower_grpc;
use tower_grpc::{Request, Response};

#[derive(Clone)]
pub struct Handler {
    storage_engine: Arc<Box<StorageEngine>>
}

impl Handler {
    pub fn new(storage_engine: Box<StorageEngine>) -> Handler {
        Handler { storage_engine: Arc::new(storage_engine) }
    }
}

impl generated::server::ProtoDb for Handler {
    type CreateDatabaseFuture = future::FutureResult<Response<CreateDatabaseResponse>, tower_grpc::Error>;

    /// returns the feature at the given point.
    fn create_database(&mut self, request: Request<CreateDatabaseRequest>) -> Self::CreateDatabaseFuture {
        println!("got request to create {}", request.get_ref().name);
        // Otherwise, return some other feature?
        let response = Response::new(CreateDatabaseResponse {
            success: true,
            failure_code: generated::create_database_response::FailureCode::NoError as i32,
        });

        future::ok(response)
    }
}
mod collection;
mod database;
mod object;
mod wasm;

use std::{sync::Arc, time::Instant};

use super::generated::{
    protodb,
    protodb::{
        collection as protodb_collection, database as protodb_database, object as protodb_object,
        wasm as protodb_wasm,
    },
};

use crate::{storage::StorageEngine, wasm::Interpreter};

use futures::future;
use tower_grpc;
use tower_grpc::{Request, Response};

#[derive(Clone)]
pub struct Handler {
    storage_engine: Arc<dyn StorageEngine>,
    wasm_interpreter: Arc<Interpreter>,
}

impl Handler {
    pub fn new(storage_engine: Arc<dyn StorageEngine>) -> Handler {
        Handler {
            storage_engine: storage_engine.clone(),
            wasm_interpreter: Arc::new(Interpreter::new(storage_engine)),
        }
    }
}

macro_rules! method_handler {
    (  $desc:expr, $method:ident, $handler:ident, $future:ident, $request:path, $response:path ) => {
        type $future = future::FutureResult<Response<$response>, tower_grpc::Error>;

        fn $method(&mut self, request: Request<$request>) -> Self::$future {
            let start = Instant::now();

            debug!("received {} request", $desc);
            let response = self.$handler(&request);
            debug!("{} request took {:?}", $desc, start.elapsed());

            future::ok(Response::new(response))
        }
    }
}

impl protodb::server::ProtoDb for Handler {
    method_handler!(
        "create database",
        create_database,
        handle_create_database,
        CreateDatabaseFuture,
        protodb_database::CreateDatabaseRequest,
        protodb_database::CreateDatabaseResponse
    );

    method_handler!(
        "list databases",
        list_databases,
        handle_list_databases,
        ListDatabasesFuture,
        protodb_database::ListDatabasesRequest,
        protodb_database::ListDatabasesResponse
    );

    method_handler!(
        "create collection",
        create_collection,
        handle_create_collection,
        CreateCollectionFuture,
        protodb_collection::CreateCollectionRequest,
        protodb_collection::CreateCollectionResponse
    );

    method_handler!(
        "list collections",
        list_collections,
        handle_list_collections,
        ListCollectionsFuture,
        protodb_collection::ListCollectionsRequest,
        protodb_collection::ListCollectionsResponse
    );

    method_handler!(
        "insert object",
        insert_object,
        handle_insert_object,
        InsertObjectFuture,
        protodb_object::InsertObjectRequest,
        protodb_object::InsertObjectResponse
    );

    method_handler!(
        "find object",
        find_object,
        handle_find_object,
        FindObjectFuture,
        protodb_object::FindObjectRequest,
        protodb_object::FindObjectResponse
    );

    method_handler!(
        "register wasm module",
        register_wasm_module,
        handle_register_wasm_module,
        RegisterWasmModuleFuture,
        protodb_wasm::RegisterModuleRequest,
        protodb_wasm::RegisterModuleResponse
    );

    method_handler!(
        "run wasm module",
        run_wasm_module,
        handle_run_wasm_module,
        RunWasmModuleFuture,
        protodb_wasm::RunModuleRequest,
        protodb_wasm::RunModuleResponse
    );
}

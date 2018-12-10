use std::sync::Arc;

use protodb::{
    storage::engine::rocksdb::RocksDBStorageEngine,
    transport::grpc::{handler::Handler, server::Server},
};

pub fn main() {
    pretty_env_logger::init();

    let storage_engine = RocksDBStorageEngine::try_new("/tmp/protodb/data");
    let handler = Handler::new(Arc::new(storage_engine.unwrap_or_else(|err| {
        panic!(format!("error initializing RocksDB database: {}", err));
    })));
    let server = Server::new("[::1]:10000".parse().unwrap(), handler);
    server.run();
}

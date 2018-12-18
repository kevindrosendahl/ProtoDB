use std::sync::Arc;

use protodb::{
    storage::engine::rocksdb::RocksDBStorageEngine,
    transport::grpc::{handler::Handler, server::Server},
};
use clap::{Arg, App};

pub fn main() {
    pretty_env_logger::init();
    let matches = App::new("protodb")
        .version("0.1.0")
        .author("Kevin Rosendahl <kevindrosendahl@gmail.com>")
        .arg(Arg::with_name("data-dir")
            .long("data-dir")
            .value_name("DATA-DIR")
            .help("root directory to store data in")
            .takes_value(true)
            .default_value("/var/lib/protodb/data"))
        .get_matches();

    let data_dir = matches.value_of("data-dir").unwrap();

    let storage_engine = RocksDBStorageEngine::try_new(data_dir);
    let handler = Handler::new(Arc::new(storage_engine.unwrap_or_else(|err| {
        panic!(format!("error initializing RocksDB database: {}", err));
    })));
    let server = Server::new("[::1]:10000".parse().unwrap(), handler);
    server.run();
}

use libprotodb::{
    storage::engine::in_memory::InMemoryStorageEngine,
    transport::grpc::{handler::Handler, server::Server},
};

pub fn main() {
    pretty_env_logger::init();

    let storage_engine: InMemoryStorageEngine = Default::default();
    let handler = Handler::new(Box::new(storage_engine));
    let server = Server::new("127.0.0.1:10000".parse().unwrap(), handler);
    server.run();
}

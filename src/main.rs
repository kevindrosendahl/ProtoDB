use libprotodb::{
    grpc::{handler::Handler, server::Server},
    storage::in_memory::InMemoryStorageEngine,
};

pub fn main() {
    let storage_engine = InMemoryStorageEngine::new();
    let handler = Handler::new(Box::new(storage_engine));
    let server = Server::new("127.0.0.1:10000".parse().unwrap(), handler);
    server.run();
}

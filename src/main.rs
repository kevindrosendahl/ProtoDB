extern crate libprotodb;

pub fn main() {
    let storage_engine = libprotodb::storage::in_memory::InMemoryStorageEngine::new();
    let handler = libprotodb::grpc::handler::Handler::new(Box::new(storage_engine));
    let server = libprotodb::grpc::server::Server::new("127.0.0.1:10000".parse().unwrap(), handler);
    server.run();
}
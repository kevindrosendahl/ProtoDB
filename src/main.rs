extern crate libprotodb;
use libprotodb::server::Server;
use libprotodb::server::protos::server_grpc::ProtoDBServer;

use std::path::Path;
use std::thread;

fn main() {
    println!("starting");
    let _server = ProtoDBServer::new(8888, Server::new(Path::new("/tmp/protodb.lmdb")));
    println!("started");
    loop {
        thread::park();
    }
}

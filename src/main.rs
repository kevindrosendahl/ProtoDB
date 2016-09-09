extern crate libprotodb;
use libprotodb::server::grpc::sync::SyncServer;
use libprotodb::server::protos::server_grpc::ProtoDBServer;

use std::thread;

fn main() {
    println!("starting");
    let _server = ProtoDBServer::new(8888, SyncServer);
    println!("started");
    loop {
        thread::park();
    }
}

//extern crate bytes;
//extern crate http;
//extern crate futures;
//extern crate prost;
//extern crate tokio_core;
//extern crate tower_h2;
//extern crate tower_http;
//extern crate tower_grpc;

use crate::transport::grpc::generated::protodb;

use futures::Future;
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use tower_grpc::Request;
use tower_h2::client::Connection;


pub fn install() {
    let mut core = Core::new().unwrap();
    let reactor = core.handle();

    let addr = "[::1]:10000".parse().unwrap();
    let uri: http::Uri = format!("http://localhost:10000").parse().unwrap();

    let list_databases = TcpStream::connect(&addr, &reactor)
        .and_then(move |socket| {
            // Bind the HTTP/2.0 connection
            Connection::handshake(socket, reactor)
                .map_err(|_| panic!("failed HTTP/2.0 handshake"))
        })
        .map(move |conn| {
            use tower_http::add_origin;

            let conn = add_origin::Builder::new()
                .uri(uri)
                .build(conn)
                .unwrap();

            protodb::client::ProtoDb::new(conn)
        })
        .and_then(|mut client| {
            client.list_databases(Request::new(protodb::database::ListDatabasesRequest {
            })).map_err(|e| panic!("gRPC request failed; err={:?}", e))
        })
        .and_then(|response| {
            println!("RESPONSE = {:?}", response);
            Ok(())
        })
        .map_err(|e| {
            println!("ERR = {:?}", e);
        });

    core.run(list_databases).unwrap();
}

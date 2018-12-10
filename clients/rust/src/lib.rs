mod collection;
mod database;
mod errors;
pub mod generated;
mod object;
mod wasm;

#[macro_use]
extern crate prost_derive;

pub use self::errors::ClientError;
use self::generated::protodb;

use futures::Future;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tower_h2::client::Connection;

pub struct Client {
    core: tokio_core::reactor::Core,
    // proper type courtesy of https://github.com/tower-rs/tower-grpc/blob/40b059abac9ca07edac252f4d0b69c55f6ecf88d/tower-grpc-interop/src/client.rs#L225-L230
    client: protodb::client::ProtoDb<
        tower_http::AddOrigin<
            tower_h2::client::Connection<
                tokio_core::net::TcpStream,
                tokio_core::reactor::Handle,
                tower_grpc::BoxBody,
            >,
        >,
    >,
}

impl Client {
    pub fn new() -> Self {
        let core = Core::new().unwrap();
        Client::with_core(core)
    }

    pub fn with_core(mut core: Core) -> Self {
        let reactor = core.handle();

        let addr = "[::1]:10000".parse().unwrap();
        let uri: http::Uri = "http://localhost:10000".to_string().parse().unwrap();

        let get_client = TcpStream::connect(&addr, &reactor)
            .and_then(move |socket| {
                // Bind the HTTP/2.0 connection
                Connection::handshake(socket, reactor)
                    .map_err(|_| panic!("failed HTTP/2.0 handshake"))
            })
            .map(move |conn| {
                use tower_http::add_origin;

                let conn = add_origin::Builder::new().uri(uri).build(conn).unwrap();

                protodb::client::ProtoDb::new(conn)
            })
            .map_err(|e| {
                panic!("ERR = {:?}", e);
            });;

        let client = core.run(get_client).unwrap();

        Client { core, client }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

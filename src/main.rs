extern crate bytes;
extern crate futures;
extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate tokio_core;
extern crate tower_h2;
extern crate tower_grpc;

extern crate env_logger;
#[macro_use]
extern crate log;


pub mod protod {
    include!(concat!(env!("OUT_DIR"), "/protodb.rs"));
}
use protod::{server, CreateDatabaseRequest, CreateDatabaseResponse};

use futures::{future, Future, Stream};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tower_h2::Server;
use tower_grpc::{Request, Response};

#[derive(Debug, Clone)]
struct ProtoDB {
}

impl protod::server::ProtoDb for ProtoDB {
    type CreateDatabaseFuture = future::FutureResult<Response<CreateDatabaseResponse>, tower_grpc::Error>;

    /// returns the feature at the given point.
    fn create_database(&mut self, request: Request<CreateDatabaseRequest>) -> Self::CreateDatabaseFuture {
        println!("got request to create {}", request.get_ref().name);
        // Otherwise, return some other feature?
        let response = Response::new(CreateDatabaseResponse {
           success: true,
            failure_code: protod::create_database_response::FailureCode::NoError as i32,
        });

        future::ok(response)
    }
}

pub fn main() {
    let _ = ::env_logger::init();

    let mut core = Core::new().unwrap();
    let reactor = core.handle();

    let handler = ProtoDB{};

    let new_service = server::ProtoDbServer::new(handler);

    let h2 = Server::new(new_service, Default::default(), reactor.clone());

    let addr = "127.0.0.1:10000".parse().unwrap();
    let bind = TcpListener::bind(&addr, &reactor).expect("bind");

    println!("listining on {:?}", addr);

    let serve = bind.incoming()
        .fold((h2, reactor), |(h2, reactor), (sock, _)| {
            if let Err(e) = sock.set_nodelay(true) {
                return Err(e);
            }

            let serve = h2.serve(sock);
            reactor.spawn(serve.map_err(|e| error!("h2 error: {:?}", e)));

            Ok((h2, reactor))
        });

    core.run(serve).unwrap();
}
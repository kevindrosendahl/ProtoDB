use std::net::SocketAddr;

use super::generated::server;
use super::handler::Handler;

use futures::{Future, Stream};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tower_h2;

pub struct Server {
    addr: SocketAddr,
    handler: Handler,
}

impl Server {
    pub fn new(addr: SocketAddr, handler: Handler) -> Server {
        Server { addr, handler }
    }

    pub fn run(self) {
        let mut core = Core::new().unwrap();
        let reactor = core.handle();

        let new_service = server::ProtoDbServer::new(self.handler);
        let h2 = tower_h2::Server::new(new_service, Default::default(), reactor.clone());
        let bind = TcpListener::bind(&self.addr, &reactor).expect("bind");

        println!("listening on {:?}", self.addr);

        let serve = bind
            .incoming()
            .fold((h2, reactor), |(h2, reactor), (sock, _)| {
                if let Err(e) = sock.set_nodelay(true) {
                    return Err(e);
                }

                let serve = h2.serve(sock);
                reactor.spawn(serve.map_err(|e| println!("h2 error: {:?}", e)));

                Ok((h2, reactor))
            });

        core.run(serve).unwrap();
    }
}

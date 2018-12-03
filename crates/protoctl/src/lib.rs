#[macro_use]
extern crate prost_derive;
#[macro_use]
extern crate structopt;

pub mod command;
pub mod transport;
pub mod util;

use std::cell::RefCell;

thread_local! {
    /// The global client.
    pub static CLIENT: RefCell<transport::grpc::Client> = RefCell::new(transport::grpc::Client::new());
}

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(subcommand)]
    pub cmd: command::Command,
}

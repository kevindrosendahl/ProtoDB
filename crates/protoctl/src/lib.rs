pub mod command;
pub mod util;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate structopt;

use protodb_client::Client;

use std::cell::RefCell;

thread_local! {
    /// The global client.
    pub static CLIENT: RefCell<Client> = RefCell::new(Client::new());
}

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(subcommand)]
    pub cmd: command::Command,
}

#[macro_use]
extern crate prost_derive;
#[macro_use]
extern crate structopt;

pub mod command;
pub mod transport;
pub mod util;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(subcommand)]
    pub cmd: command::Command,
}

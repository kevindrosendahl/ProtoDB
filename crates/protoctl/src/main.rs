use protoctl::{command::run_protoctl, Cli};

use human_panic::setup_panic;
use structopt::StructOpt;

fn main() {
    //    setup_panic!();
    let args = Cli::from_args();
    run_protoctl(args.cmd);
}

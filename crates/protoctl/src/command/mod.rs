pub mod install;

use self::install::install;

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "install")]
    Install,
}

pub fn run_protoctl(command: Command) {
    match command {
        Command::Install => install(),
    }
}

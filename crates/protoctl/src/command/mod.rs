mod collection;
mod database;

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "collection")]
    Collection(collection::Collection),

    #[structopt(name = "database")]
    Database(database::Database),
}

pub fn run_protoctl(command: Command) {
    match command {
        Command::Collection(collection) => collection::run_collection(collection),
        Command::Database(database) => database::run_database(database),
    }
}

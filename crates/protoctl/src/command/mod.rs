mod collection;
mod database;
mod index;
mod object;
mod wasm;

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "collection")]
    Collection(collection::Collection),

    #[structopt(name = "database")]
    Database(database::Database),

    #[structopt(name = "index")]
    Index(index::Index),

    #[structopt(name = "object")]
    Object(object::Object),

    #[structopt(name = "wasm")]
    Wasm(wasm::Wasm),
}

pub fn run_protoctl(command: Command) {
    match command {
        Command::Collection(collection) => collection::run_collection(collection),
        Command::Database(database) => database::run_database(database),
        Command::Index(index) => index::run_index(index),
        Command::Object(object) => object::run_object(object),
        Command::Wasm(wasm) => wasm::run_wasm(wasm),
    }
}

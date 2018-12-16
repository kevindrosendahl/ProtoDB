use crate::CLIENT;

#[derive(Debug, StructOpt)]
pub enum Index {
    #[structopt(name = "create")]
    Create {
        #[structopt(long = "database", short = "d")]
        database: String,

        #[structopt(long = "collection", short = "c")]
        collection: String,

        #[structopt(long = "field", short = "f")]
        field: i32,
    },

    #[structopt(name = "get")]
    Get {
        #[structopt(long = "database", short = "d")]
        database: String,

        #[structopt(long = "collection", short = "c")]
        collection: String,

        #[structopt(long = "id", short = "i")]
        id: u64,
    },

    #[structopt(name = "list")]
    List {
        #[structopt(long = "database", short = "d")]
        database: String,

        #[structopt(long = "collection", short = "c")]
        collection: String,
    },
}

pub fn run_index(index: Index) {
    match index {
        Index::Create {
            database,
            collection,
            field,
        } => create_index(database, collection, field),
        Index::Get {
            database,
            collection,
            id,
        } => get_index(database, collection, id),
        Index::List {
            database,
            collection,
        } => list_indexes(database, collection),
    }
}

fn create_index(database: String, collection: String, field: i32) {
    CLIENT
        .with(|c| c.borrow_mut().create_index(database, collection, field))
        .and_then(|response| {
            use protodb_client::generated::protodb::index::create_index_response::ErrorCode;
            match response.error_code() {
                ErrorCode::NoError => {
                    println!("successfully created index on field {}, id: {}", field, response.id)
                },
                ErrorCode::InternalError => println!("internal error"),
                ErrorCode::InvalidDatabase => println!("invalid database"),
                ErrorCode::InvalidCollection => println!("invalid collection"),
                ErrorCode::InvalidField => println!("invalid field"),
                ErrorCode::UnsupportedFieldType => println!("unsupported field type"),
            }
            Ok(())
        })
        .map_err(|err| println!("error creating index: {:?}", err))
        .unwrap();
}

fn get_index(database: String, collection: String, id: u64) {
    CLIENT
        .with(|c| c.borrow_mut().get_index(database, collection, id))
        .and_then(|response| {
            use protodb_client::generated::protodb::index::get_index_response::ErrorCode;
            match response.error_code() {
                ErrorCode::NoError => {
                    let index = response.index.unwrap();
                    println!("index {} on field {}", index.id, index.field);
                }
                ErrorCode::InternalError => {
                    println!("internal error");
                }
                ErrorCode::InvalidDatabase => {
                    println!("invalid database");
                }
                ErrorCode::InvalidCollection => {
                    println!("invalid collection");
                }
                ErrorCode::InvalidId => {
                    println!("invalid id");
                }
            }

            Ok(())
        })
        .map_err(|err| println!("error listing indexes: {:?}", err))
        .unwrap();
}

fn list_indexes(database: String, collection: String) {
    CLIENT
        .with(|c| c.borrow_mut().list_indexes(database, collection))
        .and_then(|response| {
            use protodb_client::generated::protodb::index::list_indexes_response::ErrorCode;
            match response.error_code() {
                ErrorCode::NoError => {
                    for index in response.indexes.iter() {
                        println!("index {} on field {}", index.id, index.field);
                    }
                }
                ErrorCode::InternalError => {
                    println!("internal error");
                }
                ErrorCode::InvalidDatabase => {
                    println!("invalid database");
                }
                ErrorCode::InvalidCollection => {
                    println!("invalid collection");
                }
            }

            Ok(())
        })
        .map_err(|err| println!("error listing indexes: {:?}", err))
        .unwrap();
}

use crate::CLIENT;

#[derive(Debug, StructOpt)]
pub enum Database {
    #[structopt(name = "create")]
    Create {
        #[structopt(name = "name")]
        name: String,
    },
    #[structopt(name = "list")]
    List,
}

pub fn run_database(database: Database) {
    match database {
        Database::Create { name } => create_database(name),
        Database::List => list_databases(),
    }
}

fn create_database(name: String) {
    CLIENT
        .with(|c| c.borrow_mut().create_database(name))
        .and_then(|response| {
            use crate::transport::grpc::generated::protodb::database::create_database_response::ErrorCode;
            match response.error_code() {
                ErrorCode::NoError => (),
                ErrorCode::InternalError => println!("error creating database: internal error"),
                ErrorCode::DatabaseExists => println!("database already exists"),
            }
            Ok(())
        })
        .map_err(|err| println!("error creating database: {:?}", err))
        .unwrap();
}

fn list_databases() {
    CLIENT
        .with(|c| c.borrow_mut().list_databases())
        .and_then(|response| {
            println!("databases: {:?}", response.databases);
            Ok(())
        })
        .map_err(|err| println!("error listing databases: {:?}", err))
        .unwrap();
}

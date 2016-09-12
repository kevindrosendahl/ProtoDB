pub mod collection;
pub mod database;
pub mod grpc;
mod errors;
pub mod protos;
pub mod query_planner;
mod result;
pub mod schema;
pub mod util;

pub use self::errors::ServerError;
pub use self::result::Result;

use self::collection::Find;
use self::database::Database;
use self::protos::database as database_proto;
use self::util::protobuf::descriptor_from_file_descriptor;

use ::protobuf::CodedInputStream;
use ::protobuf::descriptor;
use ::protobuf::Message;

extern crate lmdb_rs as lmdb;
use self::lmdb::{DbFlags as LmdbDbFlags, EnvBuilder as LmdbEnvBuilder,
                 Environment as LmdbEnvironment};

use std::collections::HashMap;
use std::path::Path;
use std::sync::RwLock;

pub const METADATA_DATABASE: &'static str = "__system";
pub const METADATA_DATABASE_DB_ID: u64 = 1;
pub const DATABASE_METADATA_COLLECTION: &'static str = "databases";

lazy_static! {
    static ref DATABASE_DESCRIPTOR: descriptor::DescriptorProto = {
        let file_descriptor_proto = database_proto::file_descriptor_proto();
        descriptor_from_file_descriptor(file_descriptor_proto, "Database")
            .expect("cannot find Database DescriptorProto")
    };
}

struct ProtoDBServer {
    databases: RwLock<HashMap<String, Database>>,
    lmdb_environment: LmdbEnvironment,
}

impl ProtoDBServer {
    pub fn new(db_path: &Path) -> ProtoDBServer {
        let databases: HashMap<String, Database> = HashMap::new();

        let lmdb_environment = LmdbEnvBuilder::new()
            .open(db_path, 0o777)
            .expect(format!("unable to open LMDB database at {:?}", db_path).as_str());

        let protodb_server = ProtoDBServer {
            databases: RwLock::new(databases),
            lmdb_environment: lmdb_environment,
        };

        // protodb_server.create_database(String::from(METADATA_DATABASE), METADATA_DATABASE_DB_ID)
        //     .expect(format!("Unable to create {} database", METADATA_DATABASE).as_str());

        protodb_server
    }

    fn process_metadata_database(&self) {
        // unimplemented!();
        let lmdb_db_handle = self.lmdb_environment
            .get_default_db(LmdbDbFlags::empty())
            .expect("unable to get default LMDB db");

        let txn =
            self.lmdb_environment.get_reader().expect("unable to get lmdb read only transaction");

        let lmdb_db = txn.bind(&lmdb_db_handle);
        {
            match self.databases
                .read()
                .expect("lock for databases poisoned")
                .get(METADATA_DATABASE) {
                Some(&db) => {
                    match db.get_collection(String::from(DATABASE_METADATA_COLLECTION)) {
                        Some(&collection) => {
                            match collection.find_all(&lmdb_db)
                                .expect("failed to read metadata collection") {
                                Some(databases) => {
                                    for database_data in databases {
                                        let mut database_data_buf =
                                            CodedInputStream::from_bytes(database_data.as_slice());

                                        let mut database_message = database_proto::Database::new();
                                        database_message.merge_from(&mut database_data_buf);
                                        self.create_database(String::from(database_message.get_name()),
                                                             database_message.get__id());
                                    }
                                }
                                None => {
                                    self.populate_metadata_database();
                                }
                            }
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }


    }

    fn populate_metadata_database(&self) {}

    pub fn create_database(&self, name: String, db_id: u64) -> Result<()> {
        let lmdb_db_handle = self.lmdb_environment
            .get_default_db(LmdbDbFlags::empty())
            .expect("unable to get default LMDB db");

        let txn = self.lmdb_environment.new_transaction().expect("unable to get lmdb transaction");

        let database: Database;
        {
            let lmdb_db = txn.bind(&lmdb_db_handle);

            database = try!(Database::new(name, db_id, &lmdb_db));
        }

        txn.commit().expect("unable to commit create_database");
        Ok(())
    }

    // pub fn get_database<>(&self, name: String) -> Option<&Database> {
    //     // let databases = self.databases.read().expect("lock for databases poisoned");
    //     // databases.get(&name)
    //     self.databases.read().expect("lock for databases poisoned").get(&name)
    // }
}

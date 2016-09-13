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

use self::collection::{Collection, Find, Insert};
use self::database::{Database, DatabaseError};
use self::protos::database as database_proto;
use self::util::protobuf::descriptor_from_file_descriptor;

use ::protobuf::CodedInputStream;
use ::protobuf::descriptor;
use ::protobuf::Message;

extern crate lmdb_rs as lmdb;
use self::lmdb::{Database as LmdbDatabase, DbFlags as LmdbDbFlags, EnvBuilder as LmdbEnvBuilder,
                 Environment as LmdbEnvironment};

use std::collections::HashMap;
use std::path::Path;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

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

        protodb_server.process_metadata_database();

        protodb_server
    }

    fn process_metadata_database(&self) {
        let lmdb_db_handle = self.lmdb_environment
            .get_default_db(LmdbDbFlags::empty())
            .expect("unable to get default LMDB db");

        let txn = self.lmdb_environment.new_transaction().expect("unable to get lmdb transaction");

        {
            let lmdb_db = txn.bind(&lmdb_db_handle);
            {
                if let Some(db) = self.get_mut_databases().get_mut(METADATA_DATABASE) {
                    if let Some(collection) = db.get_collections()
                        .get(DATABASE_METADATA_COLLECTION) {
                        self.process_database_metadata_collection(collection, &lmdb_db);
                    }

                    self.populate_database_metadata_collection(db, &lmdb_db);
                }

                self.populate_metadata_database(&lmdb_db);
            }
        }

        txn.commit().expect("unable to commit create_database");
    }

    fn populate_metadata_database(&self, lmdb_db: &LmdbDatabase) {
        let mut metadata_database =
            self.create_database_with_txn(METADATA_DATABASE, METADATA_DATABASE_DB_ID, lmdb_db)
                .unwrap();

        self.populate_database_metadata_collection(&mut metadata_database, lmdb_db);
    }

    fn process_database_metadata_collection(&self,
                                            collection: &Collection,
                                            lmdb_db: &LmdbDatabase) {
        if let Some(databases) = collection.find_all(&lmdb_db)
            .expect("failed to read metadata collection") {
            for database_data in databases {
                let mut database_data_buf = CodedInputStream::from_bytes(database_data.as_slice());

                let mut database_message = database_proto::Database::new();
                database_message.merge_from(&mut database_data_buf);
                self.create_database(database_message.get_name(), database_message.get__id());
            }
        }
    }

    fn populate_database_metadata_collection(&self,
                                             metadata_database: &mut Database,
                                             lmdb_db: &LmdbDatabase) {
        match metadata_database.create_collection(METADATA_DATABASE_DB_ID,
                                                  DATABASE_METADATA_COLLECTION,
                                                  (*DATABASE_DESCRIPTOR).clone(),
                                                  lmdb_db) {
            Ok(()) => {}
            Err(err) => {
                match err {
                    DatabaseError::CollectionAlreadyExists => {}
                    _ => panic!(err.to_string()),
                }
            }
        }

        let mut metadata_database_message = database_proto::Database::new();
        metadata_database_message.set_name(String::from(METADATA_DATABASE));

        let metadata_database_message_buf = metadata_database_message.write_to_bytes().unwrap();

        metadata_database.get_collections()
            .get(DATABASE_METADATA_COLLECTION)
            .expect(format!("Unable to find {} even though it was just created",
                            DATABASE_METADATA_COLLECTION)
                .as_str())
            .insert(&mut &metadata_database_message_buf[..], lmdb_db)
            .unwrap();
    }

    pub fn create_database(&self, name: &str, db_id: u64) -> Result<()> {
        let lmdb_db_handle = self.lmdb_environment
            .get_default_db(LmdbDbFlags::empty())
            .expect("unable to get default LMDB db");

        let txn = self.lmdb_environment.new_transaction().expect("unable to get lmdb transaction");

        let database: Database;
        {
            let lmdb_db = txn.bind(&lmdb_db_handle);

            database = try!(self.create_database_with_txn(name, db_id, &lmdb_db));
        }

        txn.commit().expect("unable to commit create_database");
        Ok(())
    }

    fn create_database_with_txn(&self,
                                name: &str,
                                db_id: u64,
                                lmdb_db: &LmdbDatabase)
                                -> Result<Database> {
        Ok(try!(Database::new(name, db_id, lmdb_db)))
    }

    pub fn get_databases(&self) -> RwLockReadGuard<HashMap<String, Database>> {
        self.databases.read().expect("lock for databases poisoned")
    }

    pub fn get_mut_databases(&self) -> RwLockWriteGuard<HashMap<String, Database>> {
        self.databases.write().expect("lock for databases poisoned")
    }
}

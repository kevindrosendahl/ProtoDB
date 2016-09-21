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

use std::cmp::max;
use std::collections::HashMap;
use std::option::Option;
use std::path::Path;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub const METADATA_DATABASE: &'static str = "__system";
pub const METADATA_DATABASE_DB_ID: u64 = 1;
pub const DATABASE_METADATA_COLLECTION: &'static str = "databases";
pub const NUM_METADATA_DATABASES: u64 = 32;

lazy_static! {
    static ref DATABASE_DESCRIPTOR: descriptor::DescriptorProto = {
        let file_descriptor_proto = database_proto::file_descriptor_proto();
        descriptor_from_file_descriptor(file_descriptor_proto, "Database")
            .expect("cannot find Database DescriptorProto")
    };
}

pub struct Server {
    databases: RwLock<HashMap<String, Database>>,
    database_id_counter: RwLock<u64>,
    lmdb_environment: LmdbEnvironment,
}

impl Server {
    pub fn new(db_path: &Path) -> Server {
        let databases: HashMap<String, Database> = HashMap::new();

        let lmdb_environment = LmdbEnvBuilder::new()
            .open(db_path, 0o777)
            .expect(format!("unable to open LMDB database at {:?}", db_path).as_str());

        let protodb_server = Server {
            databases: RwLock::new(databases),
            database_id_counter: RwLock::new(NUM_METADATA_DATABASES + 1),
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
            self.populate_metadata_database(&lmdb_db);
            self.process_database_metadata_collection(&lmdb_db);
        }

        txn.commit().expect("unable to commit create_database");
        self.lmdb_environment.sync(true).expect("unable to sync environment to disk");
    }

    fn populate_metadata_database(&self, lmdb_db: &LmdbDatabase) {
        self.create_database_with_txn(METADATA_DATABASE, METADATA_DATABASE_DB_ID, lmdb_db)
            .expect(format!("failed to create {} database", METADATA_DATABASE).as_str());
        self.populate_database_metadata_collection(lmdb_db);
    }

    fn populate_database_metadata_collection(&self, lmdb_db: &LmdbDatabase) {
        // Create databases collection in __system db.
        let mut databases = self.get_mut_databases();
        let mut metadata_database = databases.get_mut(METADATA_DATABASE)
            .expect(format!("could not retrieve {} while trying to populate {}",
                            METADATA_DATABASE,
                            DATABASE_METADATA_COLLECTION)
                .as_str());

        match metadata_database.create_collection(0,
                                                  DATABASE_METADATA_COLLECTION,
                                                  (*DATABASE_DESCRIPTOR).clone(),
                                                  true,
                                                  lmdb_db) {
            Ok(_) => {}
            Err(err) => {
                match err {
                    DatabaseError::CollectionAlreadyExists => {}
                    _ => panic!(err.to_string()),
                }
            }
        }

        // Add __system database to databases metadata collection.
        let mut metadata_database_message = database_proto::Database::new();
        metadata_database_message.set_name(String::from(METADATA_DATABASE));

        let metadata_database_message_buf = metadata_database_message.write_to_bytes().unwrap();

        metadata_database.get_collections()
            .get(DATABASE_METADATA_COLLECTION)
            .expect(format!("Unable to find {} even though it was just created",
                            DATABASE_METADATA_COLLECTION)
                .as_str())
            .insert(&mut &metadata_database_message_buf[..], false, lmdb_db)
            .unwrap();
    }

    fn process_database_metadata_collection(&self, lmdb_db: &LmdbDatabase) {
        let databases_data_option: Option<Vec<Vec<u8>>>;
        {
            let databases = self.get_databases();
            let metadata_database = databases.get(METADATA_DATABASE)
                .expect(format!("Unable to find {} database while attempting to process \
                                 database metadata collection",
                                METADATA_DATABASE)
                    .as_str());

            let collections = metadata_database.get_collections();
            let database_metadata_collection = collections.get(DATABASE_METADATA_COLLECTION)
                .expect(format!("Unable to find {} collection while attempting to process \
                                 database metadata collection",
                                DATABASE_METADATA_COLLECTION)
                    .as_str());

            databases_data_option = database_metadata_collection.find_all(&lmdb_db)
                .expect("failed to read metadata collection");
        }

        let mut max_db_id = NUM_METADATA_DATABASES;
        if let Some(databases_data) = databases_data_option {
            for database_data in databases_data {
                let mut database_data_buf = CodedInputStream::from_bytes(database_data.as_slice());

                let mut database_message = database_proto::Database::new();
                database_message.merge_from(&mut database_data_buf)
                    .expect("Unable to create database object from proto while attempting to \
                             process database metadata collection");

                println!("found db: {} {}",
                         database_message.get_name(),
                         database_message.get__id());
                max_db_id = max(max_db_id, database_message.get__id());

                match self.create_database_with_txn(database_message.get_name(),
                                                    database_message.get__id(),
                                                    lmdb_db) {
                    Ok(()) => {}
                    Err(err) => {
                        match err {
                            ServerError::DatabaseAlreadyExists => {}
                            _ => panic!(err.to_string()),
                        }
                    }
                }
            }
        }

        println!("got max id: {}", max_db_id);
        self.set_database_id_counter(max_db_id + 1);
    }

    pub fn get_databases(&self) -> RwLockReadGuard<HashMap<String, Database>> {
        self.databases.read().expect("lock for databases poisoned")
    }

    pub fn get_mut_databases(&self) -> RwLockWriteGuard<HashMap<String, Database>> {
        self.databases.write().expect("lock for databases poisoned")
    }

    fn next_database_id(&self) -> u64 {
        let database_id: u64;

        let mut database_id_counter = self.database_id_counter
            .write()
            .expect("lock for database_id_counter poisoned");
        database_id = *database_id_counter;
        *database_id_counter += 1;

        database_id
    }

    fn set_database_id_counter(&self, new_value: u64) {
        let database_id: u64;

        let mut database_id_counter = self.database_id_counter
            .write()
            .expect("lock for database_id_counter poisoned");
        *database_id_counter = new_value;
    }

    pub fn create_database(&self, name: &str, mut db_id: u64) -> Result<()> {
        if self.get_databases().contains_key(name) {
            return Err(ServerError::DatabaseAlreadyExists);
        }

        if db_id == 0 {
            db_id = self.next_database_id();
        }

        let lmdb_db_handle = self.lmdb_environment
            .get_default_db(LmdbDbFlags::empty())
            .expect("unable to get default LMDB db");

        let txn = self.lmdb_environment.new_transaction().expect("unable to get lmdb transaction");

        {
            let lmdb_db = txn.bind(&lmdb_db_handle);
            try!(self.create_database_with_txn(name, db_id, &lmdb_db));
        }

        txn.commit().expect("unable to commit create_database");
        Ok(())
    }

    fn create_database_with_txn(&self,
                                name: &str,
                                mut db_id: u64,
                                lmdb_db: &LmdbDatabase)
                                -> Result<()> {
        if self.database_exists(name) {
            return Err(ServerError::DatabaseAlreadyExists);
        }

        if name != METADATA_DATABASE {
            let mut database_collection_entry = database_proto::Database::new();
            database_collection_entry.set__id(db_id);
            database_collection_entry.set_name(String::from(name.clone()));
            let mut database_collection_entry_buf = database_collection_entry.write_to_bytes()
                .unwrap();

            let databases = self.get_databases();
            let metadata_database = databases.get(METADATA_DATABASE)
                .expect(format!("unable to find {} database", METADATA_DATABASE).as_str());

            let collections = metadata_database.get_collections();
            let database_metadata_collection = collections.get(DATABASE_METADATA_COLLECTION)
                .expect(format!("unable to find {} collection in {} database",
                                DATABASE_METADATA_COLLECTION,
                                METADATA_DATABASE)
                    .as_str());
            database_metadata_collection.insert(&mut &database_collection_entry_buf[..],
                                                true,
                                                lmdb_db)
                .unwrap();
        }

        let database = try!(Database::new(name, db_id, lmdb_db));
        self.get_mut_databases().insert(String::from(name), database);
        Ok(())
    }

    pub fn get_database_names(&self) -> Vec<String> {
        self.get_databases().values().map(|db| db.name.clone()).collect()
    }

    pub fn database_exists(&self, name: &str) -> bool {
        self.get_databases().contains_key(name)
    }

    pub fn create_collection(&self,
                             db_name: &str,
                             collection_name: &str,
                             schema: descriptor::DescriptorProto)
                             -> Result<()> {
        let lmdb_db_handle = self.lmdb_environment
            .get_default_db(LmdbDbFlags::empty())
            .expect("unable to get default LMDB db");

        let txn = self.lmdb_environment.new_transaction().expect("unable to get lmdb transaction");

        {
            let lmdb_db = txn.bind(&lmdb_db_handle);
            try!(self.create_collection_with_txn(db_name, collection_name, schema, &lmdb_db));
        }

        txn.commit().expect("unable to commit create_collection");

        println!("double returning okay");
        Ok(())
    }

    fn create_collection_with_txn(&self,
                                  db_name: &str,
                                  collection_name: &str,
                                  schema: descriptor::DescriptorProto,
                                  lmdb_db: &LmdbDatabase)
                                  -> Result<()> {
        let mut databases = self.get_mut_databases();
        println!("getting db with name {}", db_name);
        for key in databases.keys() {
            println!("key: {}", key);
        }
        println!("db: {:?}",
                 databases.get_mut(db_name).map(|db| db.name.clone()));
        let mut db = try!(databases.get_mut(db_name)
            .ok_or(ServerError::DatabaseDoesNotExist));
        try!(db.create_collection(0, collection_name, schema, true, lmdb_db));
        println!("returning okay");
        Ok(())
    }

    pub fn get_collection_names(&self, db_name: &str) -> Result<Vec<String>> {
        let databases = self.get_databases();
        let db = try!(databases.get(db_name).ok_or(ServerError::DatabaseDoesNotExist));
        Ok(db.get_collection_names())
    }

    pub fn find(&self,
                database_name: &str,
                collection_name: &str,
                obj_id: u64)
                -> Result<Option<Vec<u8>>> {
        let lmdb_db_handle = self.lmdb_environment
            .get_default_db(LmdbDbFlags::empty())
            .expect("unable to get default LMDB db");

        let read_txn = self.lmdb_environment.get_reader().expect("unable to get lmdb reader");

        let databases = self.get_databases();
        let database = try!(databases.get(database_name).ok_or(ServerError::DatabaseDoesNotExist));

        let collections = database.get_collections();
        let collection = try!(collections.get(collection_name)
            .ok_or(ServerError::CollectionDoesNotExist));

        let lmdb_db = read_txn.bind(&lmdb_db_handle);
        Ok(try!(collection.find(obj_id, &lmdb_db)))
    }

    pub fn find_all(&self,
                    database_name: &str,
                    collection_name: &str)
                    -> Result<Option<Vec<Vec<u8>>>> {
        let lmdb_db_handle = self.lmdb_environment
            .get_default_db(LmdbDbFlags::empty())
            .expect("unable to get default LMDB db");

        let read_txn = self.lmdb_environment.get_reader().expect("unable to get lmdb reader");

        let databases = self.get_databases();
        let database = try!(databases.get(database_name).ok_or(ServerError::DatabaseDoesNotExist));

        let collections = database.get_collections();
        let collection = try!(collections.get(collection_name)
            .ok_or(ServerError::CollectionDoesNotExist));

        let lmdb_db = read_txn.bind(&lmdb_db_handle);
        Ok(try!(collection.find_all(&lmdb_db)))
    }

    pub fn insert(&self,
                  database_name: &str,
                  collection_name: &str,
                  data: &mut Vec<u8>)
                  -> Result<u64> {
        let lmdb_db_handle = self.lmdb_environment
            .get_default_db(LmdbDbFlags::empty())
            .expect("unable to get default LMDB db");

        let txn = self.lmdb_environment.new_transaction().expect("unable to get lmdb transaction");

        let databases = self.get_databases();
        let database = try!(databases.get(database_name).ok_or(ServerError::DatabaseDoesNotExist));

        let collections = database.get_collections();
        let collection = try!(collections.get(collection_name)
            .ok_or(ServerError::CollectionDoesNotExist));

        let _id: u64;
        {
            let lmdb_db = txn.bind(&lmdb_db_handle);
            _id = try!(collection.insert(&mut data.as_slice(), false, &lmdb_db));
        }

        txn.commit().expect("unable to commit create_database");
        Ok(_id)
    }
}

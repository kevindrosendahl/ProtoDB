mod errors;
mod result;

pub use self::errors::DatabaseError;
pub use self::result::Result;

use super::collection::{Collection, Insert, Find};
use super::protos::collection as collection_proto;
use super::util::protobuf::descriptor_from_file_descriptor;

use ::protobuf::descriptor;
use ::protobuf::Message;
use ::protobuf::stream::CodedInputStream;

extern crate lmdb_rs as lmdb;
use self::lmdb::Database as LmdbDatabase;

use std::collections::HashMap;
use std::option::Option;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub const COLLECTION_METADATA_COLLECTION: &'static str = "__collections";

lazy_static! {
    // Going to need the DescriptorProto of the Collection proto
    // every time a new database is created.
    static ref COLLECTION_DESCRIPTOR: descriptor::DescriptorProto = {
        let file_descriptor_proto = collection_proto::file_descriptor_proto();
        descriptor_from_file_descriptor(file_descriptor_proto, "Collection")
            .expect("cannot find Collection DescriptorProto")
    };
}

pub struct Database {
    id: u64,
    name: String,
    collections: RwLock<HashMap<String, Collection>>,
    collection_id_counter: RwLock<u64>,
}

impl Database {
    pub fn new(name: &str, id: u64, lmdb_db: &LmdbDatabase) -> Result<Database> {
        let collections: HashMap<String, Collection> = HashMap::new();

        let mut db = Database {
            id: id,
            name: String::from(name),
            collections: RwLock::new(collections),
            collection_id_counter: RwLock::new(32), // reserve first 32 for internal usage.
        };

        // Create the database's collection metadata collection.
        try!(db.create_collection(1,
                                  COLLECTION_METADATA_COLLECTION,
                                  (*COLLECTION_DESCRIPTOR).clone(),
                                  lmdb_db));

        try!(db.process_collection_metadata_collection(lmdb_db));

        Ok(db)
    }

    pub fn create_collection(&mut self,
                             mut collection_id: u64,
                             collection_name: &str,
                             schema: descriptor::DescriptorProto,
                             lmdb_db: &LmdbDatabase)
                             -> Result<()> {
        if self.get_collections().contains_key(collection_name) {
            return Err(DatabaseError::CollectionAlreadyExists);
        }

        if collection_id == 0 {
            collection_id = self.next_collection_id();
        }

        // Add to database's collection map.
        // This has to go before inserting the collection into the
        // COLLECTION_METADATA_COLLECTION so we can boostrap that collection.
        let collection = try!(Collection::new(collection_id,
                                              String::from(collection_name),
                                              self.id,
                                              schema.clone()));
        self.get_mut_collections().insert(String::from(collection_name), collection);

        // Create Collection proto message.
        let mut collection_message = collection_proto::Collection::new();
        collection_message.set_name(String::from(collection_name));
        collection_message.set_schema(schema.clone());

        // Get serialized version of Collection proto message.
        let collection_message_buf = try!(collection_message.write_to_bytes());

        // Attempt to insert Collection proto message into
        // collection metadata collection.
        let insert_result: Result<()>;
        {
            // get_collection will borrow self until the end of its
            // scope, so need to do this in its own block.
            let collections = self.get_collections();
            let collection_metadata_collection = collections.get(COLLECTION_METADATA_COLLECTION)
                .expect(format!("{} collection cannot be found",
                                COLLECTION_METADATA_COLLECTION)
                    .as_str());

            let collection_data = &mut &collection_message_buf[..];
            insert_result = collection_metadata_collection.insert(collection_data, lmdb_db)
                .map_err(|err| DatabaseError::CollectionError(err));
        }

        // Clean up if insert failed.
        if insert_result.is_err() {
            self.get_mut_collections().remove(collection_name);
        }

        insert_result
    }

    fn next_collection_id(&self) -> u64 {
        let collection_id: u64;

        let mut collection_id_counter = self.collection_id_counter
            .write()
            .expect(format!("lock for database {} collection_id_counter poisoned",
                            self.name)
                .as_str());
        collection_id = *collection_id_counter;
        *collection_id_counter += 1;

        collection_id
    }

    fn process_collection_metadata_collection(&mut self, lmdb_db: &LmdbDatabase) -> Result<()> {
        let collections_data_option: Option<Vec<Vec<u8>>>;
        {
            let collections = self.get_collections();
            let collection_metadata_collection = collections.get(COLLECTION_METADATA_COLLECTION)
                .expect(format!("{} collection cannot be found",
                                COLLECTION_METADATA_COLLECTION)
                    .as_str());

            collections_data_option = collection_metadata_collection.find_all(lmdb_db)
                .expect(format!("failed to read {} database {} collection",
                                self.name,
                                COLLECTION_METADATA_COLLECTION)
                    .as_str());
        }

        if let Some(collections_data) = collections_data_option {
            for collection_data in collections_data {
                let mut collection_data_buf =
                    CodedInputStream::from_bytes(collection_data.as_slice());

                let mut collection_message = collection_proto::Collection::new();
                collection_message.merge_from(&mut collection_data_buf);
                match self.create_collection(collection_message.get__id(),
                                             collection_message.get_name(),
                                             collection_message.get_schema().clone(),
                                             lmdb_db) {
                    Ok(()) => {}
                    Err(err) => {
                        match err {
                            DatabaseError::CollectionAlreadyExists => {}
                            _ => return Err(err),
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn get_collections(&self) -> RwLockReadGuard<HashMap<String, Collection>> {
        self.collections
            .read()
            .expect(format!("lock for database {} collections poisoned", self.name).as_str())
    }

    pub fn get_mut_collections(&self) -> RwLockWriteGuard<HashMap<String, Collection>> {
        self.collections
            .write()
            .expect(format!("lock for database {} collections poisoned", self.name).as_str())
    }
}

#[cfg(test)]
mod tests {
    extern crate lmdb_rs as lmdb;
    use self::lmdb::{DbFlags, EnvBuilder};

    extern crate protobuf;
    use self::protobuf::Message;

    use super::*;

    use server::protos;
    use server::collection::Find;

    use std::path;

    #[test]
    fn test_new_database() {
        let db_path = path::Path::new("/tmp/protodb_database_test_new_lmdb");
        let env = EnvBuilder::new().open(&db_path, 0o777).unwrap();

        let db_handle = env.get_default_db(DbFlags::empty()).unwrap();
        let txn = env.new_transaction().unwrap();

        let database: Database;

        {
            let db = txn.bind(&db_handle);

            database = Database::new("test_db", 0, &db).expect("Database::new failed");
        }

        txn.commit().expect("commit failed");

        let reader = env.get_reader().unwrap();
        let db = reader.bind(&db_handle);

        let collections = database.get_collections();
        let collections_collection = collections.get(COLLECTION_METADATA_COLLECTION)
            .expect(format!("{} collection cannot be found",
                            COLLECTION_METADATA_COLLECTION)
                .as_str());

        let collection_data = collections_collection.find(1, &db)
            .expect("collections database obj_id 1 find failed")
            .expect("collections database obj_id 1 does not exist");

        let mut collection_proto = protos::collection::Collection::new();

        {
            let collection_data_ref = &mut &collection_data[..];
            let mut input_stream = protobuf::stream::CodedInputStream::new(collection_data_ref);
            collection_proto.merge_from(&mut input_stream).expect("merge from failed");
        }
        println!("collection_data: {:?}", collection_proto);
    }
}

mod errors;
mod result;

pub use self::errors::DatabaseError;
pub use self::result::Result;

use super::collection::{Collection, Insert, Find};
use super::protos::collection as collection_proto;
use super::util::protobuf::descriptor_from_file_descriptor;

use ::protobuf::descriptor;
use ::protobuf::Message;

extern crate lmdb_rs as lmdb;
use self::lmdb::Database as LmdbDatabase;

use std::collections::HashMap;
use std::sync::RwLock;

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
    collections: HashMap<String, Collection>,
    collection_id_counter: RwLock<u64>,
}

impl Database {
    pub fn new(name: String, id: u64, lmdb_database: &LmdbDatabase) -> Result<Database> {
        let mut collections: HashMap<String, Collection> = HashMap::new();

        let mut db = Database {
            id: id,
            name: name,
            collections: collections,
            collection_id_counter: RwLock::new(32), // reserve first 32 for internal usage.
        };

        // Create the database's collection metadata collection.
        try!(db.create_collection(0,
                                  String::from(COLLECTION_METADATA_COLLECTION),
                                  (*COLLECTION_DESCRIPTOR).clone(),
                                  lmdb_database));

        Ok(db)
    }

    pub fn create_collection(&mut self,
                             collection_id: u64,
                             collection_name: String,
                             schema: descriptor::DescriptorProto,
                             lmdb_database: &LmdbDatabase)
                             -> Result<()> {
        if self.collections.contains_key(&collection_name) {
            return Err(DatabaseError::CollectionAlreadyExists);
        }

        // Add to database's collection map.
        let collection = try!(Collection::new(collection_id,
                                              collection_name.clone(),
                                              self.id,
                                              schema.clone()));
        self.collections.insert(collection_name.clone(), collection);

        // Create Collection proto message.
        let mut collection_message = collection_proto::Collection::new();
        collection_message.set_name(collection_name.clone());
        collection_message.set_schema(schema.clone());

        // Get serialized version of Collection proto message.
        let collection_message_buf = try!(collection_message.write_to_bytes());

        // Attempt to insert Collection proto message into
        // collection metadata collection.
        let insert_result: Result<()>;
        {
            // get_collection will borrow self until the end of its
            // scope, so need to do this in its own block.
            let collection_metadata_collection =
                self.get_collection(String::from(COLLECTION_METADATA_COLLECTION))
                    .expect(format!("{} collection cannot be found",
                                    COLLECTION_METADATA_COLLECTION)
                        .as_str());

            let collection_data = &mut &collection_message_buf[..];
            insert_result = collection_metadata_collection.insert(collection_data, lmdb_database)
                .map_err(|err| DatabaseError::CollectionError(err));
        }

        // Clean up if insert failed.
        if insert_result.is_err() {
            self.collections.remove(&collection_name);
        }

        insert_result
    }

    pub fn get_collection(&self, name: String) -> Option<&Collection> {
        self.collections.get(&name)
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

            database = Database::new(String::from("test_db"), 0, &db)
                .expect("Database::new failed");
        }

        txn.commit().expect("commit failed");

        let reader = env.get_reader().unwrap();
        let db = reader.bind(&db_handle);

        let collections_collection =
            database.get_collection(String::from(COLLECTION_METADATA_COLLECTION))
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

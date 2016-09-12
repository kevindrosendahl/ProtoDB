mod errors;
mod insert;
mod find;
mod result;

pub use self::errors::CollectionError;
pub use self::insert::Insert;
pub use self::find::Find;
pub use self::result::Result;

use super::schema::Schema;
use super::util::encoding::{MAX_UVARINT_LEN, encode_uvarint_into, decode_uvarint};

use protobuf::descriptor;

use std::ops::Range;
use std::sync;

pub struct Collection {
    id: u64,
    name: String,
    db_id: u64,
    key_prefix: Vec<u8>,
    schema: Schema,
    id_counter: sync::RwLock<u64>,
}

impl Collection {
    pub fn new(id: u64,
               name: String,
               db_id: u64,
               descriptor: descriptor::DescriptorProto)
               -> Result<Collection> {
        let mut buf = [0u8; 18];
        let mut key_len = try!(encode_uvarint_into(db_id, &mut buf));
        key_len += try!(encode_uvarint_into(id, &mut buf[key_len..]));
        let key_prefix = <Vec<u8> as From<&[u8]>>::from(&buf[..key_len]);

        let schema = try!(Schema::new(descriptor));

        Ok(Collection {
            id: id,
            name: name,
            db_id: db_id,
            key_prefix: key_prefix,
            schema: schema,
            id_counter: sync::RwLock::new(1),
        })
    }

    fn decode_key(&self, key_buf: &[u8]) -> Result<DecodedKey> {
        let (_, db_id_len) = try!(decode_uvarint(&key_buf));
        let (_, collection_id_len) = try!(decode_uvarint(&key_buf[db_id_len..]));
        let (obj_id, obj_id_len) = try!(decode_uvarint(&key_buf[db_id_len + collection_id_len..]));

        let remaining_buf = &key_buf[db_id_len + collection_id_len + obj_id_len..];
        if remaining_buf.len() == 0 {
            return Ok(DecodedKey {
                obj_id: obj_id,
                column_id_option: None,
            });
        }

        let (column_id, _) = try!(decode_uvarint(remaining_buf));
        Ok(DecodedKey {
            obj_id: obj_id,
            column_id_option: Some(column_id as u32),
        })
    }

    fn key_range(&self, range: &Range<u64>) -> Result<(Vec<u8>, Vec<u8>)> {
        let max_key_len = (3 * MAX_UVARINT_LEN) as usize;
        let mut start_key = Vec::with_capacity(max_key_len);
        let mut end_key = Vec::with_capacity(max_key_len);

        {
            let key_prefix_len = &self.key_prefix.len();
            let mut key_buf = [0u8; 9];

            let mut append_obj_key_to_buf = |obj_id, key: &mut Vec<u8>| -> Result<usize> {
                key.extend(&self.key_prefix);

                let key_suffix_len = try!(encode_uvarint_into(obj_id, &mut key_buf[..]));
                key.extend(&key_buf[..key_suffix_len]);

                Ok(key_prefix_len + key_suffix_len)
            };

            try!(append_obj_key_to_buf(range.start, &mut start_key));
            try!(append_obj_key_to_buf(range.end, &mut end_key));
        }

        Ok((start_key, end_key))
    }

    fn next_obj_id(&self) -> u64 {
        let _id: u64;

        let mut id_counter = self.id_counter
            .write()
            .expect(format!("lock for database {} collection {} id_counter poisoned",
                            self.db_id,
                            self.id)
                .as_str());
        _id = *id_counter;
        *id_counter += 1;

        _id
    }
}

struct DecodedKey {
    obj_id: u64,
    column_id_option: Option<u32>,
}


#[cfg(test)]
mod tests {
    extern crate lmdb_rs as lmdb;
    use self::lmdb::{DbFlags, EnvBuilder};

    use super::*;

    use server::util;
    use server::protos::test;

    use std::fs;
    use std::path;

    #[test]
    fn test_new_collection() {
        let descriptor =
            util::protobuf::descriptor_from_file_descriptor(test::file_descriptor_proto(),
                                                            "ScalarSchema")
                .expect("Could not find descriptor");

        let collection = Collection::new(2, String::from("testcoll"), 1, descriptor).unwrap();
        assert_eq!(collection.key_prefix, [1, 1, 1, 2]);
    }

    #[test]
    fn test_insert() {
        let db_path = path::Path::new("/tmp/protodb_collection_test_insert_lmdb");
        let env = EnvBuilder::new().open(&db_path, 0o777).unwrap();

        let db_handle = env.get_default_db(DbFlags::empty()).unwrap();
        let txn = env.new_transaction().unwrap();

        {
            let db = txn.bind(&db_handle);
            let descriptor =
                util::protobuf::descriptor_from_file_descriptor(test::file_descriptor_proto(),
                                                                "ScalarSchema")
                    .expect("Could not find descriptor");

            let collection = Collection::new(2, String::from("testcoll"), 1, descriptor).unwrap();
            let mut data: &[u8] = &[8, 0, 112, 1];
            collection.insert(&mut data, &db).expect("insert failed");
        }

        txn.commit().expect("commit failed");

        let reader = env.get_reader().unwrap();
        let db = reader.bind(&db_handle);

        assert_eq!(1,
                   db.get::<u8>(&vec![1u8, 1u8, 1u8, 2u8, 1u8, 1u8, 1u8, 14u8])
                       .expect("get failed"));

        fs::remove_dir_all(&db_path).unwrap();
    }

    #[test]
    fn test_find() {
        let db_path = path::Path::new("/tmp/protodb_collection_test_find_lmdb");
        let env = EnvBuilder::new().open(&db_path, 0o777).unwrap();

        let db_handle = env.get_default_db(DbFlags::empty()).unwrap();
        let txn = env.new_transaction().unwrap();

        let descriptor =
            util::protobuf::descriptor_from_file_descriptor(test::file_descriptor_proto(),
                                                            "ScalarSchema")
                .expect("Could not find descriptor");

        let collection = Collection::new(2, String::from("testcoll"), 1, descriptor).unwrap();
        {
            let db = txn.bind(&db_handle);
            let mut data: &[u8] = &[8, 0, 112, 1];
            collection.insert(&mut data, &db).expect("insert failed");
        }

        txn.commit().expect("commit failed");

        let reader = env.get_reader().unwrap();
        let db = reader.bind(&db_handle);

        let found_data = collection.find(1, &db).expect("find failed").expect("could not find");
        assert_eq!(found_data, &[8, 1, 112, 1]);

        fs::remove_dir_all(&db_path).expect("failed on db teardown");
    }
}

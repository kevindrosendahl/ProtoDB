use super::{Collection, Result};

use server::schema::EncodeField;

extern crate lmdb_rs as lmdb;
use self::lmdb::Database as LmdbDatabase;

use std::ops::Range;

pub trait Find {
    fn find_range(&self, &Range<u64>, &LmdbDatabase) -> Result<Option<Vec<Vec<u8>>>>;
    fn find(&self, u64, &LmdbDatabase) -> Result<Option<Vec<u8>>>;
    fn find_all(&self, &LmdbDatabase) -> Result<Option<Vec<Vec<u8>>>>;
}

impl Find for Collection {
    fn find_range(&self,
                  range: &Range<u64>,
                  lmdb_database: &LmdbDatabase)
                  -> Result<Option<Vec<Vec<u8>>>> {
        let (start_key, end_key) = try!(self.key_range(range));

        let mut data: Vec<Vec<u8>> = Vec::new();
        let mut obj_data: Vec<u8> = Vec::new();
        let mut encode_buf: Vec<u8> = Vec::new();
        let mut tombstone_id = 0u64;
        let mut current_obj_id = 0u64;

        // For each key found, if it is the marker key then encode the obj_id and its
        // tag, otherwise encode the tag and column value .
        for result in try!(lmdb_database.keyrange_from_to(&start_key, &end_key)) {
            encode_buf.clear();

            let decoded_key = try!(self.decode_key(result.get_key()));
            if let Some(column_id) = decoded_key.column_id_option {
                // Just a normal column key/value pair.
                let value = result.get_value::<&[u8]>();
                if decoded_key.obj_id == tombstone_id {
                    panic!("Column {} with value {:?} found for deleted obj_id {}",
                           column_id,
                           value,
                           decoded_key.obj_id);
                }

                // Add encoded tag to obj_data.
                try!(self.schema
                    .encode_tag(column_id,
                                &mut ::protobuf::CodedOutputStream::new(&mut encode_buf)));
                obj_data.extend(&encode_buf);

                // Add the already encoded value to obj_data.
                obj_data.extend(value);
            } else {
                // Marker key.
                if result.get_value::<&[u8]>() == [0] {
                    tombstone_id = decoded_key.obj_id;
                    continue;
                }

                // If it's a new object, add the old object's data
                // to data if there was any.
                if decoded_key.obj_id != current_obj_id {
                    if obj_data.len() > 0 {
                        data.push(obj_data.clone());
                        obj_data.clear();
                    }
                    current_obj_id = decoded_key.obj_id;
                }

                // Add encoded tag to obj_data.
                try!(self.schema
                    .encode_tag(self.schema.id_field_number,
                                &mut ::protobuf::CodedOutputStream::new(&mut encode_buf)));
                obj_data.extend(&encode_buf);

                // Add encoded obj_id to obj_data.
                encode_buf.clear();
                try!(self.schema
                    .encode_field(self.schema.id_field_number,
                                  decoded_key.obj_id,
                                  &mut ::protobuf::CodedOutputStream::new(&mut encode_buf)));
                obj_data.extend(&encode_buf);
            }
        } // end for

        // Push any remaining object into data.
        if obj_data.len() > 0 {
            data.push(obj_data);
        }

        match data.len() {
            0 => Ok(None),
            _ => Ok(Some(data)),
        }
    }

    fn find(&self, obj_id: u64, lmdb_database: &LmdbDatabase) -> Result<Option<Vec<u8>>> {
        let results = try!(self.find_range(&(obj_id..obj_id + 1), lmdb_database));
        Ok(results.map(|objects| {
            if objects.len() > 1 {
                panic!("attempted to find single object but got multiple results");
            }
            objects[0].clone()
        }))
    }

    fn find_all(&self, lmdb_database: &LmdbDatabase) -> Result<Option<Vec<Vec<u8>>>> {
        self.find_range(&(0u64..u64::max_value()), lmdb_database)
    }
}

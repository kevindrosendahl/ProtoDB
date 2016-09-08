use super::{Collection, Result};

use server::schema::EncodeField;

extern crate lmdb_rs as lmdb;
use self::lmdb::Database as LmdbDatabase;

pub trait Find {
    fn find(&self, u64, &LmdbDatabase) -> Result<Option<Vec<u8>>>;
}

impl Find for Collection {
    fn find(&self, obj_id: u64, database: &LmdbDatabase) -> Result<Option<Vec<u8>>> {
        let (start_key, end_key) = try!(self.key_range(obj_id, obj_id + 1));

        let mut data: Vec<u8> = Vec::new();
        let mut encode_buf: Vec<u8> = Vec::new();
        let mut tombstone_id = 0u64;


        // For each key found, if it is the marker key then encode the obj_id and its
        // tag, otherwise encode the tag and column value .
        for result in try!(database.keyrange_from_to(&start_key, &end_key)) {
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

                // Add encoded tag to data.
                try!(self.schema
                    .encode_tag(column_id,
                                &mut ::protobuf::CodedOutputStream::new(&mut encode_buf)));
                data.extend(&encode_buf);

                // Add the already encoded value to data.
                data.extend(value);
            } else {
                // Marker key.
                if result.get_value::<&[u8]>() == [0] {
                    tombstone_id = decoded_key.obj_id;
                    continue;
                }
                
                // Add encoded tag to data.
                try!(self.schema
                    .encode_tag(self.schema.id_field_number,
                                &mut ::protobuf::CodedOutputStream::new(&mut encode_buf)));
                data.extend(&encode_buf);

                // Add encoded obj_id to data.
                encode_buf.clear();
                try!(self.schema
                    .encode_field(self.schema.id_field_number,
                                  obj_id,
                                  &mut ::protobuf::CodedOutputStream::new(&mut encode_buf)));
                data.extend(&encode_buf);
            }
        } // end for

        match data.len() {
            0 => Ok(None),
            _ => Ok(Some(data)),
        }
    }
}

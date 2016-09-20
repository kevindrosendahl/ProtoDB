use super::{Collection, CollectionError, Result};

use server::schema::{DecodeField, DecodeObject};
use server::util::encoding::{encode_uvarint_into, MAX_UVARINT_LEN};

use ::protobuf::descriptor;

extern crate lmdb_rs as lmdb;
use self::lmdb::Database as LmdbDatabase;

pub trait Insert {
    fn insert<'a>(&'a self, &'a mut &'a [u8], bool, &LmdbDatabase) -> Result<u64>;
}


// TODO: refactor into insert, and insert_with_id that both bottom out to this insert
//       so you don't always have to pass an allow_set_obj_id val when calling insert.
impl Insert for Collection {
    fn insert<'a>(&'a self,
                  data: &'a mut &'a [u8],
                  allow_set_obj_id: bool,
                  database: &LmdbDatabase)
                  -> Result<u64> {
        let (inserts, obj_id_option) = try!(self.get_inserts(data, allow_set_obj_id));

        let obj_id = if allow_set_obj_id {
            obj_id_option.expect("allow_set_obj_id set but no obj_id included")
        } else {
            self.next_obj_id()
        };
        let mut buf = [0u8; (2 * MAX_UVARINT_LEN) as usize];
        let obj_id_len = try!(encode_uvarint_into(obj_id, &mut buf));

        // Insert the marker key.
        let marker_key = [&self.key_prefix, &buf[..obj_id_len]].concat();
        try!(database.set(&marker_key, &vec![1]));

        for insert in inserts {
            let column_id_len = try!(encode_uvarint_into(insert.column_id as u64,
                                                         &mut buf[obj_id_len..]));

            let key = [&self.key_prefix, &buf[..obj_id_len + column_id_len]].concat();
            try!(database.set(&key, &insert.data));
        }

        Ok(obj_id)
    }
}

trait GetInserts {
    fn get_inserts<'a>(&'a self,
                       &'a mut &'a [u8],
                       bool)
                       -> Result<(Vec<InsertKVPair>, Option<u64>)>;
}

impl GetInserts for Collection {
    fn get_inserts<'a>(&'a self,
                       data: &'a mut &'a [u8],
                       allow_set_obj_id: bool)
                       -> Result<(Vec<InsertKVPair>, Option<u64>)> {
        let mut inserts: Vec<InsertKVPair> = Vec::with_capacity(self.schema.num_fields);
        let mut obj_id_option = None;

        for result in self.schema.decode_object(data) {
            match result {
                Ok((field_number, field_data)) => {
                    // If the column is the _id column, simply make sure that it is not set.
                    if field_number == self.schema.id_field_number {
                        let id_field_val: u64 = try!(self.schema
                            .decode_field(&descriptor::FieldDescriptorProto_Type::TYPE_UINT64,
                                &mut ::protobuf::CodedInputStream::new(&mut &field_data[..])));

                        if id_field_val != 0 {
                            if allow_set_obj_id {
                                obj_id_option = Some(id_field_val)
                            } else {
                                return Err(CollectionError::IdFieldSetOnInsert);
                            }
                        }

                        continue;
                    }

                    inserts.push(InsertKVPair {
                        column_id: field_number,
                        data: field_data,
                    });
                }
                Err(err) => return Err(CollectionError::SchemaError(err)),
            } // end match
        } // end for

        Ok((inserts, obj_id_option))
    }
}

#[derive(Debug)]
struct InsertKVPair {
    column_id: u32,
    data: Vec<u8>,
}

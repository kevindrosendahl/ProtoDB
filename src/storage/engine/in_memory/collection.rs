use super::cache::{Cache, Range};
use crate::storage::{
    errors,
    schema::{
        encoding::{FieldInfo, FieldValue},
        errors::{ObjectError, SchemaError},
        Schema,
    },
};

use byteorder::{LittleEndian, WriteBytesExt};
use prost_types::DescriptorProto;

const KEY_DELIMITER: &str = "/";
const NULL_CHAR: &str = "\u{0}";

pub(crate) struct Collection {
    pub database: String,
    pub name: String,
    pub schema: Schema,
    cache: Cache,
}

macro_rules! value_encoder {
    (  $val:ident, $encoder:ident, $size:expr ) => {{
        let mut writer = Vec::with_capacity($size);
        writer.$encoder::<LittleEndian>($val).unwrap();
        writer
    }};
}

impl Collection {
    pub fn new(
        database: String,
        name: String,
        descriptor: &DescriptorProto,
    ) -> Result<Collection, SchemaError> {
        let schema = Schema::new(descriptor)?;
        Ok(Collection {
            database,
            name,
            schema,
            cache: Default::default(),
        })
    }

    pub fn insert_object(
        &self,
        object: &[u8],
    ) -> Result<(), errors::collection::InsertObjectError> {
        let mut id = None;
        let fields = self
            .schema
            .decode_object(object)
            .map(|f| {
                // check to see if this field is the id field. if it is,
                // ensure that the value is a Uint64 (for now) and set id
                // to it
                if f.is_err() {
                    return f;
                }

                let f = f.unwrap();
                if f.tag != self.schema.id_field {
                    return Ok(f);
                }

                match f.value {
                    FieldValue::Uint64(val) => {
                        id = Some(val);
                        Ok(f)
                    }
                    _ => Err(ObjectError::SchemaError(SchemaError::InvalidIdType(
                        format!("{:?}", f.value),
                    ))),
                }
            }).collect::<Result<Vec<FieldInfo>, ObjectError>>()?;

        if id.is_none() {
            return Err(errors::collection::InsertObjectError::ObjectError(
                ObjectError::SchemaError(SchemaError::MissingIdField),
            ));
        }

        let id = id.unwrap();
        for field in fields {
            self.cache.put(
                self.field_key(id, field.tag).as_bytes().to_vec(),
                Self::field_encoding(field.value),
            )
        }

        Ok(())
    }

    pub fn find_object(&self, id: u64) -> Result<Vec<u8>, errors::collection::FindObjectError> {
//        let start = self.object_key_prefix(id);
//        let mut end = start.clone();
//        end.push_str(NULL_CHAR);
//
//        let mut buf = Vec::new();
//        // the id is encoded as part of the key, so we won't
//        // see it during the iteration over the fields, so add
//        // it to the buffer first
//        self.schema.encode_field(self.schema.id_field, FieldValue::Uint64(id), &mut buf);
//
//        let entries = self.cache.get_range(&Range {
//            start: start.into_bytes(),
//            end: end.into_bytes(),
//        });
//        for (key, value) in entries {
//
//        }
        Ok(vec![])
    }

    fn field_encoding(value: FieldValue) -> Vec<u8> {
        // TODO: can probably macro this out a bit more
        match value {
            FieldValue::Float(val) => value_encoder!(val, write_f32, 4),
            FieldValue::Double(val) => value_encoder!(val, write_f64, 8),
            FieldValue::Int32(val) => value_encoder!(val, write_i32, 4),
            FieldValue::Int64(val) => value_encoder!(val, write_i64, 8),
            FieldValue::Uint32(val) => value_encoder!(val, write_u32, 4),
            FieldValue::Uint64(val) => value_encoder!(val, write_u64, 8),
            FieldValue::Sint32(val) => value_encoder!(val, write_i32, 4),
            FieldValue::Sint64(val) => value_encoder!(val, write_i64, 8),
            FieldValue::Fixed32(val) => value_encoder!(val, write_u32, 4),
            FieldValue::Fixed64(val) => value_encoder!(val, write_u64, 8),
            FieldValue::Sfixed32(val) => value_encoder!(val, write_i32, 4),
            FieldValue::Sfixed64(val) => value_encoder!(val, write_i64, 8),
            FieldValue::Bool(val) => {
                let val = val as u32;
                value_encoder!(val, write_u32, 4)
            }
            FieldValue::String(val) => val.to_vec(),
            FieldValue::Bytes(val) => val.to_vec(),
            FieldValue::Enum(val) => value_encoder!(val, write_u64, 8),
        }
    }

//    #[inline(always)]
//    fn field_from_key(&self, id: &str) ->

    #[inline(always)]
    fn object_key_prefix(&self, id: u64) -> String {
        format!(
            "{database}{delimiter}{collection}{delimiter}{id}",
            database = self.database,
            delimiter = KEY_DELIMITER,
            collection = self.name,
            id = id,
        )
    }

    #[inline(always)]
    fn field_key(&self, id: u64, tag: i32) -> String {
        format!(
            "{prefix}{delimiter}{tag}",
            prefix = self.object_key_prefix(id),
            delimiter = KEY_DELIMITER,
            tag = tag,
        )
    }
}

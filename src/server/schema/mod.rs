mod decode_field;
mod decode_object;
mod encode_field;
mod errors;
mod result;

pub use self::decode_field::DecodeField;
pub use self::decode_object::{DecodeObject, ObjectDecoder};
pub use self::encode_field::EncodeField;
pub use self::errors::SchemaError;
pub use self::result::Result;

use super::util;

use ::protobuf::descriptor;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Schema {
    descriptor: descriptor::DescriptorProto,
    field_types: HashMap<u32, descriptor::FieldDescriptorProto_Type>,
    pub id_field_number: u32,
    pub num_fields: usize,
}

impl Schema {
    pub fn new(descriptor: descriptor::DescriptorProto) -> Result<Schema> {
        let id_field_number = try!(descriptor.get_field()
            .iter()
            .find(|ref field| field.get_name() == "_id")
            .ok_or(SchemaError::MissingIdField)
            .map(|field| field.get_number() as u32));

        let field_types: HashMap<u32, descriptor::FieldDescriptorProto_Type> = descriptor.get_field()
            .iter()
            .map(|field| (field.get_number() as u32, field.get_field_type()))
            .collect();

        let num_fields = field_types.len();

        Ok(Schema {
            descriptor: descriptor,
            field_types: field_types,
            id_field_number: id_field_number,
            num_fields: num_fields,
        })
    }

    pub fn encode_tag(&self,
                      field_number: u32,
                      output_stream: &mut ::protobuf::CodedOutputStream)
                      -> Result<()> {
        let data_type = try!(self.field_types.get(&field_number).ok_or(SchemaError::EncodeError));
        try!(output_stream.write_tag(field_number, util::protobuf::wire_type(data_type)));
        output_stream.flush().map_err(SchemaError::ProtobufError)
    }
}

#[cfg(test)]
mod tests {
    extern crate protobuf;

    use super::*;
    use server::util;
    use server::protos::test;

    #[test]
    fn test_decode() {
        let descriptor =
            util::protobuf::descriptor_from_file_descriptor(test::file_descriptor_proto(),
                                                            "ScalarSchema")
                .expect("Could not find descriptor");

        let schema = Schema::new(descriptor).unwrap();
        let mut data: &[u8] = &[112, 1];
        schema.decode_object(&mut data);
    }
}

use super::{Result, Schema, SchemaError};

use ::protobuf::descriptor;

use std::collections::HashMap;

pub trait DecodeObject {
    fn decode_object<'a>(&'a self, data: &'a mut &'a [u8]) -> ObjectDecoder;
}

impl DecodeObject for Schema {
    fn decode_object<'a>(&'a self, data: &'a mut &'a [u8]) -> ObjectDecoder {
        ObjectDecoder::new(data, &self.field_types)
    }
}

pub struct ObjectDecoder<'a> {
    input_stream: ::protobuf::CodedInputStream<'a>,
    buffer: Vec<u8>,
    field_types: &'a HashMap<u32, descriptor::FieldDescriptorProto_Type>,
}

impl<'a> ObjectDecoder<'a> {
    pub fn new(data: &'a mut &'a [u8],
               field_types: &'a HashMap<u32, descriptor::FieldDescriptorProto_Type>)
               -> ObjectDecoder<'a> {
        ObjectDecoder {
            input_stream: ::protobuf::CodedInputStream::new(data),
            buffer: Vec::new(),
            field_types: field_types,
        }
    }

    fn decode_field_into(&mut self, field_number: u32) -> Result<()> {
        let data_type = try!(self.field_types.get(&field_number).ok_or(SchemaError::DecodeError));

        self.buffer.clear();
        let mut target = ::protobuf::CodedOutputStream::new(&mut self.buffer);

        // TODO: can this be refactored?
        match data_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_BOOL => {
                let val = try!(self.input_stream.read_bool());
                try!(target.write_bool_no_tag(val));
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_UINT32 => {
                let val = try!(self.input_stream.read_uint32());
                try!(target.write_uint32_no_tag(val));
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_FIXED32 => {
                let val = try!(self.input_stream.read_fixed32());
                try!(target.write_fixed32_no_tag(val));
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_INT32 => {
                let val = try!(self.input_stream.read_int32());
                try!(target.write_int32_no_tag(val));
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_SINT32 => {
                let val = try!(self.input_stream.read_sint32());
                try!(target.write_sint32_no_tag(val));
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_SFIXED32 => {
                let val = try!(self.input_stream.read_sfixed32());
                try!(target.write_sfixed32_no_tag(val));
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_FLOAT => {
                let val = try!(self.input_stream.read_float());
                try!(target.write_float_no_tag(val));
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_UINT64 |
            &descriptor::FieldDescriptorProto_Type::TYPE_ENUM => {
                let val = try!(self.input_stream.read_uint64());
                try!(target.write_uint64_no_tag(val));
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_FIXED64 => {
                let val = try!(self.input_stream.read_fixed64());
                try!(target.write_fixed64_no_tag(val));
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_INT64 => {
                let val = try!(self.input_stream.read_int64());
                try!(target.write_int64_no_tag(val));
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_SINT64 => {
                let val = try!(self.input_stream.read_sint64());
                try!(target.write_sint64_no_tag(val));
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_SFIXED64 => {
                let val = try!(self.input_stream.read_sfixed64());
                try!(target.write_sfixed64_no_tag(val));
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_DOUBLE => {
                let val = try!(self.input_stream.read_double());
                try!(target.write_double_no_tag(val));
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_BYTES |
            &descriptor::FieldDescriptorProto_Type::TYPE_STRING |
            &descriptor::FieldDescriptorProto_Type::TYPE_GROUP |
            &descriptor::FieldDescriptorProto_Type::TYPE_MESSAGE => {
                let val = try!(self.input_stream.read_bytes());
                try!(target.write_bytes_no_tag(&val));
            }
        }

        target.flush().map_err(SchemaError::ProtobufError)
    }
}

impl<'a> Iterator for ObjectDecoder<'a> {
    type Item = Result<(u32, Vec<u8>)>;

    fn next(&mut self) -> Option<Result<(u32, Vec<u8>)>> {
        self.input_stream
            .read_tag_unpack()
            .map(|(field_number, _)| {
                try!(self.decode_field_into(field_number));
                Ok((field_number, self.buffer.clone()))
            })
            .ok()
    }
}

use super::{Result, Schema, SchemaError};

use ::protobuf::descriptor;

pub trait EncodeField<T> {
    fn encode_field(&self,
                    u32,
                    T,
                    &mut ::protobuf::CodedOutputStream)
                    -> Result<()>;

    fn encode_field_with_tag(&self,
                             u32,
                             T,
                             &mut ::protobuf::CodedOutputStream)
                             -> Result<()>;
}

impl EncodeField<bool> for Schema {
    fn encode_field(&self,
                    field_number: u32,
                    value: bool,
                    output_stream: &mut ::protobuf::CodedOutputStream)
                    -> Result<()> {
        let field_type = try!(self.field_types.get(&field_number).ok_or(SchemaError::EncodeError));
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_BOOL => {
                try!(output_stream.write_bool_no_tag(value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::EncodeError),
        }
    }

    fn encode_field_with_tag(&self,
                             field_number: u32,
                             value: bool,
                             output_stream: &mut ::protobuf::CodedOutputStream)
                             -> Result<()> {
        let field_type = try!(self.field_types.get(&field_number).ok_or(SchemaError::EncodeError));
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_BOOL => {
                try!(output_stream.write_bool(field_number, value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::EncodeError),
        }
    }
}

impl EncodeField<u32> for Schema {
    fn encode_field(&self,
                    field_number: u32,
                    value: u32,
                    output_stream: &mut ::protobuf::CodedOutputStream)
                    -> Result<()> {
        let field_type = try!(self.field_types.get(&field_number).ok_or(SchemaError::EncodeError));
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_UINT32 => {
                try!(output_stream.write_uint32_no_tag(value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_FIXED32 => {
                try!(output_stream.write_fixed32_no_tag(value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::EncodeError),
        }
    }

    fn encode_field_with_tag(&self,
                             field_number: u32,
                             value: u32,
                             output_stream: &mut ::protobuf::CodedOutputStream)
                             -> Result<()> {
        let field_type = try!(self.field_types.get(&field_number).ok_or(SchemaError::EncodeError));
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_UINT32 => {
                try!(output_stream.write_uint32(field_number, value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_FIXED32 => {
                try!(output_stream.write_fixed32(field_number, value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::EncodeError),
        }
    }
}
impl EncodeField<i32> for Schema {
    fn encode_field(&self,
                    field_number: u32,
                    value: i32,
                    output_stream: &mut ::protobuf::CodedOutputStream)
                    -> Result<()> {
        let field_type = try!(self.field_types.get(&field_number).ok_or(SchemaError::EncodeError));
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_INT32 => {
                try!(output_stream.write_int32_no_tag(value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_SINT32 => {
                try!(output_stream.write_sint32_no_tag(value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_SFIXED32 => {
                try!(output_stream.write_sfixed32_no_tag(value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::EncodeError),
        }
    }

    fn encode_field_with_tag(&self,
                             field_number: u32,
                             value: i32,
                             output_stream: &mut ::protobuf::CodedOutputStream)
                             -> Result<()> {
        let field_type = try!(self.field_types.get(&field_number).ok_or(SchemaError::EncodeError));
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_INT32 => {
                try!(output_stream.write_int32(field_number, value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_SINT32 => {
                try!(output_stream.write_sint32(field_number, value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_SFIXED32 => {
                try!(output_stream.write_sfixed32(field_number, value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::EncodeError),
        }
    }
}

impl EncodeField<f32> for Schema {
    fn encode_field(&self,
                    field_number: u32,
                    value: f32,
                    output_stream: &mut ::protobuf::CodedOutputStream)
                    -> Result<()> {
        let field_type = try!(self.field_types.get(&field_number).ok_or(SchemaError::EncodeError));
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_FLOAT => {
                try!(output_stream.write_float_no_tag(value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::EncodeError),
        }
    }

    fn encode_field_with_tag(&self,
                             field_number: u32,
                             value: f32,
                             output_stream: &mut ::protobuf::CodedOutputStream)
                             -> Result<()> {
        let field_type = try!(self.field_types.get(&field_number).ok_or(SchemaError::EncodeError));
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_FLOAT => {
                try!(output_stream.write_float(field_number, value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::EncodeError),
        }
    }
}

impl EncodeField<u64> for Schema {
    fn encode_field(&self,
                    field_number: u32,
                    value: u64,
                    output_stream: &mut ::protobuf::CodedOutputStream)
                    -> Result<()> {
        let field_type = try!(self.field_types.get(&field_number).ok_or(SchemaError::EncodeError));
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_UINT64 |
            &descriptor::FieldDescriptorProto_Type::TYPE_ENUM => {
                try!(output_stream.write_uint64_no_tag(value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_FIXED64 => {
                try!(output_stream.write_fixed64_no_tag(value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::EncodeError),
        }
    }

    fn encode_field_with_tag(&self,
                             field_number: u32,
                             value: u64,
                             output_stream: &mut ::protobuf::CodedOutputStream)
                             -> Result<()> {
        let field_type = try!(self.field_types.get(&field_number).ok_or(SchemaError::EncodeError));
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_UINT64 |
            &descriptor::FieldDescriptorProto_Type::TYPE_ENUM => {
                try!(output_stream.write_uint64(field_number, value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_FIXED64 => {
                try!(output_stream.write_fixed64(field_number, value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::EncodeError),
        }
    }
}

impl EncodeField<i64> for Schema {
    fn encode_field(&self,
                    field_number: u32,
                    value: i64,
                    output_stream: &mut ::protobuf::CodedOutputStream)
                    -> Result<()> {
        let field_type = try!(self.field_types.get(&field_number).ok_or(SchemaError::EncodeError));
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_INT64 => {
                try!(output_stream.write_int64_no_tag(value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_SINT64 => {
                try!(output_stream.write_sint64_no_tag(value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_SFIXED64 => {
                try!(output_stream.write_sfixed64_no_tag(value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::EncodeError),
        }
    }

    fn encode_field_with_tag(&self,
                             field_number: u32,
                             value: i64,
                             output_stream: &mut ::protobuf::CodedOutputStream)
                             -> Result<()> {
        let field_type = try!(self.field_types.get(&field_number).ok_or(SchemaError::EncodeError));
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_INT64 => {
                try!(output_stream.write_int64(field_number, value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_SINT64 => {
                try!(output_stream.write_sint64(field_number, value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_SFIXED64 => {
                try!(output_stream.write_sfixed64(field_number, value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::EncodeError),
        }
    }
}

impl EncodeField<f64> for Schema {
    fn encode_field(&self,
                    field_number: u32,
                    value: f64,
                    output_stream: &mut ::protobuf::CodedOutputStream)
                    -> Result<()> {
        let field_type = try!(self.field_types.get(&field_number).ok_or(SchemaError::EncodeError));
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_DOUBLE => {
                try!(output_stream.write_double_no_tag(value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::EncodeError),
        }
    }

    fn encode_field_with_tag(&self,
                             field_number: u32,
                             value: f64,
                             output_stream: &mut ::protobuf::CodedOutputStream)
                             -> Result<()> {
        let field_type = try!(self.field_types.get(&field_number).ok_or(SchemaError::EncodeError));
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_DOUBLE => {
                try!(output_stream.write_double(field_number, value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::EncodeError),
        }
    }
}

impl<'a> EncodeField<&'a [u8]> for Schema {
    fn encode_field(&self,
                    field_number: u32,
                    value: &[u8],
                    output_stream: &mut ::protobuf::CodedOutputStream)
                    -> Result<()> {
        let field_type = try!(self.field_types.get(&field_number).ok_or(SchemaError::EncodeError));
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_BYTES |
            &descriptor::FieldDescriptorProto_Type::TYPE_STRING |
            &descriptor::FieldDescriptorProto_Type::TYPE_GROUP |
            &descriptor::FieldDescriptorProto_Type::TYPE_MESSAGE => {
                try!(output_stream.write_bytes_no_tag(value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::EncodeError),
        }
    }

    fn encode_field_with_tag(&self,
                             field_number: u32,
                             value: &[u8],
                             output_stream: &mut ::protobuf::CodedOutputStream)
                             -> Result<()> {
        let field_type = try!(self.field_types.get(&field_number).ok_or(SchemaError::EncodeError));
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_BYTES |
            &descriptor::FieldDescriptorProto_Type::TYPE_STRING |
            &descriptor::FieldDescriptorProto_Type::TYPE_GROUP |
            &descriptor::FieldDescriptorProto_Type::TYPE_MESSAGE => {
                try!(output_stream.write_bytes(field_number, value));
                output_stream.flush().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::EncodeError),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate protobuf;

    use super::*;
    use server::util;
    use server::schema;
    use server::protos::test;

    #[test]
    fn test_encode_bool() {
        let descriptor =
            util::protobuf::descriptor_from_file_descriptor(test::file_descriptor_proto(),
                                                            "ScalarSchema")
                .expect("Could not find descriptor");

        let schema = schema::Schema::new(descriptor).unwrap();

        {
            let mut output_vec = vec![];
            {
                let mut output = ::protobuf::CodedOutputStream::new(&mut output_vec);
                <schema::Schema as EncodeField<bool>>::encode_field(&schema,
                                          14,
                                          false,
                                          &mut output)
                    .unwrap();
            }

            assert_eq!(output_vec, [0]);
        }

        {
            let mut output_vec = vec![];
            {
                let mut output = ::protobuf::CodedOutputStream::new(&mut output_vec);
                <schema::Schema as EncodeField<bool>>::encode_field(&schema,
                                          14,
                                          true,
                                          &mut output)
                    .unwrap();
            }

            assert_eq!(output_vec, [1]);
        }
    }

    #[test]
    fn test_encode_u64() {
        let descriptor =
            util::protobuf::descriptor_from_file_descriptor(test::file_descriptor_proto(),
                                                            "ScalarSchema")
                .expect("Could not find descriptor");

        let schema = schema::Schema::new(descriptor).unwrap();

        {
            let mut output_vec = vec![];
            {
                let mut output = ::protobuf::CodedOutputStream::new(&mut output_vec);
                <schema::Schema as EncodeField<u64>>::encode_field(&schema,
                                          7,
                                          1u64,
                                          &mut output)
                    .unwrap();
            }

            assert_eq!(output_vec, [1]);
        }

        // {
        //     let mut output_vec = vec![];
        //     {
        //         let mut output = ::protobuf::CodedOutputStream::new(&mut output_vec);
        //         <schema::Schema as EncodeField<bool>>::encode_field(&schema,
        //                                   true,
        //                                   &descriptor::FieldDescriptorProto_Type::TYPE_BOOL,
        //                                   &mut output)
        //             .unwrap();
        //         output.flush().unwrap();
        //     }
        //
        //     assert_eq!(output_vec, [1]);
        // }
    }
}

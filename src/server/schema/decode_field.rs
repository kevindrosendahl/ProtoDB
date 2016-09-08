use super::{Result, Schema, SchemaError};

use ::protobuf::descriptor;

pub trait DecodeField<T> {
    fn decode_field(&self,
                    &descriptor::FieldDescriptorProto_Type,
                    &mut ::protobuf::CodedInputStream)
                    -> Result<T>;
}

impl DecodeField<bool> for Schema {
    fn decode_field(&self,
                    field_type: &descriptor::FieldDescriptorProto_Type,
                    input_stream: &mut ::protobuf::CodedInputStream)
                    -> Result<bool> {
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_BOOL => {
                input_stream.read_bool().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::DecodeError),
        }
    }
}

impl DecodeField<u32> for Schema {
    fn decode_field(&self,
                    field_type: &descriptor::FieldDescriptorProto_Type,
                    input_stream: &mut ::protobuf::CodedInputStream)
                    -> Result<u32> {
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_UINT32 => {
                input_stream.read_uint32().map_err(SchemaError::ProtobufError)
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_FIXED32 => {
                input_stream.read_fixed32().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::DecodeError),
        }
    }
}

impl DecodeField<i32> for Schema {
    fn decode_field(&self,
                    field_type: &descriptor::FieldDescriptorProto_Type,
                    input_stream: &mut ::protobuf::CodedInputStream)
                    -> Result<i32> {
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_INT32 => {
                input_stream.read_int32().map_err(SchemaError::ProtobufError)
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_SINT32 => {
                input_stream.read_sint32().map_err(SchemaError::ProtobufError)
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_SFIXED32 => {
                input_stream.read_sfixed32().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::DecodeError),
        }
    }
}

impl DecodeField<f32> for Schema {
    fn decode_field(&self,
                    field_type: &descriptor::FieldDescriptorProto_Type,
                    input_stream: &mut ::protobuf::CodedInputStream)
                    -> Result<f32> {
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_FLOAT => {
                input_stream.read_float().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::DecodeError),
        }
    }
}

impl DecodeField<u64> for Schema {
    fn decode_field(&self,
                    field_type: &descriptor::FieldDescriptorProto_Type,
                    input_stream: &mut ::protobuf::CodedInputStream)
                    -> Result<u64> {
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_UINT64 |
            &descriptor::FieldDescriptorProto_Type::TYPE_ENUM => {
                input_stream.read_uint64().map_err(SchemaError::ProtobufError)
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_FIXED64 => {
                input_stream.read_fixed64().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::DecodeError),
        }
    }
}

impl DecodeField<i64> for Schema {
    fn decode_field(&self,
                    field_type: &descriptor::FieldDescriptorProto_Type,
                    input_stream: &mut ::protobuf::CodedInputStream)
                    -> Result<i64> {
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_INT64 => {
                input_stream.read_int64().map_err(SchemaError::ProtobufError)
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_SINT64 => {
                input_stream.read_sint64().map_err(SchemaError::ProtobufError)
            }

            &descriptor::FieldDescriptorProto_Type::TYPE_SFIXED64 => {
                input_stream.read_sfixed64().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::DecodeError),
        }
    }
}

impl DecodeField<f64> for Schema {
    fn decode_field(&self,
                    field_type: &descriptor::FieldDescriptorProto_Type,
                    input_stream: &mut ::protobuf::CodedInputStream)
                    -> Result<f64> {
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_DOUBLE => {
                input_stream.read_double().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::DecodeError),
        }
    }
}

impl DecodeField<Vec<u8>> for Schema {
    fn decode_field(&self,
                    field_type: &descriptor::FieldDescriptorProto_Type,
                    input_stream: &mut ::protobuf::CodedInputStream)
                    -> Result<Vec<u8>> {
        match field_type {
            &descriptor::FieldDescriptorProto_Type::TYPE_BYTES |
            &descriptor::FieldDescriptorProto_Type::TYPE_STRING |
            &descriptor::FieldDescriptorProto_Type::TYPE_GROUP |
            &descriptor::FieldDescriptorProto_Type::TYPE_MESSAGE => {
                input_stream.read_bytes().map_err(SchemaError::ProtobufError)
            }

            _ => Err(SchemaError::DecodeError),
        }
    }
}

#[cfg(test)]
mod tests {
    use ::protobuf::descriptor;

    use super::*;
    use server::util;
    use server::schema;
    use server::protos::test;

    #[test]
    fn test_decode_bool() {
        let descriptor =
            util::protobuf::descriptor_from_file_descriptor(test::file_descriptor_proto(),
                                                            "ScalarSchema")
                .expect("Could not find descriptor");

        let schema = schema::Schema::new(descriptor).unwrap();

        {
            let mut slice: &[u8] = &[0];
            let mut output_stream = ::protobuf::CodedInputStream::new(&mut slice);
            let result = <schema::Schema as DecodeField<bool>>::decode_field(&schema,
                                      &descriptor::FieldDescriptorProto_Type::TYPE_BOOL,
                                      &mut output_stream)
                    .unwrap();

            assert!(!result);
        }

        {
            let mut slice: &[u8] = &[1];
            let mut output_stream = ::protobuf::CodedInputStream::new(&mut slice);
            let result = <schema::Schema as DecodeField<bool>>::decode_field(&schema,
                                      &descriptor::FieldDescriptorProto_Type::TYPE_BOOL,
                                      &mut output_stream)
                    .unwrap();

            assert!(result);
        }
    }
}

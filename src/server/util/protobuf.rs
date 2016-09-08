extern crate protobuf;

use self::protobuf::descriptor;
use self::protobuf::wire_format;

pub fn descriptor_from_file_descriptor(file_descriptor: &descriptor::FileDescriptorProto,
                                       descriptor_name: &str)
                                       -> Option<descriptor::DescriptorProto> {
    file_descriptor.get_message_type()
        .iter()
        .find(|ref descriptor| descriptor.get_name() == descriptor_name)
        .map(|descriptor| descriptor.clone())
}

pub fn wire_type(data_type: &descriptor::FieldDescriptorProto_Type) -> wire_format::WireType {
    match data_type {
        &descriptor::FieldDescriptorProto_Type::TYPE_INT32 |
        &descriptor::FieldDescriptorProto_Type::TYPE_UINT32 |
        &descriptor::FieldDescriptorProto_Type::TYPE_SINT32 |
        &descriptor::FieldDescriptorProto_Type::TYPE_INT64 |
        &descriptor::FieldDescriptorProto_Type::TYPE_UINT64 |
        &descriptor::FieldDescriptorProto_Type::TYPE_SINT64 |
        &descriptor::FieldDescriptorProto_Type::TYPE_BOOL |
        &descriptor::FieldDescriptorProto_Type::TYPE_ENUM => wire_format::WireType::WireTypeVarint,

        &descriptor::FieldDescriptorProto_Type::TYPE_FIXED64 |
        &descriptor::FieldDescriptorProto_Type::TYPE_SFIXED64 |
        &descriptor::FieldDescriptorProto_Type::TYPE_DOUBLE => {
            wire_format::WireType::WireTypeFixed64
        }

        &descriptor::FieldDescriptorProto_Type::TYPE_STRING |
        &descriptor::FieldDescriptorProto_Type::TYPE_BYTES |
        &descriptor::FieldDescriptorProto_Type::TYPE_GROUP |
        &descriptor::FieldDescriptorProto_Type::TYPE_MESSAGE => {
            wire_format::WireType::WireTypeLengthDelimited
        }

        &descriptor::FieldDescriptorProto_Type::TYPE_FIXED32 |
        &descriptor::FieldDescriptorProto_Type::TYPE_SFIXED32 |
        &descriptor::FieldDescriptorProto_Type::TYPE_FLOAT => {
            wire_format::WireType::WireTypeFixed32
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate protobuf;
    use self::protobuf::descriptor::FieldDescriptorProto_Type;

    use server::protos::test;
    use super::*;

    #[test]
    fn test_scalar_descriptor_proto_from_file_descriptor_proto() {
        let descriptor = descriptor_from_file_descriptor(test::file_descriptor_proto(),
                                                         "ScalarSchema")
            .expect("Could not find descriptor");

        assert_eq!(descriptor.get_field().len(), 16);

        struct FieldInfo(&'static str, FieldDescriptorProto_Type);
        let expected_info = vec![
            FieldInfo("_id", FieldDescriptorProto_Type::TYPE_UINT64),
            FieldInfo("double_field", FieldDescriptorProto_Type::TYPE_DOUBLE),
            FieldInfo("float_field", FieldDescriptorProto_Type::TYPE_FLOAT),
            FieldInfo("int32_field", FieldDescriptorProto_Type::TYPE_INT32),
            FieldInfo("int64_field", FieldDescriptorProto_Type::TYPE_INT64),
            FieldInfo("uint32_field", FieldDescriptorProto_Type::TYPE_UINT32),
            FieldInfo("uint64_field", FieldDescriptorProto_Type::TYPE_UINT64),
            FieldInfo("sint32_field", FieldDescriptorProto_Type::TYPE_SINT32),
            FieldInfo("sint64_field", FieldDescriptorProto_Type::TYPE_SINT64),
            FieldInfo("fixed32_field", FieldDescriptorProto_Type::TYPE_FIXED32),
            FieldInfo("fixed64_field", FieldDescriptorProto_Type::TYPE_FIXED64),
            FieldInfo("sfixed32_field", FieldDescriptorProto_Type::TYPE_SFIXED32),
            FieldInfo("sfixed64_field", FieldDescriptorProto_Type::TYPE_SFIXED64),
            FieldInfo("bool_field", FieldDescriptorProto_Type::TYPE_BOOL),
            FieldInfo("string_field", FieldDescriptorProto_Type::TYPE_STRING),
            FieldInfo("bytes_field", FieldDescriptorProto_Type::TYPE_BYTES),
        ];

        assert_eq!(descriptor.get_field().len(), expected_info.len());

        for field in descriptor.get_field() {
            match field.get_number() {
                n if {
                    let info = expected_info.get((n - 1) as usize).unwrap();
                    field.get_name() == info.0 && field.get_field_type() == info.1
                } => (),
                _ => panic!("Unexpected field {:?}", field),
            }
        }
    }

    #[test]
    fn test_repeated_descriptor_proto_from_file_descriptor_proto() {
        let descriptor = descriptor_from_file_descriptor(test::file_descriptor_proto(),
                                                         "RepeatedScalarSchema")
            .expect("Could not find descriptor");

        assert_eq!(descriptor.get_field().len(), 16);

        struct FieldInfo(&'static str, FieldDescriptorProto_Type);
        let expected_info = vec![
            FieldInfo("_id", FieldDescriptorProto_Type::TYPE_UINT64),
            FieldInfo("repeated_double_field", FieldDescriptorProto_Type::TYPE_DOUBLE),
            FieldInfo("repeated_float_field", FieldDescriptorProto_Type::TYPE_FLOAT),
            FieldInfo("repeated_int32_field", FieldDescriptorProto_Type::TYPE_INT32),
            FieldInfo("repeated_int64_field", FieldDescriptorProto_Type::TYPE_INT64),
            FieldInfo("repeated_uint32_field", FieldDescriptorProto_Type::TYPE_UINT32),
            FieldInfo("repeated_uint64_field", FieldDescriptorProto_Type::TYPE_UINT64),
            FieldInfo("repeated_sint32_field", FieldDescriptorProto_Type::TYPE_SINT32),
            FieldInfo("repeated_sint64_field", FieldDescriptorProto_Type::TYPE_SINT64),
            FieldInfo("repeated_fixed32_field", FieldDescriptorProto_Type::TYPE_FIXED32),
            FieldInfo("repeated_fixed64_field", FieldDescriptorProto_Type::TYPE_FIXED64),
            FieldInfo("repeated_sfixed32_field", FieldDescriptorProto_Type::TYPE_SFIXED32),
            FieldInfo("repeated_sfixed64_field", FieldDescriptorProto_Type::TYPE_SFIXED64),
            FieldInfo("repeated_bool_field", FieldDescriptorProto_Type::TYPE_BOOL),
            FieldInfo("repeated_string_field", FieldDescriptorProto_Type::TYPE_STRING),
            FieldInfo("repeated_bytes_field", FieldDescriptorProto_Type::TYPE_BYTES),
        ];

        assert_eq!(descriptor.get_field().len(), expected_info.len());

        for field in descriptor.get_field() {
            match field.get_number() {
                n if {
                    let info = expected_info.get((n - 1) as usize).unwrap();
                    field.get_name() == info.0 && field.get_field_type() == info.1
                } => (),
                _ => panic!("Unexpected field {:?}", field),
            }
        }
    }
}

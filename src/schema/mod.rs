use std::collections::HashMap;

use prost_types::{
    field_descriptor_proto::{Label, Type},
    DescriptorProto,
};

pub mod encoding;
pub mod errors;

const ID_FIELD: &str = "id";

#[derive(Clone, Debug)]
pub struct Schema {
    pub descriptor: DescriptorProto,
    pub id_field: i32,
    pub fields: HashMap<i32, (Label, Type)>,
}

impl Schema {
    pub fn new(descriptor: &DescriptorProto) -> Result<Schema, errors::SchemaError> {
        let mut id_field = Err(errors::SchemaError::MissingIdField);
        let mut fields = HashMap::new();
        for field in descriptor.field.iter() {
            // check for invalid labels we currently don't support:
            //   - required (deprecated in proto3)
            //   - repeated
            match field.label() {
                Label::Optional => Ok(()),
                _ => Err(errors::SchemaError::InvalidFieldType((
                    field.number(),
                    field.label(),
                    field.type_(),
                ))),
            }?;

            // check for invalid types we currently don't support:
            //    - group (deprecated in proto3)
            //    - message
            match field.type_() {
                Type::Group | Type::Message => Err(errors::SchemaError::InvalidFieldType((
                    field.number(),
                    field.label(),
                    field.type_(),
                ))),
                _ => Ok(()),
            }?;

            if field.name() == ID_FIELD {
                id_field = Ok(field.number());
            }

            fields.insert(field.number(), (field.label(), field.type_()));
        }

        let id_field = id_field?;
        Ok(Schema {
            descriptor: descriptor.clone(),
            id_field,
            fields,
        })
    }
}

use std::collections::HashMap;

use prost_types::{
    field_descriptor_proto::{Label, Type},
    DescriptorProto,
};

pub mod encoding;
pub mod errors;

pub use self::encoding::{DecodedIdObject, DecodedObjectBuilder};

const ID_FIELD: &str = "id";

#[derive(Clone, Debug)]
pub struct Schema {
    pub descriptor: DescriptorProto,
    pub id_field: i32,
    pub fields: DescriptorFields,
}

impl Schema {
    pub fn new(descriptor: &DescriptorProto) -> Result<Schema, errors::SchemaError> {
        let (fields, id_field) = descriptor_fields(descriptor)?;
        let id_field = match id_field {
            Some(id_field) => Ok(id_field),
            None => Err(errors::SchemaError::MissingIdField),
        }?;

        Ok(Schema {
            descriptor: descriptor.clone(),
            id_field,
            fields,
        })
    }
}

#[derive(Clone, Debug)]
pub struct DescriptorFields {
    inner: HashMap<i32, (String, Label, Type)>,
}

impl DescriptorFields {
    pub fn info(&self, field: &i32) -> Option<&(String, Label, Type)> {
        self.inner.get(field)
    }
}

pub fn descriptor_fields(descriptor: &DescriptorProto) -> Result<(DescriptorFields, Option<i32>), errors::SchemaError> {
    let mut id_field = None;
    let mut fields = HashMap::new();
    for field in descriptor.field.iter() {
        // check for invalid labels we currently don't support:
        //   - required (deprecated in proto3)
        //   - repeated
        match field.label() {
            Label::Optional => Ok(()),
            _ => Err(errors::SchemaError::InvalidFieldType((
                field.number(),
                field.name().to_string(),
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
                field.name().to_string(),
                field.label(),
                field.type_(),
            ))),
            _ => Ok(()),
        }?;

        if field.name() == ID_FIELD {
            id_field = Some(field.number());
        }

        fields.insert(
            field.number(),
            (field.name().to_string(), field.label(), field.type_()),
        );
    }

    Ok((DescriptorFields { inner: fields }, id_field))
}
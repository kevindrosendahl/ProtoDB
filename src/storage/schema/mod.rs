use prost_types::DescriptorProto;

pub mod errors;

pub struct Schema {
    pub descriptor: DescriptorProto,
    pub id_field: u32,
}

impl Schema {
    pub fn new(descriptor: DescriptorProto) -> Result<Schema, errors::SchemaError> {
        let id_field = descriptor
            .field
            .iter()
            .find(|field| field.name() == "id")
            .ok_or(errors::SchemaError::MissingIdField)
            .map(|field| field.number() as u32)?;

        Ok(Schema {
            descriptor,
            id_field,
        })
    }
}

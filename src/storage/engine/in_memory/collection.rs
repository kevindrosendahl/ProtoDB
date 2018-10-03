use super::cache::Cache;
use crate::storage::{
    errors,
    schema::{errors::SchemaError, Schema},
};

use prost_types::DescriptorProto;

pub(crate) struct Collection {
    pub schema: Schema,
    cache: Cache,
}

impl Collection {
    pub fn new(descriptor: &DescriptorProto) -> Result<Collection, SchemaError> {
        let schema = Schema::new(descriptor)?;
        Ok(Collection {
            schema,
            cache: Default::default(),
        })
    }

    pub fn insert_object(&self, object: &[u8]) -> Result<(), errors::InsertObjectError> {
        self.schema.decode_object(object);
        Ok(())
    }
}
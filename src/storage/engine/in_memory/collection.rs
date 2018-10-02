use std::{
    collections::BTreeMap,
    ops::Bound,
    sync::{Arc, RwLock},
};

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

    fn insert_object(&self, object: &[u8]) -> Result<(), errors::InsertError> {
        Ok(())
    }
}

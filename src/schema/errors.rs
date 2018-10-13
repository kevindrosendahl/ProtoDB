use std::{error, fmt};

use prost_types::field_descriptor_proto::{Label, Type};

#[derive(Debug)]
pub enum SchemaError {
    EncodingError(String),
    InvalidFieldType((i32, Label, Type)),
    InvalidIdType(String),
    MissingIdField,
}

impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for SchemaError {}

#[derive(Debug)]
pub enum ObjectError {
    ProstDecodeError(prost::DecodeError),
    SchemaError(SchemaError),
}

impl fmt::Display for ObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for ObjectError {}

impl From<prost::DecodeError> for ObjectError {
    fn from(err: prost::DecodeError) -> ObjectError {
        ObjectError::ProstDecodeError(err)
    }
}

impl From<SchemaError> for ObjectError {
    fn from(err: SchemaError) -> ObjectError {
        ObjectError::SchemaError(err)
    }
}

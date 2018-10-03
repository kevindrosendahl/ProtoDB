use std::{error, fmt};

use prost_types::{
    field_descriptor_proto::{Label, Type},
    DescriptorProto,
};

#[derive(Debug)]
pub enum SchemaError {
    InvalidFieldType((i32, Label, Type)),
    InvalidIdType(String),
    MissingIdField,
}

impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for SchemaError {
    fn description(&self) -> &str {
        let str = match self {
            SchemaError::InvalidFieldType(_) => "invalid field type",
            SchemaError::InvalidIdType(_) => "invalid id type",
            SchemaError::MissingIdField => "missing id field",
        };
        &str
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            SchemaError::InvalidFieldType(_) => None,
            SchemaError::InvalidIdType(_) => None,
            SchemaError::MissingIdField => None,
        }
    }
}

#[derive(Debug)]
pub enum ObjectError {
    DecodeError(prost::DecodeError),
}

impl fmt::Display for ObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for ObjectError {
    fn description(&self) -> &str {
        match self {
            ObjectError::DecodeError(err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            ObjectError::DecodeError(err) => err.cause(),
        }
    }
}

impl From<prost::DecodeError> for ObjectError {
    fn from(err: prost::DecodeError) -> ObjectError {
        ObjectError::DecodeError(err)
    }
}

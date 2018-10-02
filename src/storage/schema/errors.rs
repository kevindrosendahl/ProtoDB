use std::{error, fmt};

#[derive(Debug)]
pub enum SchemaError {
    InvalidIdType,
    MissingIdField,
}

impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for SchemaError {
    fn description(&self) -> &str {
        match self {
            SchemaError::InvalidIdType => "invalid id type",
            SchemaError::MissingIdField => "missing id field",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            SchemaError::InvalidIdType => None,
            SchemaError::MissingIdField => None,
        }
    }
}

#[derive(Debug)]
pub enum ObjectError {
    MalformedData,
}

impl fmt::Display for ObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for ObjectError {
    fn description(&self) -> &str {
        match self {
            ObjectError::MalformedData => "malformed data",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            ObjectError::MalformedData => None,
        }
    }
}

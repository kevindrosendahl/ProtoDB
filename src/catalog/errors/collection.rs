use std::{error, fmt};

use crate::schema::errors::ObjectError;

#[derive(Debug)]
pub enum InsertObjectError {
    ObjectExists,
    ObjectError(ObjectError),
}

impl fmt::Display for InsertObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for InsertObjectError {
    fn description(&self) -> &str {
        match self {
            InsertObjectError::ObjectExists => "object exists",
            InsertObjectError::ObjectError(err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            InsertObjectError::ObjectExists => None,
            InsertObjectError::ObjectError(err) => err.cause(),
        }
    }
}

impl From<ObjectError> for InsertObjectError {
    fn from(err: ObjectError) -> InsertObjectError {
        InsertObjectError::ObjectError(err)
    }
}

#[derive(Debug)]
pub enum FindObjectError {
    ObjectError(ObjectError),
}

impl fmt::Display for FindObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for FindObjectError {
    fn description(&self) -> &str {
        match self {
            FindObjectError::ObjectError(err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            FindObjectError::ObjectError(err) => err.cause(),
        }
    }
}

impl From<ObjectError> for FindObjectError {
    fn from(err: ObjectError) -> FindObjectError {
        FindObjectError::ObjectError(err)
    }
}

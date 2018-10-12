use std::{error, fmt};

use crate::{schema::errors::ObjectError, storage::errors::InternalStorageEngineError};

#[derive(Debug)]
pub enum InsertObjectError {
    InternalStorageEngineError(InternalStorageEngineError),
    ObjectExists,
    ObjectError(ObjectError),
}

impl fmt::Display for InsertObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for InsertObjectError {}

impl From<ObjectError> for InsertObjectError {
    fn from(err: ObjectError) -> InsertObjectError {
        InsertObjectError::ObjectError(err)
    }
}

impl From<InternalStorageEngineError> for InsertObjectError {
    fn from(err: InternalStorageEngineError) -> InsertObjectError {
        InsertObjectError::InternalStorageEngineError(err)
    }
}

#[derive(Debug)]
pub enum FindObjectError {
    InternalStorageEngineError(InternalStorageEngineError),
    ObjectError(ObjectError),
}

impl fmt::Display for FindObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for FindObjectError {}

impl From<ObjectError> for FindObjectError {
    fn from(err: ObjectError) -> FindObjectError {
        FindObjectError::ObjectError(err)
    }
}

impl From<InternalStorageEngineError> for FindObjectError {
    fn from(err: InternalStorageEngineError) -> FindObjectError {
        FindObjectError::InternalStorageEngineError(err)
    }
}

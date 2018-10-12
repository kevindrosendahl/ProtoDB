use std::{error, fmt};

use crate::{schema::errors::SchemaError, storage::errors::InternalStorageEngineError};

#[derive(Debug)]
pub enum CreateDatabaseError {
    DatabaseExists,
    InternalStorageEngineError(InternalStorageEngineError),
}

impl fmt::Display for CreateDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for CreateDatabaseError {}

#[derive(Debug)]
pub enum CreateCollectionError {
    CollectionExists,
    InternalStorageEngineError(InternalStorageEngineError),
    SchemaError(SchemaError),
}

impl fmt::Display for CreateCollectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for CreateCollectionError {}

impl From<InternalStorageEngineError> for CreateCollectionError {
    fn from(err: InternalStorageEngineError) -> CreateCollectionError {
        CreateCollectionError::InternalStorageEngineError(err)
    }
}

impl From<SchemaError> for CreateCollectionError {
    fn from(err: SchemaError) -> CreateCollectionError {
        CreateCollectionError::SchemaError(err)
    }
}

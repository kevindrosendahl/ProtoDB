use std::{error, fmt};

use crate::schema::errors::SchemaError;

#[derive(Debug)]
pub enum CreateDatabaseError {
    DatabaseExists,
}

impl fmt::Display for CreateDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for CreateDatabaseError {
    fn description(&self) -> &str {
        match self {
            CreateDatabaseError::DatabaseExists => "database already exists",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            CreateDatabaseError::DatabaseExists => None,
        }
    }
}

#[derive(Debug)]
pub enum CreateCollectionError {
    CollectionExists,
    SchemaError(SchemaError),
}

impl fmt::Display for CreateCollectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for CreateCollectionError {
    fn description(&self) -> &str {
        match self {
            CreateCollectionError::CollectionExists => "collection exists",
            CreateCollectionError::SchemaError(err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            CreateCollectionError::CollectionExists => None,
            CreateCollectionError::SchemaError(err) => err.cause(),
        }
    }
}

impl From<SchemaError> for CreateCollectionError {
    fn from(err: SchemaError) -> CreateCollectionError {
        CreateCollectionError::SchemaError(err)
    }
}

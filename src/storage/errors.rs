use std::{error, fmt};

use super::schema::errors::{ObjectError, SchemaError};

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
pub enum ListCollectionsError {
    InvalidDatabase,
}

impl fmt::Display for ListCollectionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for ListCollectionsError {
    fn description(&self) -> &str {
        match self {
            ListCollectionsError::InvalidDatabase => "invald database",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            ListCollectionsError::InvalidDatabase => None,
        }
    }
}

#[derive(Debug)]
pub enum CreateCollectionError {
    InvalidDatabase,
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
            CreateCollectionError::InvalidDatabase => "invalid database",
            CreateCollectionError::CollectionExists => "collection exists",
            CreateCollectionError::SchemaError(err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            CreateCollectionError::InvalidDatabase => None,
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

#[derive(Debug)]
pub enum InsertError {
    InvalidDatabase,
    InvalidCollection,
    ObjectError(ObjectError),
}

impl fmt::Display for InsertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for InsertError {
    fn description(&self) -> &str {
        match self {
            InsertError::InvalidDatabase => "invalid database",
            InsertError::InvalidCollection => "invalid collection",
            InsertError::ObjectError(err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            InsertError::InvalidDatabase => None,
            InsertError::InvalidCollection => None,
            InsertError::ObjectError(err) => err.cause(),
        }
    }
}

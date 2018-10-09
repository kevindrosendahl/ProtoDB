use std::{error, fmt};

use super::super::schema::errors::{ObjectError, SchemaError};

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

use std::{error, fmt};

#[derive(Debug)]
pub enum StorageError {
    DatabaseError(DatabaseError),
    CollectionError(CollectionError),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for StorageError {
    fn description(&self) -> &str {
        match self {
            StorageError::DatabaseError(ref err) => err.description(),
            StorageError::CollectionError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            StorageError::DatabaseError(ref e) => e.cause(),
            StorageError::CollectionError(ref e) => e.cause(),
        }
    }
}

impl From<DatabaseError> for StorageError {
    fn from(err: DatabaseError) -> StorageError {
        StorageError::DatabaseError(err)
    }
}

impl From<CollectionError> for StorageError {
    fn from(err: CollectionError) -> StorageError {
        StorageError::CollectionError(err)
    }
}

#[derive(Debug)]
pub enum DatabaseError {
    DatabaseAlreadyExists,
    InvalidDatabase,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for DatabaseError {
    fn description(&self) -> &str {
        match self {
            DatabaseError::DatabaseAlreadyExists => "database already exists",
            DatabaseError::InvalidDatabase => "invalid database",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            DatabaseError::DatabaseAlreadyExists => None,
            DatabaseError::InvalidDatabase => None,
        }
    }
}

#[derive(Debug)]
pub enum CollectionError {
    CollectionAlreadyExists,
}

impl fmt::Display for CollectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for CollectionError {
    fn description(&self) -> &str {
        match self {
            CollectionError::CollectionAlreadyExists => "collection already exists",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            CollectionError::CollectionAlreadyExists => None,
        }
    }
}

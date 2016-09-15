use super::collection::CollectionError;
use super::database::DatabaseError;

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ServerError {
    CollectionError(CollectionError),
    DatabaseError(DatabaseError),
    DatabaseDoesNotExist,
    DatabaseAlreadyExists,
    CollectionDoesNotExist,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for ServerError {
    fn description(&self) -> &str {
        match self {
            &ServerError::CollectionError(ref err) => err.description(),
            &ServerError::DatabaseError(ref err) => err.description(),
            &ServerError::DatabaseDoesNotExist => "database does not exist",
            &ServerError::DatabaseAlreadyExists => "database already exists",
            &ServerError::CollectionDoesNotExist => "collection does not exist",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &ServerError::CollectionError(ref e) => e.cause(),
            &ServerError::DatabaseError(ref e) => e.cause(),
            &ServerError::DatabaseDoesNotExist => None,
            &ServerError::DatabaseAlreadyExists => None,
            &ServerError::CollectionDoesNotExist => None,
        }
    }
}

impl From<DatabaseError> for ServerError {
    fn from(err: DatabaseError) -> ServerError {
        ServerError::DatabaseError(err)
    }
}

impl From<CollectionError> for ServerError {
    fn from(err: CollectionError) -> ServerError {
        ServerError::CollectionError(err)
    }
}

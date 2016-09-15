use super::database::DatabaseError;

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ServerError {
    DatabaseError(DatabaseError),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for ServerError {
    fn description(&self) -> &str {
        match self {
            &ServerError::DatabaseError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &ServerError::DatabaseError(ref e) => e.cause(),
        }
    }
}

impl From<DatabaseError> for ServerError {
    fn from(err: DatabaseError) -> ServerError {
        ServerError::DatabaseError(err)
    }
}

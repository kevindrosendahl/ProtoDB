use std::{error, fmt};

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

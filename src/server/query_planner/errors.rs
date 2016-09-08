use std::error;
use std::fmt;

#[derive(Debug)]
pub enum QueryPlannerError {
    InvalidDatabase,
    InvalidCollection,
}

impl fmt::Display for QueryPlannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for QueryPlannerError {
    fn description(&self) -> &str {
        match self {
            &QueryPlannerError::InvalidDatabase => "invalid database",
            &QueryPlannerError::InvalidCollection => "invalid_collection",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            _ => None,
        }
    }
}

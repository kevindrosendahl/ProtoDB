use super::DatabaseError;

use std::result;

pub type Result<T> = result::Result<T, DatabaseError>;

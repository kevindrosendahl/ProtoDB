use super::ServerError;

use std::result;

pub type Result<T> = result::Result<T, ServerError>;

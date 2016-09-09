use super::EncodingError;

use std::result;

pub type Result<T> = result::Result<T, EncodingError>;

use super::errors::CollectionError;

use std::result;

pub type Result<T> = result::Result<T, CollectionError>;

use std::{error, fmt};

use crate::storage::errors::InternalStorageEngineError;

#[derive(Debug)]
pub enum IndexInsertError {
    InternalStorageEngineError(InternalStorageEngineError),
}

impl fmt::Display for IndexInsertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for IndexInsertError {}

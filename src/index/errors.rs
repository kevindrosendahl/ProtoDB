use std::{error, fmt};

use crate::storage::errors::InternalStorageEngineError;

#[derive(Debug)]
pub enum BuildIndexError {
    InternalStorageEngineError(InternalStorageEngineError),
}

impl fmt::Display for BuildIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for BuildIndexError {}

impl From<InternalStorageEngineError> for BuildIndexError {
    fn from(err: InternalStorageEngineError) -> Self {
        BuildIndexError::InternalStorageEngineError(err)
    }
}

impl From<IndexInsertError> for BuildIndexError {
    fn from(err: IndexInsertError) -> Self {
        match err {
            IndexInsertError::InternalStorageEngineError(err) => {
                BuildIndexError::InternalStorageEngineError(err)
            }
        }
    }
}

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

impl From<InternalStorageEngineError> for IndexInsertError {
    fn from(err: InternalStorageEngineError) -> Self {
        IndexInsertError::InternalStorageEngineError(err)
    }
}

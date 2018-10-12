use std::{error, fmt};

#[derive(Debug)]
pub struct InternalStorageEngineError {
    pub message: String,
}

impl fmt::Display for InternalStorageEngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for InternalStorageEngineError {}

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum EncodingError {
    BufferTooSmall,
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for EncodingError {
    fn description(&self) -> &str {
        match self {
            &EncodingError::BufferTooSmall => "supplied buffer too small",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            _ => None,
        }
    }
}

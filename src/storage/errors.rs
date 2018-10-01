use std::{error, fmt};

#[derive(Debug)]
pub enum StorageError {
    CollectionAlreadyExists,
    CollectionError(CollectionError),
    ProtobufError(::protobuf::ProtobufError),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for StorageError {
    fn description(&self) -> &str {
        match self {
            &StorageError::CollectionAlreadyExists => "collection already exists",
            &StorageError::CollectionError(ref err) => err.description(),
            &StorageError::ProtobufError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &StorageError::CollectionAlreadyExists => None,
            &StorageError::CollectionError(ref e) => e.cause(),
            &StorageError::ProtobufError(ref e) => e.cause(),
        }
    }
}

impl From<CollectionError> for StorageError {
    fn from(err: CollectionError) -> StorageError {
        StorageError::CollectionError(err)
    }
}

impl From<::protobuf::ProtobufError> for StorageError {
    fn from(err: ::protobuf::ProtobufError) -> StorageError {
        StorageError::ProtobufError(err)
    }
}

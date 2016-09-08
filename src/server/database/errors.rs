use server::collection::CollectionError;

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
    CollectionAlreadyExists,
    CollectionError(CollectionError),
    ProtobufError(::protobuf::ProtobufError),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for DatabaseError {
    fn description(&self) -> &str {
        match self {
            &DatabaseError::CollectionAlreadyExists => "collection already exists",
            &DatabaseError::CollectionError(ref err) => err.description(),
            &DatabaseError::ProtobufError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &DatabaseError::CollectionAlreadyExists => None,
            &DatabaseError::CollectionError(ref e) => e.cause(),
            &DatabaseError::ProtobufError(ref e) => e.cause(),
        }
    }
}

impl From<CollectionError> for DatabaseError {
    fn from(err: CollectionError) -> DatabaseError {
        DatabaseError::CollectionError(err)
    }
}

impl From<::protobuf::ProtobufError> for DatabaseError {
    fn from(err: ::protobuf::ProtobufError) -> DatabaseError {
        DatabaseError::ProtobufError(err)
    }
}

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum SchemaError {
    MissingIdField,
    DecodeError,
    EncodeError,
    ProtobufError(::protobuf::ProtobufError),
}

impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for SchemaError {
    fn description(&self) -> &str {
        match self {
            &SchemaError::MissingIdField => "missing _id field",
            &SchemaError::DecodeError => "error decoding",
            &SchemaError::EncodeError => "error encoding",
            &SchemaError::ProtobufError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &SchemaError::ProtobufError(ref e) => e.cause(),
            _ => None,
        }
    }
}

impl From<::protobuf::ProtobufError> for SchemaError {
    fn from(err: ::protobuf::ProtobufError) -> SchemaError {
        SchemaError::ProtobufError(err)
    }
}

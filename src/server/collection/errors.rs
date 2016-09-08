use server::schema::SchemaError;
use server::util::encoding::EncodingError;

extern crate lmdb_rs as lmdb;
use self::lmdb::MdbError;

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum CollectionError {
    IdFieldSetOnInsert,
    SchemaError(SchemaError),
    LmdbError(MdbError),
    EncodingError(EncodingError),
}

impl fmt::Display for CollectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for CollectionError {
    fn description(&self) -> &str {
        match self {
            &CollectionError::IdFieldSetOnInsert => "_id field set on insert call",
            &CollectionError::SchemaError(ref err) => err.description(),
            &CollectionError::LmdbError(ref err) => err.description(),
            &CollectionError::EncodingError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &CollectionError::IdFieldSetOnInsert => None,
            &CollectionError::SchemaError(ref e) => e.cause(),
            &CollectionError::LmdbError(ref e) => e.cause(),
            &CollectionError::EncodingError(ref e) => e.cause(),
        }
    }
}

impl From<SchemaError> for CollectionError {
    fn from(err: SchemaError) -> CollectionError {
        CollectionError::SchemaError(err)
    }
}

impl From<MdbError> for CollectionError {
    fn from(err: MdbError) -> CollectionError {
        CollectionError::LmdbError(err)
    }
}

impl From<EncodingError> for CollectionError {
    fn from(err: EncodingError) -> CollectionError {
        CollectionError::EncodingError(err)
    }
}

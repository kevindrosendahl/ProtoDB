use std::{error, fmt};

#[derive(Debug)]
pub enum ClientError {
    GrpcError(tower_grpc::Error),
    HttpError(tower_grpc::Error<tower_h2::client::Error>),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for ClientError {}

impl From<tower_grpc::Error> for ClientError {
    fn from(err: tower_grpc::Error) -> ClientError {
        ClientError::GrpcError(err)
    }
}

impl From<tower_grpc::Error<tower_h2::client::Error>> for ClientError {
    fn from(err: tower_grpc::Error<tower_h2::client::Error>) -> ClientError {
        ClientError::HttpError(err)
    }
}

use super::generated::protodb;
use super::{Client, ClientError};

use futures::Future;
use prost_types::DescriptorProto;
use tower_grpc::Request;

impl Client {
    pub fn insert_object(
        &mut self,
        database: String,
        collection: String,
        object: Vec<u8>,
    ) -> Result<protodb::object::InsertObjectResponse, ClientError> {
        let request = self
            .client
            .insert_object(Request::new(protodb::object::InsertObjectRequest {
                database,
                collection,
                object,
            }))
            .and_then(|response| Ok(response.into_inner()));

        Ok(self.core.run(request)?)
    }

    pub fn find_object(
        &mut self,
        database: String,
        collection: String,
        id: u64,
    ) -> Result<protodb::object::FindObjectResponse, ClientError> {
        let request = self
            .client
            .find_object(Request::new(protodb::object::FindObjectRequest {
                database,
                collection,
                id,
            }))
            .and_then(|response| Ok(response.into_inner()));

        Ok(self.core.run(request)?)
    }
}

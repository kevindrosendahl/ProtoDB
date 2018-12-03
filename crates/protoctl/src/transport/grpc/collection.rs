use super::generated::protodb;
use super::{Client, ClientError};

use futures::Future;
use prost_types::DescriptorProto;
use tower_grpc::Request;

impl Client {
    pub fn create_collection(
        &mut self,
        database: String,
        name: String,
        schema: DescriptorProto,
    ) -> Result<protodb::collection::CreateCollectionResponse, ClientError> {
        let request = self
            .client
            .create_collection(Request::new(protodb::collection::CreateCollectionRequest {
                database,
                name,
                schema: Some(schema),
            }))
            .and_then(|response| Ok(response.into_inner()));

        Ok(self.core.run(request)?)
    }

    pub fn list_collections(
        &mut self,
        database: String,
    ) -> Result<protodb::collection::ListCollectionsResponse, ClientError> {
        let request = self
            .client
            .list_collections(Request::new(protodb::collection::ListCollectionsRequest {
                database,
            }))
            .and_then(|response| Ok(response.into_inner()));

        Ok(self.core.run(request)?)
    }

    pub fn get_collection_info(
        &mut self,
        database: String,
        collection: String,
    ) -> Result<protodb::collection::GetCollectionInfoResponse, ClientError> {
        let request = self
            .client
            .get_collection_info(Request::new(
                protodb::collection::GetCollectionInfoRequest {
                    database,
                    collection,
                },
            ))
            .and_then(|response| Ok(response.into_inner()));

        Ok(self.core.run(request)?)
    }
}

use super::generated::protodb;
use super::{Client, ClientError};

use futures::Future;
use tower_grpc::Request;

impl Client {
    pub fn create_index(
        &mut self,
        database: String,
        collection: String,
        field: i32,
    ) -> Result<protodb::index::CreateIndexResponse, ClientError> {
        let request = self
            .client
            .create_index(Request::new(protodb::index::CreateIndexRequest {
                database,
                collection,
                field,
            }))
            .and_then(|response| Ok(response.into_inner()));

        Ok(self.core.run(request)?)
    }

    pub fn get_index(
        &mut self,
        database: String,
        collection: String,
        id: u64,
    ) -> Result<protodb::index::GetIndexResponse, ClientError> {
        let request = self
            .client
            .get_index(Request::new(protodb::index::GetIndexRequest {
                database,
                collection,
                id,
            }))
            .and_then(|response| Ok(response.into_inner()));

        Ok(self.core.run(request)?)
    }

    pub fn list_indexes(
        &mut self,
        database: String,
        collection: String,
    ) -> Result<protodb::index::ListIndexesResponse, ClientError> {
        let request = self
            .client
            .list_indexes(Request::new(protodb::index::ListIndexesRequest {
                database,
                collection,
            }))
            .and_then(|response| Ok(response.into_inner()));

        Ok(self.core.run(request)?)
    }
}

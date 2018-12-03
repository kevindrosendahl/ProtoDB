use super::generated::protodb;
use super::{Client, ClientError};

use futures::Future;
use tower_grpc::Request;

impl Client {
    pub fn create_database(
        &mut self,
        name: String,
    ) -> Result<protodb::database::CreateDatabaseResponse, ClientError> {
        let request = self
            .client
            .create_database(Request::new(protodb::database::CreateDatabaseRequest {
                name,
            }))
            .and_then(|response| Ok(response.into_inner()));

        Ok(self.core.run(request)?)
    }

    pub fn list_databases(
        &mut self,
    ) -> Result<protodb::database::ListDatabasesResponse, ClientError> {
        let request = self
            .client
            .list_databases(Request::new(protodb::database::ListDatabasesRequest {}))
            .and_then(|response| Ok(response.into_inner()));

        Ok(self.core.run(request)?)
    }
}

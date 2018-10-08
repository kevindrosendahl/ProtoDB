use super::super::generated::protodb::database;
use super::Handler;

use crate::storage::errors;

use tower_grpc::Request;

impl Handler {
    pub(crate) fn handle_create_database(
        &mut self,
        request: &Request<database::CreateDatabaseRequest>,
    ) -> database::CreateDatabaseResponse {
        self.storage_engine
            .clone()
            .catalog()
            .create_database(&request.get_ref().name)
            .and(Ok(database::CreateDatabaseResponse {
                success: true,
                failure_code: database::create_database_response::FailureCode::NoError as i32,
            })).unwrap_or_else(|err| {
                let failure_code = match err {
                    errors::database::CreateDatabaseError::DatabaseExists => {
                        database::create_database_response::FailureCode::DatabaseExists
                    }
                };
                database::CreateDatabaseResponse {
                    success: false,
                    failure_code: failure_code as i32,
                }
            })
    }

    pub(crate) fn handle_list_databases(
        &mut self,
        _: &Request<database::ListDatabasesRequest>,
    ) -> database::ListDatabasesResponse {
        database::ListDatabasesResponse {
            databases: self.storage_engine.clone().catalog().list_databases(),
        }
    }
}

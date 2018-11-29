use super::super::generated::protodb::database;
use super::Handler;

use crate::catalog::errors;

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
                error_code: database::create_database_response::ErrorCode::NoError as i32,
            }))
            .unwrap_or_else(|err| {
                let error_code = match err {
                    errors::database::CreateDatabaseError::DatabaseExists => {
                        database::create_database_response::ErrorCode::DatabaseExists
                    }
                    errors::database::CreateDatabaseError::InternalStorageEngineError(_) => {
                        database::create_database_response::ErrorCode::InternalError
                    }
                };
                database::CreateDatabaseResponse {
                    error_code: error_code as i32,
                }
            })
    }

    pub(crate) fn handle_list_databases(
        &mut self,
        _: &Request<database::ListDatabasesRequest>,
    ) -> database::ListDatabasesResponse {
        database::ListDatabasesResponse {
            error_code: database::list_databases_response::ErrorCode::NoError as i32,
            databases: self.storage_engine.clone().catalog().list_databases(),
        }
    }
}

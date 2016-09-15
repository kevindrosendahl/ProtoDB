use ::server::{Server, ServerError};

use ::server::protos::server_grpc::*;
use ::server::protos::database_create::{CreateDatabaseResponse, CreateDatabaseResponse_FailureCode};

use ::grpc::result::GrpcResult;
use ::grpc::error::GrpcError;

use std::error::Error;

impl ProtoDB for Server {
    fn CreateDatabase
        (&self,
         p: ::server::protos::database_create::CreateDatabase)
         -> ::grpc::result::GrpcResult<::server::protos::database_create::CreateDatabaseResponse> {

        let mut response = CreateDatabaseResponse::new();
        if let Err(err) = self.create_database(p.get_name(), self.next_database_id()) {
            match err {
                ServerError::DatabaseAlreadyExists => {
                    response.set_success(false);
                    response.set_failure_code(CreateDatabaseResponse_FailureCode::DATABASE_ALREADY_EXISTS);
                }
                _ => return Err(GrpcError::Panic(String::from(err.description())))
            }
        } else {
            response.set_success(true)
        }

        Ok(response)
    }

    fn DeleteDatabase
        (&self,
         p: ::server::protos::database_delete::DeleteDatabase)
         -> ::grpc::result::GrpcResult<::server::protos::database_delete::DeleteDatabaseResponse> {
        unimplemented!();
    }

    fn ListDatabases(&self,
                     p: ::server::protos::database_list::ListDatabases)
                     -> ::grpc::result::GrpcResult<::server::protos::database_list::ListDatabasesResponse> {
        unimplemented!();
    }

    fn DefineDatabase
        (&self,
         p: ::server::protos::database_define::DefineDatabase)
         -> ::grpc::result::GrpcResult<::server::protos::database_define::DefineDatabaseResponse> {
        unimplemented!();
    }

    fn CreateCollection
        (&self,
         p: ::server::protos::collection_create::CreateCollection)
         -> ::grpc::result::GrpcResult<::server::protos::collection_create::CreateCollectionResponse> {
        unimplemented!();
    }

    fn DeleteCollection
        (&self,
         p: ::server::protos::collection_delete::DeleteCollection)
         -> ::grpc::result::GrpcResult<::server::protos::collection_delete::DeleteCollectionResponse> {
        unimplemented!();
    }

    fn ListCollections
        (&self,
         p: ::server::protos::collection_list::ListCollections)
         -> ::grpc::result::GrpcResult<::server::protos::collection_list::ListCollectionsResponse> {
        unimplemented!();
    }

    fn DefineCollection
        (&self,
         p: ::server::protos::collection_define::DefineCollection)
         -> ::grpc::result::GrpcResult<::server::protos::collection_define::DefineCollectionResponse> {
        unimplemented!();
    }
}

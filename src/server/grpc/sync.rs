use ::server::protos::server_grpc::*;

use ::grpc::result::GrpcResult;

pub struct SyncServer;

impl ProtoDB for SyncServer {
    fn CreateDatabase
        (&self,
         p: ::server::protos::database_create::CreateDatabase)
         -> ::grpc::result::GrpcResult<::server::protos::database_create::CreateDatabaseResponse> {
        println!("Got request to create database: {}", p.get_name());
        let mut r = ::server::protos::database_create::CreateDatabaseResponse::new();
        r.set_success(false);
        r.set_failure_code(::server::protos::database_create::CreateDatabaseResponse_FailureCode::INVALID_DATABASE_NAME);
        Ok(r)
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

use ::server::{Server, ServerError};

use ::server::protos::server_grpc::*;
use ::server::protos::database_create::{CreateDatabase, CreateDatabaseResponse,
                                        CreateDatabaseResponse_FailureCode as CreateDbFailureCode};
use ::server::protos::database_delete::{DeleteDatabase, DeleteDatabaseResponse};
use ::server::protos::database_define::{DefineDatabase, DefineDatabaseResponse};
use ::server::protos::database_list::{ListDatabases, ListDatabasesResponse};
use ::server::protos::collection_create::{CreateCollection, CreateCollectionResponse};
use ::server::protos::collection_delete::{DeleteCollection, DeleteCollectionResponse};
use ::server::protos::collection_define::{DefineCollection, DefineCollectionResponse};
use ::server::protos::collection_list::{ListCollections, ListCollectionsResponse};

use ::grpc::result::GrpcResult;
use ::grpc::error::GrpcError;

use ::protobuf::RepeatedField;

use std::error::Error;

extern crate lmdb_rs;
use self::lmdb_rs::DbFlags as LmdbDbFlags;

impl ProtoDB for Server {
    fn CreateDatabase(&self, p: CreateDatabase) -> GrpcResult<CreateDatabaseResponse> {
        let mut response = CreateDatabaseResponse::new();
        match self.create_database(p.get_name(), self.next_database_id()) {
            Ok(_) => response.set_success(true),
            Err(err) => try!(self.handle_create_database_error(err, &mut response)),
        }

        Ok(response)
    }

    fn DeleteDatabase(&self, p: DeleteDatabase) -> GrpcResult<DeleteDatabaseResponse> {
        unimplemented!();
    }

    fn ListDatabases(&self, p: ListDatabases) -> GrpcResult<ListDatabasesResponse> {
        {
            let db_handle = self.lmdb_environment.get_default_db(LmdbDbFlags::empty()).unwrap();
            let reader = self.lmdb_environment.get_reader().unwrap();
            let db = reader.bind(&db_handle);
            for kv in db
                .iter()
                .unwrap() {
                println!("key: {:?}\nvalue: {}\n\n",
                         kv.get_key::<Vec<u8>>(),
                         String::from_utf8(kv.get_value::<Vec<u8>>()).unwrap());
            }
        }

        let mut response = ListDatabasesResponse::new();
        response.set_database(RepeatedField::from_vec(self.get_database_names()));
        Ok(response)
    }

    fn DefineDatabase(&self, p: DefineDatabase) -> GrpcResult<DefineDatabaseResponse> {
        unimplemented!();
    }

    fn CreateCollection(&self, p: CreateCollection) -> GrpcResult<CreateCollectionResponse> {
        unimplemented!();
    }

    fn DeleteCollection(&self, p: DeleteCollection) -> GrpcResult<DeleteCollectionResponse> {
        unimplemented!();
    }

    fn ListCollections(&self, p: ListCollections) -> GrpcResult<ListCollectionsResponse> {
        unimplemented!();
    }

    fn DefineCollection(&self, p: DefineCollection) -> GrpcResult<DefineCollectionResponse> {
        unimplemented!();
    }
}

impl Server {
    fn handle_create_database_error(&self,
                                    err: ServerError,
                                    response: &mut CreateDatabaseResponse)
                                    -> GrpcResult<()> {
        match err {
            ServerError::DatabaseAlreadyExists => {
                response.set_success(false);
                response.set_failure_code(CreateDbFailureCode::DATABASE_ALREADY_EXISTS);
            }
            _ => return Err(GrpcError::Panic(String::from(err.description()))),
        }
        Ok(())
    }
}

use ::server::{Server, ServerError};

use ::server::protos::server_grpc::*;
use ::server::protos::database_create::{CreateDatabase, CreateDatabaseResponse,
                                        CreateDatabaseResponse_FailureCode};
use ::server::protos::database_delete::{DeleteDatabase, DeleteDatabaseResponse};
use ::server::protos::database_define::{DefineDatabase, DefineDatabaseResponse};
use ::server::protos::database_list::{ListDatabases, ListDatabasesResponse};
use ::server::protos::collection_create::{CreateCollection, CreateCollectionResponse,
                                          CreateCollectionResponse_FailureCode};
use ::server::protos::collection_delete::{DeleteCollection, DeleteCollectionResponse};
use ::server::protos::collection_define::{DefineCollection, DefineCollectionResponse};
use ::server::protos::collection_list::{ListCollections, ListCollectionsResponse,
                                        ListCollectionsResponse_FailureCode};

use ::server::protos::find::{Find, FindResponse, FindResponse_FailureCode,
                             Query_QueryFieldOptions_Operator};
use ::server::protos::insert::{Insert, InsertResponse, InsertResponse_FailureCode};

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
            for kv in db.iter()
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
        let mut response = CreateCollectionResponse::new();
        match self.create_collection(p.get_database(), p.get_name(), p.get_schema().clone()) {
            Ok(_) => response.set_success(true),
            Err(err) => try!(self.handle_create_collection_error(err, &mut response)),
        }

        Ok(response)
    }

    fn DeleteCollection(&self, p: DeleteCollection) -> GrpcResult<DeleteCollectionResponse> {
        unimplemented!();
    }

    fn ListCollections(&self, p: ListCollections) -> GrpcResult<ListCollectionsResponse> {
        let mut response = ListCollectionsResponse::new();
        match self.get_collection_names(p.get_database()) {
            Ok(collection_names) => {
                response.set_success(true);
                response.set_collection(RepeatedField::from_vec(collection_names))
            }
            Err(err) => try!(self.handle_list_collections_error(err, &mut response)),
        }

        Ok(response)
    }

    fn DefineCollection(&self, p: DefineCollection) -> GrpcResult<DefineCollectionResponse> {
        unimplemented!();
    }

    fn Find(&self, p: Find) -> GrpcResult<FindResponse> {
        let mut response = FindResponse::new();
        let query = p.get_query();
        let query_field_options = query.get_query_field_options();

        match query_field_options.len() {
            0 => {
                match self.find_all(p.get_database(), p.get_collection()) {
                    Ok(option) => {
                        match option {
                            Some(data) => {
                                response.set_success(true);
                                response.set_num_found(data.len() as u64);
                                response.set_data(RepeatedField::from_vec(data));
                            }
                            None => {
                                response.set_success(true);
                                response.set_num_found(0);
                            }
                        };
                    }
                    Err(err) => try!(self.handle_find_error(err, &mut response)),
                };
            }
            1 => {
                let ref query_field_option = query_field_options[0];
                if query_field_option.has_field() &&
                   query_field_option.get_operator() == Query_QueryFieldOptions_Operator::EQUAL &&
                   query_field_option.has_uint64_value() {
                    match query_field_option.get_field() {
                        "_id" => {
                            match self.find(p.get_database(),
                                            p.get_collection(),
                                            query_field_option.get_uint64_value()) {
                                Ok(option) => {
                                    match option {
                                        Some(data) => {
                                            response.set_success(true);
                                            response.set_num_found(1);
                                            let mut data_field = RepeatedField::new();
                                            data_field.push(data);
                                            response.set_data(data_field);
                                        }
                                        None => {
                                            response.set_success(true);
                                            response.set_num_found(0);
                                        }
                                    }
                                }
                                Err(err) => try!(self.handle_find_error(err, &mut response)),
                            };
                        }
                        _ => {
                            response.set_success(false);
                            response.set_failure_code(FindResponse_FailureCode::INVALID_QUERY);
                            // set_invalid_query();
                        }
                    };
                } else {
                    response.set_success(false);
                    response.set_failure_code(FindResponse_FailureCode::INVALID_QUERY);
                }
            }
            _ => {
                response.set_success(false);
                response.set_failure_code(FindResponse_FailureCode::INVALID_QUERY);
                // set_invalid_query();
            }
        };

        Ok(response)
    }

    fn Insert(&self, p: Insert) -> GrpcResult<InsertResponse> {
        let mut response = InsertResponse::new();
        match self.insert(p.get_database(),
                          p.get_collection(),
                          &mut Vec::from(p.get_data())) {
            Ok(_id) => {
                response.set_success(true);
                response.set__id(_id);
            }
            Err(err) => try!(self.handle_insert_error(err, &mut response)),
        }

        Ok(response)
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
                response.set_failure_code(CreateDatabaseResponse_FailureCode::DB_EXISTS);
            }
            _ => return Err(GrpcError::Panic(String::from(err.description()))),
        }
        Ok(())
    }

    fn handle_create_collection_error(&self,
                                      err: ServerError,
                                      response: &mut CreateCollectionResponse)
                                      -> GrpcResult<()> {
        match err {
            ServerError::DatabaseAlreadyExists => {
                response.set_success(false);
                response.set_failure_code(CreateCollectionResponse_FailureCode::COLLECTION_EXISTS);
            }
            _ => return Err(GrpcError::Panic(String::from(err.description()))),
        }
        Ok(())
    }

    fn handle_list_collections_error(&self,
                                     err: ServerError,
                                     response: &mut ListCollectionsResponse)
                                     -> GrpcResult<()> {
        match err {
            ServerError::DatabaseDoesNotExist => {
                response.set_success(false);
                response.set_failure_code(ListCollectionsResponse_FailureCode::INVALID_DB);
            }
            _ => return Err(GrpcError::Panic(String::from(err.description()))),
        }
        Ok(())
    }

    fn handle_find_error(&self, err: ServerError, response: &mut FindResponse) -> GrpcResult<()> {
        println!("*********** got err: {:?}", err);
        match err {
            ServerError::DatabaseDoesNotExist => {
                response.set_success(false);
                response.set_failure_code(FindResponse_FailureCode::INVALID_DB);
            }
            ServerError::CollectionDoesNotExist => {
                response.set_success(false);
                response.set_failure_code(FindResponse_FailureCode::INVALID_COLLECTION);
            }
            _ => return Err(GrpcError::Panic(String::from(err.description()))),
        }
        Ok(())
    }

    fn handle_insert_error(&self,
                           err: ServerError,
                           response: &mut InsertResponse)
                           -> GrpcResult<()> {
        match err {
            ServerError::DatabaseDoesNotExist => {
                response.set_success(false);
                response.set_failure_code(InsertResponse_FailureCode::INVALID_DB);
            }
            ServerError::CollectionDoesNotExist => {
                response.set_success(false);
                response.set_failure_code(InsertResponse_FailureCode::INVALID_COLLECTION);
            }
            _ => return Err(GrpcError::Panic(String::from(err.description()))),
        }
        Ok(())
    }
}

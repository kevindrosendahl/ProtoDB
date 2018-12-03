pub mod client {
    use super::collection;
    use super::collection::{
        CreateCollectionRequest, CreateCollectionResponse, ListCollectionsRequest,
        ListCollectionsResponse,
    };
    use super::database;
    use super::database::{
        CreateDatabaseRequest, CreateDatabaseResponse, ListDatabasesRequest, ListDatabasesResponse,
    };
    use super::object;
    use super::object::{
        FindObjectRequest, FindObjectResponse, InsertObjectRequest, InsertObjectResponse,
    };
    use super::wasm;
    use super::wasm::{
        RegisterModuleRequest, RegisterModuleResponse, RunModuleRequest, RunModuleResponse,
    };
    use tower_grpc::codegen::client::*;

    #[derive(Debug)]
    pub struct ProtoDb<T> {
        inner: grpc::Grpc<T>,
    }

    impl<T> ProtoDb<T> {
        pub fn new(inner: T) -> Self {
            let inner = grpc::Grpc::new(inner);
            Self { inner }
        }

        pub fn poll_ready<R>(&mut self) -> futures::Poll<(), grpc::Error<T::Error>>
        where
            T: tower::HttpService<R>,
        {
            self.inner.poll_ready()
        }

        pub fn create_database<R>(
            &mut self,
            request: grpc::Request<database::CreateDatabaseRequest>,
        ) -> grpc::unary::ResponseFuture<database::CreateDatabaseResponse, T::Future, T::ResponseBody>
        where
            T: tower::HttpService<R>,
            T::ResponseBody: grpc::Body,
            grpc::unary::Once<database::CreateDatabaseRequest>: grpc::Encodable<R>,
        {
            let path = http::PathAndQuery::from_static("/protodb.ProtoDB/CreateDatabase");
            self.inner.unary(request, path)
        }

        pub fn list_databases<R>(
            &mut self,
            request: grpc::Request<database::ListDatabasesRequest>,
        ) -> grpc::unary::ResponseFuture<database::ListDatabasesResponse, T::Future, T::ResponseBody>
        where
            T: tower::HttpService<R>,
            T::ResponseBody: grpc::Body,
            grpc::unary::Once<database::ListDatabasesRequest>: grpc::Encodable<R>,
        {
            let path = http::PathAndQuery::from_static("/protodb.ProtoDB/ListDatabases");
            self.inner.unary(request, path)
        }

        pub fn create_collection<R>(
            &mut self,
            request: grpc::Request<collection::CreateCollectionRequest>,
        ) -> grpc::unary::ResponseFuture<
            collection::CreateCollectionResponse,
            T::Future,
            T::ResponseBody,
        >
        where
            T: tower::HttpService<R>,
            T::ResponseBody: grpc::Body,
            grpc::unary::Once<collection::CreateCollectionRequest>: grpc::Encodable<R>,
        {
            let path = http::PathAndQuery::from_static("/protodb.ProtoDB/CreateCollection");
            self.inner.unary(request, path)
        }

        pub fn list_collections<R>(
            &mut self,
            request: grpc::Request<collection::ListCollectionsRequest>,
        ) -> grpc::unary::ResponseFuture<
            collection::ListCollectionsResponse,
            T::Future,
            T::ResponseBody,
        >
        where
            T: tower::HttpService<R>,
            T::ResponseBody: grpc::Body,
            grpc::unary::Once<collection::ListCollectionsRequest>: grpc::Encodable<R>,
        {
            let path = http::PathAndQuery::from_static("/protodb.ProtoDB/ListCollections");
            self.inner.unary(request, path)
        }

        pub fn insert_object<R>(
            &mut self,
            request: grpc::Request<object::InsertObjectRequest>,
        ) -> grpc::unary::ResponseFuture<object::InsertObjectResponse, T::Future, T::ResponseBody>
        where
            T: tower::HttpService<R>,
            T::ResponseBody: grpc::Body,
            grpc::unary::Once<object::InsertObjectRequest>: grpc::Encodable<R>,
        {
            let path = http::PathAndQuery::from_static("/protodb.ProtoDB/InsertObject");
            self.inner.unary(request, path)
        }

        pub fn find_object<R>(
            &mut self,
            request: grpc::Request<object::FindObjectRequest>,
        ) -> grpc::unary::ResponseFuture<object::FindObjectResponse, T::Future, T::ResponseBody>
        where
            T: tower::HttpService<R>,
            T::ResponseBody: grpc::Body,
            grpc::unary::Once<object::FindObjectRequest>: grpc::Encodable<R>,
        {
            let path = http::PathAndQuery::from_static("/protodb.ProtoDB/FindObject");
            self.inner.unary(request, path)
        }

        pub fn register_wasm_module<R>(
            &mut self,
            request: grpc::Request<wasm::RegisterModuleRequest>,
        ) -> grpc::unary::ResponseFuture<wasm::RegisterModuleResponse, T::Future, T::ResponseBody>
        where
            T: tower::HttpService<R>,
            T::ResponseBody: grpc::Body,
            grpc::unary::Once<wasm::RegisterModuleRequest>: grpc::Encodable<R>,
        {
            let path = http::PathAndQuery::from_static("/protodb.ProtoDB/RegisterWasmModule");
            self.inner.unary(request, path)
        }

        pub fn run_wasm_module<R>(
            &mut self,
            request: grpc::Request<wasm::RunModuleRequest>,
        ) -> grpc::unary::ResponseFuture<wasm::RunModuleResponse, T::Future, T::ResponseBody>
        where
            T: tower::HttpService<R>,
            T::ResponseBody: grpc::Body,
            grpc::unary::Once<wasm::RunModuleRequest>: grpc::Encodable<R>,
        {
            let path = http::PathAndQuery::from_static("/protodb.ProtoDB/RunWasmModule");
            self.inner.unary(request, path)
        }
    }
}

pub mod collection {
    #[derive(Clone, PartialEq, Message)]
    pub struct CreateCollectionRequest {
        #[prost(string, tag = "1")]
        pub database: String,
        #[prost(string, tag = "2")]
        pub name: String,
        #[prost(message, optional, tag = "3")]
        pub schema: ::std::option::Option<::prost_types::DescriptorProto>,
    }
    #[derive(Clone, PartialEq, Message)]
    pub struct CreateCollectionResponse {
        #[prost(enumeration = "create_collection_response::ErrorCode", tag = "1")]
        pub error_code: i32,
        #[prost(message, optional, tag = "2")]
        pub schema_error: ::std::option::Option<create_collection_response::SchemaError>,
    }
    pub mod create_collection_response {
        #[derive(Clone, PartialEq, Message)]
        pub struct SchemaError {
            #[prost(enumeration = "schema_error::SchemaErrorCode", tag = "1")]
            pub code: i32,
            #[prost(string, tag = "2")]
            pub message: String,
        }
        pub mod schema_error {
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
            pub enum SchemaErrorCode {
                NoSchemaError = 0,
                MissingIdField = 1,
                InvalidIdType = 2,
                InvalidFieldType = 3,
                EncodingError = 4,
            }
        }
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
        pub enum ErrorCode {
            NoError = 0,
            InternalError = 1,
            InvalidDatabase = 2,
            CollectionExists = 3,
            SchemaError = 4,
        }
    }
    #[derive(Clone, PartialEq, Message)]
    pub struct ListCollectionsRequest {
        #[prost(string, tag = "1")]
        pub database: String,
    }
    #[derive(Clone, PartialEq, Message)]
    pub struct ListCollectionsResponse {
        #[prost(enumeration = "list_collections_response::ErrorCode", tag = "1")]
        pub error_code: i32,
        #[prost(string, repeated, tag = "2")]
        pub collections: ::std::vec::Vec<String>,
    }
    pub mod list_collections_response {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
        pub enum ErrorCode {
            NoError = 0,
            InternalError = 1,
            InvalidDatabase = 2,
        }
    }

}
pub mod database {
    #[derive(Clone, PartialEq, Message)]
    pub struct CreateDatabaseRequest {
        #[prost(string, tag = "1")]
        pub name: String,
    }
    #[derive(Clone, PartialEq, Message)]
    pub struct CreateDatabaseResponse {
        #[prost(enumeration = "create_database_response::ErrorCode", tag = "1")]
        pub error_code: i32,
    }
    pub mod create_database_response {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
        pub enum ErrorCode {
            NoError = 0,
            InternalError = 1,
            DatabaseExists = 2,
        }
    }
    #[derive(Clone, PartialEq, Message)]
    pub struct ListDatabasesRequest {}
    #[derive(Clone, PartialEq, Message)]
    pub struct ListDatabasesResponse {
        #[prost(enumeration = "list_databases_response::ErrorCode", tag = "1")]
        pub error_code: i32,
        #[prost(string, repeated, tag = "2")]
        pub databases: ::std::vec::Vec<String>,
    }
    pub mod list_databases_response {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
        pub enum ErrorCode {
            NoError = 0,
            InternalError = 1,
        }
    }

}
pub mod object {
    #[derive(Clone, PartialEq, Message)]
    pub struct FindObjectRequest {
        #[prost(string, tag = "1")]
        pub database: String,
        #[prost(string, tag = "2")]
        pub collection: String,
        #[prost(uint64, tag = "3")]
        pub id: u64,
    }
    #[derive(Clone, PartialEq, Message)]
    pub struct FindObjectResponse {
        #[prost(enumeration = "find_object_response::ErrorCode", tag = "1")]
        pub error_code: i32,
        #[prost(bytes, tag = "2")]
        pub object: Vec<u8>,
    }
    pub mod find_object_response {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
        pub enum ErrorCode {
            NoError = 0,
            InternalError = 1,
            InvalidDatabase = 2,
            InvalidCollection = 3,
            InvalidId = 4,
        }
    }
    #[derive(Clone, PartialEq, Message)]
    pub struct InsertObjectRequest {
        #[prost(string, tag = "1")]
        pub database: String,
        #[prost(string, tag = "2")]
        pub collection: String,
        #[prost(bytes, tag = "3")]
        pub object: Vec<u8>,
    }
    #[derive(Clone, PartialEq, Message)]
    pub struct InsertObjectResponse {
        #[prost(enumeration = "insert_object_response::ErrorCode", tag = "1")]
        pub error_code: i32,
        #[prost(message, optional, tag = "2")]
        pub object_error: ::std::option::Option<insert_object_response::ObjectError>,
    }
    pub mod insert_object_response {
        #[derive(Clone, PartialEq, Message)]
        pub struct ObjectError {
            #[prost(enumeration = "object_error::ObjectErrorCode", tag = "1")]
            pub code: i32,
            #[prost(string, tag = "2")]
            pub message: String,
        }
        pub mod object_error {
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
            pub enum ObjectErrorCode {
                NoObjectError = 0,
                DecodeError = 1,
            }
        }
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
        pub enum ErrorCode {
            NoError = 0,
            InternalError = 1,
            InvalidDatabase = 2,
            InvalidCollection = 3,
            ObjectExists = 4,
            ObjectError = 5,
        }
    }

}
pub mod wasm {
    #[derive(Clone, PartialEq, Message)]
    pub struct RegisterModuleRequest {
        #[prost(string, tag = "1")]
        pub database: String,
        #[prost(string, tag = "2")]
        pub name: String,
        #[prost(bytes, tag = "3")]
        pub wasm: Vec<u8>,
    }
    #[derive(Clone, PartialEq, Message)]
    pub struct RegisterModuleResponse {
        #[prost(enumeration = "register_module_response::ErrorCode", tag = "1")]
        pub error_code: i32,
    }
    pub mod register_module_response {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
        pub enum ErrorCode {
            NoError = 0,
            InternalError = 1,
            InvalidDatabase = 2,
            ModuleExists = 3,
        }
    }
    #[derive(Clone, PartialEq, Message)]
    pub struct RunModuleRequest {
        #[prost(string, tag = "1")]
        pub database: String,
        #[prost(string, tag = "2")]
        pub name: String,
    }
    #[derive(Clone, PartialEq, Message)]
    pub struct RunModuleResponse {
        #[prost(enumeration = "run_module_response::ErrorCode", tag = "1")]
        pub error_code: i32,
        #[prost(bytes, tag = "2")]
        pub result: Vec<u8>,
    }
    pub mod run_module_response {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
        pub enum ErrorCode {
            NoError = 0,
            InternalError = 1,
            InvalidDatabase = 2,
            InvalidModule = 3,
        }
    }

}

pub mod server {
    use ::tower_grpc::codegen::server::*;
use super::database;
use super::collection;
    use super::database::{CreateDatabaseRequest, CreateDatabaseResponse, ListDatabasesRequest, ListDatabasesResponse};
    use super::collection::{CreateCollectionRequest, CreateCollectionResponse, ListCollectionsRequest, ListCollectionsResponse, InsertObjectRequest, InsertObjectResponse};

    // Redefine the try_ready macro so that it doesn't need to be explicitly
    // imported by the user of this generated code.
    macro_rules! try_ready {
        ($e:expr) => (match $e {
            Ok(futures::Async::Ready(t)) => t,
            Ok(futures::Async::NotReady) => return Ok(futures::Async::NotReady),
            Err(e) => return Err(From::from(e)),
        })
    }

    pub trait ProtoDb: Clone {
        type CreateDatabaseFuture: futures::Future<Item = grpc::Response<database::CreateDatabaseResponse>, Error = grpc::Error>;
        type ListDatabasesFuture: futures::Future<Item = grpc::Response<database::ListDatabasesResponse>, Error = grpc::Error>;
        type CreateCollectionFuture: futures::Future<Item = grpc::Response<collection::CreateCollectionResponse>, Error = grpc::Error>;
        type ListCollectionsFuture: futures::Future<Item = grpc::Response<collection::ListCollectionsResponse>, Error = grpc::Error>;
        type InsertObjectFuture: futures::Future<Item = grpc::Response<collection::InsertObjectResponse>, Error = grpc::Error>;

        fn create_database(&mut self, request: grpc::Request<database::CreateDatabaseRequest>) -> Self::CreateDatabaseFuture;

        fn list_databases(&mut self, request: grpc::Request<database::ListDatabasesRequest>) -> Self::ListDatabasesFuture;

        fn create_collection(&mut self, request: grpc::Request<collection::CreateCollectionRequest>) -> Self::CreateCollectionFuture;

        fn list_collections(&mut self, request: grpc::Request<collection::ListCollectionsRequest>) -> Self::ListCollectionsFuture;

        fn insert_object(&mut self, request: grpc::Request<collection::InsertObjectRequest>) -> Self::InsertObjectFuture;
    }

    #[derive(Debug, Clone)]
    pub struct ProtoDbServer<T> {
        proto_db: T,
    }

    impl<T> ProtoDbServer<T>
    where T: ProtoDb,
    {
        pub fn new(proto_db: T) -> Self {
            Self { proto_db }
        }
    }

    impl<T> tower::Service for ProtoDbServer<T>
    where T: ProtoDb,
    {
        type Request = http::Request<tower_h2::RecvBody>;
        type Response = http::Response<proto_db::ResponseBody<T>>;
        type Error = h2::Error;
        type Future = proto_db::ResponseFuture<T>;

        fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
            Ok(().into())
        }

        fn call(&mut self, request: Self::Request) -> Self::Future {
            use self::proto_db::Kind::*;

            match request.uri().path() {
                "/protodb.ProtoDB/CreateDatabase" => {
                    let service = proto_db::methods::CreateDatabase(self.proto_db.clone());
                    let response = grpc::Grpc::unary(service, request);
                    proto_db::ResponseFuture { kind: Ok(CreateDatabase(response)) }
                }
                "/protodb.ProtoDB/ListDatabases" => {
                    let service = proto_db::methods::ListDatabases(self.proto_db.clone());
                    let response = grpc::Grpc::unary(service, request);
                    proto_db::ResponseFuture { kind: Ok(ListDatabases(response)) }
                }
                "/protodb.ProtoDB/CreateCollection" => {
                    let service = proto_db::methods::CreateCollection(self.proto_db.clone());
                    let response = grpc::Grpc::unary(service, request);
                    proto_db::ResponseFuture { kind: Ok(CreateCollection(response)) }
                }
                "/protodb.ProtoDB/ListCollections" => {
                    let service = proto_db::methods::ListCollections(self.proto_db.clone());
                    let response = grpc::Grpc::unary(service, request);
                    proto_db::ResponseFuture { kind: Ok(ListCollections(response)) }
                }
                "/protodb.ProtoDB/InsertObject" => {
                    let service = proto_db::methods::InsertObject(self.proto_db.clone());
                    let response = grpc::Grpc::unary(service, request);
                    proto_db::ResponseFuture { kind: Ok(InsertObject(response)) }
                }
                _ => {
                    proto_db::ResponseFuture { kind: Err(grpc::Status::UNIMPLEMENTED) }
                }
            }
        }
    }

    impl<T> tower::NewService for ProtoDbServer<T>
    where T: ProtoDb,
    {
        type Request = http::Request<tower_h2::RecvBody>;
        type Response = http::Response<proto_db::ResponseBody<T>>;
        type Error = h2::Error;
        type Service = Self;
        type InitError = h2::Error;
        type Future = futures::FutureResult<Self::Service, Self::Error>;

        fn new_service(&self) -> Self::Future {
            futures::ok(self.clone())
        }
    }

    pub mod proto_db {
        use ::tower_grpc::codegen::server::*;
        use super::ProtoDb;

        pub struct ResponseFuture<T>
        where T: ProtoDb,
        {
            pub(super) kind: Result<Kind<
                grpc::unary::ResponseFuture<methods::CreateDatabase<T>, tower_h2::RecvBody>,
                grpc::unary::ResponseFuture<methods::ListDatabases<T>, tower_h2::RecvBody>,
                grpc::unary::ResponseFuture<methods::CreateCollection<T>, tower_h2::RecvBody>,
                grpc::unary::ResponseFuture<methods::ListCollections<T>, tower_h2::RecvBody>,
                grpc::unary::ResponseFuture<methods::InsertObject<T>, tower_h2::RecvBody>,
            >, grpc::Status>,
        }

        impl<T> futures::Future for ResponseFuture<T>
        where T: ProtoDb,
        {
            type Item = http::Response<ResponseBody<T>>;
            type Error = h2::Error;

            fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
                use self::Kind::*;

                match self.kind {
                    Ok(CreateDatabase(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody { kind: Ok(CreateDatabase(body)) };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Ok(ListDatabases(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody { kind: Ok(ListDatabases(body)) };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Ok(CreateCollection(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody { kind: Ok(CreateCollection(body)) };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Ok(ListCollections(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody { kind: Ok(ListCollections(body)) };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Ok(InsertObject(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody { kind: Ok(InsertObject(body)) };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Err(ref status) => {
                        let body = ResponseBody { kind: Err(status.clone()) };
                        Ok(grpc::Response::new(body).into_http().into())
                    }
                }
            }
        }

        pub struct ResponseBody<T>
        where T: ProtoDb,
        {
            pub(super) kind: Result<Kind<
                grpc::Encode<grpc::unary::Once<<methods::CreateDatabase<T> as grpc::UnaryService>::Response>>,
                grpc::Encode<grpc::unary::Once<<methods::ListDatabases<T> as grpc::UnaryService>::Response>>,
                grpc::Encode<grpc::unary::Once<<methods::CreateCollection<T> as grpc::UnaryService>::Response>>,
                grpc::Encode<grpc::unary::Once<<methods::ListCollections<T> as grpc::UnaryService>::Response>>,
                grpc::Encode<grpc::unary::Once<<methods::InsertObject<T> as grpc::UnaryService>::Response>>,
            >, grpc::Status>,
        }

        impl<T> tower_h2::Body for ResponseBody<T>
        where T: ProtoDb,
        {
            type Data = bytes::Bytes;

            fn is_end_stream(&self) -> bool {
                use self::Kind::*;

                match self.kind {
                    Ok(CreateDatabase(ref v)) => v.is_end_stream(),
                    Ok(ListDatabases(ref v)) => v.is_end_stream(),
                    Ok(CreateCollection(ref v)) => v.is_end_stream(),
                    Ok(ListCollections(ref v)) => v.is_end_stream(),
                    Ok(InsertObject(ref v)) => v.is_end_stream(),
                    Err(_) => true,
                }
            }

            fn poll_data(&mut self) -> futures::Poll<Option<Self::Data>, h2::Error> {
                use self::Kind::*;

                match self.kind {
                    Ok(CreateDatabase(ref mut v)) => v.poll_data(),
                    Ok(ListDatabases(ref mut v)) => v.poll_data(),
                    Ok(CreateCollection(ref mut v)) => v.poll_data(),
                    Ok(ListCollections(ref mut v)) => v.poll_data(),
                    Ok(InsertObject(ref mut v)) => v.poll_data(),
                    Err(_) => Ok(None.into()),
                }
            }

            fn poll_trailers(&mut self) -> futures::Poll<Option<http::HeaderMap>, h2::Error> {
                use self::Kind::*;

                match self.kind {
                    Ok(CreateDatabase(ref mut v)) => v.poll_trailers(),
                    Ok(ListDatabases(ref mut v)) => v.poll_trailers(),
                    Ok(CreateCollection(ref mut v)) => v.poll_trailers(),
                    Ok(ListCollections(ref mut v)) => v.poll_trailers(),
                    Ok(InsertObject(ref mut v)) => v.poll_trailers(),
                    Err(ref status) => {
                        let mut map = http::HeaderMap::new();
                        map.insert("grpc-status", status.to_header_value());
                        Ok(Some(map).into())
                    }
                }
            }
        }

        #[derive(Debug, Clone)]
        pub(super) enum Kind<CreateDatabase, ListDatabases, CreateCollection, ListCollections, InsertObject> {
            CreateDatabase(CreateDatabase),
            ListDatabases(ListDatabases),
            CreateCollection(CreateCollection),
            ListCollections(ListCollections),
            InsertObject(InsertObject),
        }

        pub mod methods {
            use ::tower_grpc::codegen::server::*;
use super::super::database;
use super::super::collection;
            use super::super::ProtoDb;
            use super::super::database::{CreateDatabaseRequest, CreateDatabaseResponse, ListDatabasesRequest, ListDatabasesResponse};
            use super::super::collection::{CreateCollectionRequest, CreateCollectionResponse, ListCollectionsRequest, ListCollectionsResponse, InsertObjectRequest, InsertObjectResponse};

            pub struct CreateDatabase<T>(pub T);

            impl<T> tower::Service for CreateDatabase<T>
            where T: ProtoDb,
            {
                type Request = grpc::Request<database::CreateDatabaseRequest>;
                type Response = grpc::Response<database::CreateDatabaseResponse>;
                type Error = grpc::Error;
                type Future = T::CreateDatabaseFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: Self::Request) -> Self::Future {
                    self.0.create_database(request)
                }
            }

            pub struct ListDatabases<T>(pub T);

            impl<T> tower::Service for ListDatabases<T>
            where T: ProtoDb,
            {
                type Request = grpc::Request<database::ListDatabasesRequest>;
                type Response = grpc::Response<database::ListDatabasesResponse>;
                type Error = grpc::Error;
                type Future = T::ListDatabasesFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: Self::Request) -> Self::Future {
                    self.0.list_databases(request)
                }
            }

            pub struct CreateCollection<T>(pub T);

            impl<T> tower::Service for CreateCollection<T>
            where T: ProtoDb,
            {
                type Request = grpc::Request<collection::CreateCollectionRequest>;
                type Response = grpc::Response<collection::CreateCollectionResponse>;
                type Error = grpc::Error;
                type Future = T::CreateCollectionFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: Self::Request) -> Self::Future {
                    self.0.create_collection(request)
                }
            }

            pub struct ListCollections<T>(pub T);

            impl<T> tower::Service for ListCollections<T>
            where T: ProtoDb,
            {
                type Request = grpc::Request<collection::ListCollectionsRequest>;
                type Response = grpc::Response<collection::ListCollectionsResponse>;
                type Error = grpc::Error;
                type Future = T::ListCollectionsFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: Self::Request) -> Self::Future {
                    self.0.list_collections(request)
                }
            }

            pub struct InsertObject<T>(pub T);

            impl<T> tower::Service for InsertObject<T>
            where T: ProtoDb,
            {
                type Request = grpc::Request<collection::InsertObjectRequest>;
                type Response = grpc::Response<collection::InsertObjectResponse>;
                type Error = grpc::Error;
                type Future = T::InsertObjectFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: Self::Request) -> Self::Future {
                    self.0.insert_object(request)
                }
            }
        }
    }
}

pub mod collection {
#[derive(Clone, PartialEq, Message)]
pub struct CreateCollectionRequest {
    #[prost(string, tag="1")]
    pub database: String,
    #[prost(string, tag="2")]
    pub name: String,
    #[prost(message, optional, tag="3")]
    pub schema: ::std::option::Option<::prost_types::DescriptorProto>,
}
#[derive(Clone, PartialEq, Message)]
pub struct CreateCollectionResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(enumeration="create_collection_response::FailureCode", tag="2")]
    pub failure_code: i32,
    #[prost(enumeration="create_collection_response::SchemaError", tag="3")]
    pub schema_error: i32,
}
pub mod create_collection_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    pub enum FailureCode {
        NoFailure = 0,
        InvalidDatabase = 1,
        CollectionExists = 2,
        SchemaError = 3,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    pub enum SchemaError {
        NoSchemaError = 0,
        MissingIdField = 1,
        InvalidIdType = 2,
    }
}
#[derive(Clone, PartialEq, Message)]
pub struct InsertObjectRequest {
    #[prost(string, tag="1")]
    pub database: String,
    #[prost(string, tag="2")]
    pub collection: String,
    #[prost(bytes, tag="3")]
    pub object: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct InsertObjectResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(enumeration="insert_object_response::FailureCode", tag="2")]
    pub failure_code: i32,
    #[prost(message, optional, tag="3")]
    pub object_error: ::std::option::Option<insert_object_response::ObjectError>,
}
pub mod insert_object_response {
    #[derive(Clone, PartialEq, Message)]
    pub struct ObjectError {
        #[prost(enumeration="object_error::ObjectErrorCode", tag="1")]
        pub code: i32,
        #[prost(string, tag="2")]
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
    pub enum FailureCode {
        NoFailure = 0,
        InvalidDatabase = 1,
        InvalidCollection = 2,
        ObjectError = 3,
    }
}
#[derive(Clone, PartialEq, Message)]
pub struct ListCollectionsRequest {
    #[prost(string, tag="1")]
    pub database: String,
}
#[derive(Clone, PartialEq, Message)]
pub struct ListCollectionsResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(enumeration="list_collections_response::FailureCode", tag="2")]
    pub failure_code: i32,
    #[prost(string, repeated, tag="3")]
    pub collections: ::std::vec::Vec<String>,
}
pub mod list_collections_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    pub enum FailureCode {
        NoError = 0,
        InvalidDatabase = 1,
    }
}

}
pub mod database {
#[derive(Clone, PartialEq, Message)]
pub struct CreateDatabaseRequest {
    #[prost(string, tag="1")]
    pub name: String,
}
#[derive(Clone, PartialEq, Message)]
pub struct CreateDatabaseResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(enumeration="create_database_response::FailureCode", tag="2")]
    pub failure_code: i32,
}
pub mod create_database_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    pub enum FailureCode {
        NoError = 0,
        DatabaseExists = 1,
    }
}
#[derive(Clone, PartialEq, Message)]
pub struct ListDatabasesRequest {
}
#[derive(Clone, PartialEq, Message)]
pub struct ListDatabasesResponse {
    #[prost(string, repeated, tag="1")]
    pub databases: ::std::vec::Vec<String>,
}

}
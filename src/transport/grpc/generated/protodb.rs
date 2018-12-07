pub mod server {
    use super::collection;
    use super::collection::{
        CreateCollectionRequest, CreateCollectionResponse, GetCollectionInfoRequest,
        GetCollectionInfoResponse, ListCollectionsRequest, ListCollectionsResponse,
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
        GetModuleInfoRequest, GetModuleInfoResponse, RegisterModuleRequest, RegisterModuleResponse,
        RunModuleRequest, RunModuleResponse,
    };
    use tower_grpc::codegen::server::*;

    // Redefine the try_ready macro so that it doesn't need to be explicitly
    // imported by the user of this generated code.
    macro_rules! try_ready {
        ($e:expr) => {
            match $e {
                Ok(futures::Async::Ready(t)) => t,
                Ok(futures::Async::NotReady) => return Ok(futures::Async::NotReady),
                Err(e) => return Err(From::from(e)),
            }
        };
    }

    pub trait ProtoDb: Clone {
        type CreateDatabaseFuture: futures::Future<
            Item = grpc::Response<database::CreateDatabaseResponse>,
            Error = grpc::Error,
        >;
        type ListDatabasesFuture: futures::Future<
            Item = grpc::Response<database::ListDatabasesResponse>,
            Error = grpc::Error,
        >;
        type CreateCollectionFuture: futures::Future<
            Item = grpc::Response<collection::CreateCollectionResponse>,
            Error = grpc::Error,
        >;
        type GetCollectionInfoFuture: futures::Future<
            Item = grpc::Response<collection::GetCollectionInfoResponse>,
            Error = grpc::Error,
        >;
        type ListCollectionsFuture: futures::Future<
            Item = grpc::Response<collection::ListCollectionsResponse>,
            Error = grpc::Error,
        >;
        type FindObjectFuture: futures::Future<
            Item = grpc::Response<object::FindObjectResponse>,
            Error = grpc::Error,
        >;
        type InsertObjectFuture: futures::Future<
            Item = grpc::Response<object::InsertObjectResponse>,
            Error = grpc::Error,
        >;
        type GetWasmModuleInfoFuture: futures::Future<
            Item = grpc::Response<wasm::GetModuleInfoResponse>,
            Error = grpc::Error,
        >;
        type RegisterWasmModuleFuture: futures::Future<
            Item = grpc::Response<wasm::RegisterModuleResponse>,
            Error = grpc::Error,
        >;
        type RunWasmModuleFuture: futures::Future<
            Item = grpc::Response<wasm::RunModuleResponse>,
            Error = grpc::Error,
        >;

        fn create_database(
            &mut self,
            request: grpc::Request<database::CreateDatabaseRequest>,
        ) -> Self::CreateDatabaseFuture;

        fn list_databases(
            &mut self,
            request: grpc::Request<database::ListDatabasesRequest>,
        ) -> Self::ListDatabasesFuture;

        fn create_collection(
            &mut self,
            request: grpc::Request<collection::CreateCollectionRequest>,
        ) -> Self::CreateCollectionFuture;

        fn get_collection_info(
            &mut self,
            request: grpc::Request<collection::GetCollectionInfoRequest>,
        ) -> Self::GetCollectionInfoFuture;

        fn list_collections(
            &mut self,
            request: grpc::Request<collection::ListCollectionsRequest>,
        ) -> Self::ListCollectionsFuture;

        fn find_object(
            &mut self,
            request: grpc::Request<object::FindObjectRequest>,
        ) -> Self::FindObjectFuture;

        fn insert_object(
            &mut self,
            request: grpc::Request<object::InsertObjectRequest>,
        ) -> Self::InsertObjectFuture;

        fn get_wasm_module_info(
            &mut self,
            request: grpc::Request<wasm::GetModuleInfoRequest>,
        ) -> Self::GetWasmModuleInfoFuture;

        fn register_wasm_module(
            &mut self,
            request: grpc::Request<wasm::RegisterModuleRequest>,
        ) -> Self::RegisterWasmModuleFuture;

        fn run_wasm_module(
            &mut self,
            request: grpc::Request<wasm::RunModuleRequest>,
        ) -> Self::RunWasmModuleFuture;
    }

    #[derive(Debug, Clone)]
    pub struct ProtoDbServer<T> {
        proto_db: T,
    }

    impl<T> ProtoDbServer<T>
    where
        T: ProtoDb,
    {
        pub fn new(proto_db: T) -> Self {
            Self { proto_db }
        }
    }

    impl<T> tower::Service<http::Request<grpc::BoxBody>> for ProtoDbServer<T>
    where
        T: ProtoDb,
    {
        type Response = http::Response<proto_db::ResponseBody<T>>;
        type Error = h2::Error;
        type Future = proto_db::ResponseFuture<T>;

        fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
            Ok(().into())
        }

        fn call(&mut self, request: http::Request<grpc::BoxBody>) -> Self::Future {
            use self::proto_db::Kind::*;

            match request.uri().path() {
                "/protodb.ProtoDB/CreateDatabase" => {
                    let service = proto_db::methods::CreateDatabase(self.proto_db.clone());
                    let response = grpc::Grpc::unary(service, request);
                    proto_db::ResponseFuture {
                        kind: Ok(CreateDatabase(response)),
                    }
                }
                "/protodb.ProtoDB/ListDatabases" => {
                    let service = proto_db::methods::ListDatabases(self.proto_db.clone());
                    let response = grpc::Grpc::unary(service, request);
                    proto_db::ResponseFuture {
                        kind: Ok(ListDatabases(response)),
                    }
                }
                "/protodb.ProtoDB/CreateCollection" => {
                    let service = proto_db::methods::CreateCollection(self.proto_db.clone());
                    let response = grpc::Grpc::unary(service, request);
                    proto_db::ResponseFuture {
                        kind: Ok(CreateCollection(response)),
                    }
                }
                "/protodb.ProtoDB/GetCollectionInfo" => {
                    let service = proto_db::methods::GetCollectionInfo(self.proto_db.clone());
                    let response = grpc::Grpc::unary(service, request);
                    proto_db::ResponseFuture {
                        kind: Ok(GetCollectionInfo(response)),
                    }
                }
                "/protodb.ProtoDB/ListCollections" => {
                    let service = proto_db::methods::ListCollections(self.proto_db.clone());
                    let response = grpc::Grpc::unary(service, request);
                    proto_db::ResponseFuture {
                        kind: Ok(ListCollections(response)),
                    }
                }
                "/protodb.ProtoDB/FindObject" => {
                    let service = proto_db::methods::FindObject(self.proto_db.clone());
                    let response = grpc::Grpc::unary(service, request);
                    proto_db::ResponseFuture {
                        kind: Ok(FindObject(response)),
                    }
                }
                "/protodb.ProtoDB/InsertObject" => {
                    let service = proto_db::methods::InsertObject(self.proto_db.clone());
                    let response = grpc::Grpc::unary(service, request);
                    proto_db::ResponseFuture {
                        kind: Ok(InsertObject(response)),
                    }
                }
                "/protodb.ProtoDB/GetWasmModuleInfo" => {
                    let service = proto_db::methods::GetWasmModuleInfo(self.proto_db.clone());
                    let response = grpc::Grpc::unary(service, request);
                    proto_db::ResponseFuture {
                        kind: Ok(GetWasmModuleInfo(response)),
                    }
                }
                "/protodb.ProtoDB/RegisterWasmModule" => {
                    let service = proto_db::methods::RegisterWasmModule(self.proto_db.clone());
                    let response = grpc::Grpc::unary(service, request);
                    proto_db::ResponseFuture {
                        kind: Ok(RegisterWasmModule(response)),
                    }
                }
                "/protodb.ProtoDB/RunWasmModule" => {
                    let service = proto_db::methods::RunWasmModule(self.proto_db.clone());
                    let response = grpc::Grpc::unary(service, request);
                    proto_db::ResponseFuture {
                        kind: Ok(RunWasmModule(response)),
                    }
                }
                _ => proto_db::ResponseFuture {
                    kind: Err(grpc::Status::with_code(grpc::Code::Unimplemented)),
                },
            }
        }
    }

    impl<T> tower::Service<()> for ProtoDbServer<T>
    where
        T: ProtoDb,
    {
        type Response = Self;
        type Error = h2::Error;
        type Future = futures::FutureResult<Self::Response, Self::Error>;

        fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
            Ok(futures::Async::Ready(()))
        }

        fn call(&mut self, _target: ()) -> Self::Future {
            futures::ok(self.clone())
        }
    }

    impl<T> tower::Service<http::Request<tower_h2::RecvBody>> for ProtoDbServer<T>
    where
        T: ProtoDb,
    {
        type Response = <Self as tower::Service<http::Request<grpc::BoxBody>>>::Response;
        type Error = <Self as tower::Service<http::Request<grpc::BoxBody>>>::Error;
        type Future = <Self as tower::Service<http::Request<grpc::BoxBody>>>::Future;

        fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
            tower::Service::<http::Request<grpc::BoxBody>>::poll_ready(self)
        }

        fn call(&mut self, request: http::Request<tower_h2::RecvBody>) -> Self::Future {
            let request = request.map(|b| grpc::BoxBody::new(Box::new(b)));
            tower::Service::<http::Request<grpc::BoxBody>>::call(self, request)
        }
    }

    pub mod proto_db {
        use super::super::collection;
        use super::super::collection::{
            CreateCollectionRequest, GetCollectionInfoRequest, ListCollectionsRequest,
        };
        use super::super::database;
        use super::super::database::{CreateDatabaseRequest, ListDatabasesRequest};
        use super::super::object;
        use super::super::object::{FindObjectRequest, InsertObjectRequest};
        use super::super::wasm;
        use super::super::wasm::{GetModuleInfoRequest, RegisterModuleRequest, RunModuleRequest};
        use super::ProtoDb;
        use tower_grpc::codegen::server::*;

        pub struct ResponseFuture<T>
        where
            T: ProtoDb,
        {
            pub(super) kind: Result<
                Kind<
                    grpc::unary::ResponseFuture<
                        methods::CreateDatabase<T>,
                        grpc::BoxBody,
                        database::CreateDatabaseRequest,
                    >,
                    grpc::unary::ResponseFuture<
                        methods::ListDatabases<T>,
                        grpc::BoxBody,
                        database::ListDatabasesRequest,
                    >,
                    grpc::unary::ResponseFuture<
                        methods::CreateCollection<T>,
                        grpc::BoxBody,
                        collection::CreateCollectionRequest,
                    >,
                    grpc::unary::ResponseFuture<
                        methods::GetCollectionInfo<T>,
                        grpc::BoxBody,
                        collection::GetCollectionInfoRequest,
                    >,
                    grpc::unary::ResponseFuture<
                        methods::ListCollections<T>,
                        grpc::BoxBody,
                        collection::ListCollectionsRequest,
                    >,
                    grpc::unary::ResponseFuture<
                        methods::FindObject<T>,
                        grpc::BoxBody,
                        object::FindObjectRequest,
                    >,
                    grpc::unary::ResponseFuture<
                        methods::InsertObject<T>,
                        grpc::BoxBody,
                        object::InsertObjectRequest,
                    >,
                    grpc::unary::ResponseFuture<
                        methods::GetWasmModuleInfo<T>,
                        grpc::BoxBody,
                        wasm::GetModuleInfoRequest,
                    >,
                    grpc::unary::ResponseFuture<
                        methods::RegisterWasmModule<T>,
                        grpc::BoxBody,
                        wasm::RegisterModuleRequest,
                    >,
                    grpc::unary::ResponseFuture<
                        methods::RunWasmModule<T>,
                        grpc::BoxBody,
                        wasm::RunModuleRequest,
                    >,
                >,
                grpc::Status,
            >,
        }

        impl<T> futures::Future for ResponseFuture<T>
        where
            T: ProtoDb,
        {
            type Item = http::Response<ResponseBody<T>>;
            type Error = h2::Error;

            fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
                use self::Kind::*;

                match self.kind {
                    Ok(CreateDatabase(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody {
                            kind: Ok(CreateDatabase(body)),
                        };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Ok(ListDatabases(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody {
                            kind: Ok(ListDatabases(body)),
                        };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Ok(CreateCollection(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody {
                            kind: Ok(CreateCollection(body)),
                        };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Ok(GetCollectionInfo(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody {
                            kind: Ok(GetCollectionInfo(body)),
                        };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Ok(ListCollections(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody {
                            kind: Ok(ListCollections(body)),
                        };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Ok(FindObject(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody {
                            kind: Ok(FindObject(body)),
                        };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Ok(InsertObject(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody {
                            kind: Ok(InsertObject(body)),
                        };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Ok(GetWasmModuleInfo(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody {
                            kind: Ok(GetWasmModuleInfo(body)),
                        };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Ok(RegisterWasmModule(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody {
                            kind: Ok(RegisterWasmModule(body)),
                        };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Ok(RunWasmModule(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody {
                            kind: Ok(RunWasmModule(body)),
                        };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Err(ref status) => {
                        let body = ResponseBody {
                            kind: Err(status.clone()),
                        };
                        Ok(grpc::Response::new(body).into_http().into())
                    }
                }
            }
        }

        pub struct ResponseBody<T>
        where
            T: ProtoDb,
        {
            pub(super) kind: Result<
                Kind<
                    grpc::Encode<
                        grpc::unary::Once<
                            <methods::CreateDatabase<T> as grpc::UnaryService<
                                database::CreateDatabaseRequest,
                            >>::Response,
                        >,
                    >,
                    grpc::Encode<
                        grpc::unary::Once<
                            <methods::ListDatabases<T> as grpc::UnaryService<
                                database::ListDatabasesRequest,
                            >>::Response,
                        >,
                    >,
                    grpc::Encode<
                        grpc::unary::Once<
                            <methods::CreateCollection<T> as grpc::UnaryService<
                                collection::CreateCollectionRequest,
                            >>::Response,
                        >,
                    >,
                    grpc::Encode<
                        grpc::unary::Once<
                            <methods::GetCollectionInfo<T> as grpc::UnaryService<
                                collection::GetCollectionInfoRequest,
                            >>::Response,
                        >,
                    >,
                    grpc::Encode<
                        grpc::unary::Once<
                            <methods::ListCollections<T> as grpc::UnaryService<
                                collection::ListCollectionsRequest,
                            >>::Response,
                        >,
                    >,
                    grpc::Encode<
                        grpc::unary::Once<
                            <methods::FindObject<T> as grpc::UnaryService<
                                object::FindObjectRequest,
                            >>::Response,
                        >,
                    >,
                    grpc::Encode<
                        grpc::unary::Once<
                            <methods::InsertObject<T> as grpc::UnaryService<
                                object::InsertObjectRequest,
                            >>::Response,
                        >,
                    >,
                    grpc::Encode<
                        grpc::unary::Once<
                            <methods::GetWasmModuleInfo<T> as grpc::UnaryService<
                                wasm::GetModuleInfoRequest,
                            >>::Response,
                        >,
                    >,
                    grpc::Encode<
                        grpc::unary::Once<
                            <methods::RegisterWasmModule<T> as grpc::UnaryService<
                                wasm::RegisterModuleRequest,
                            >>::Response,
                        >,
                    >,
                    grpc::Encode<
                        grpc::unary::Once<
                            <methods::RunWasmModule<T> as grpc::UnaryService<
                                wasm::RunModuleRequest,
                            >>::Response,
                        >,
                    >,
                >,
                grpc::Status,
            >,
        }

        impl<T> grpc::Body for ResponseBody<T>
        where
            T: ProtoDb,
        {
            type Data = bytes::Bytes;

            fn is_end_stream(&self) -> bool {
                use self::Kind::*;

                match self.kind {
                    Ok(CreateDatabase(ref v)) => v.is_end_stream(),
                    Ok(ListDatabases(ref v)) => v.is_end_stream(),
                    Ok(CreateCollection(ref v)) => v.is_end_stream(),
                    Ok(GetCollectionInfo(ref v)) => v.is_end_stream(),
                    Ok(ListCollections(ref v)) => v.is_end_stream(),
                    Ok(FindObject(ref v)) => v.is_end_stream(),
                    Ok(InsertObject(ref v)) => v.is_end_stream(),
                    Ok(GetWasmModuleInfo(ref v)) => v.is_end_stream(),
                    Ok(RegisterWasmModule(ref v)) => v.is_end_stream(),
                    Ok(RunWasmModule(ref v)) => v.is_end_stream(),
                    Err(_) => true,
                }
            }

            fn poll_data(&mut self) -> futures::Poll<Option<Self::Data>, grpc::Error> {
                use self::Kind::*;

                match self.kind {
                    Ok(CreateDatabase(ref mut v)) => v.poll_data(),
                    Ok(ListDatabases(ref mut v)) => v.poll_data(),
                    Ok(CreateCollection(ref mut v)) => v.poll_data(),
                    Ok(GetCollectionInfo(ref mut v)) => v.poll_data(),
                    Ok(ListCollections(ref mut v)) => v.poll_data(),
                    Ok(FindObject(ref mut v)) => v.poll_data(),
                    Ok(InsertObject(ref mut v)) => v.poll_data(),
                    Ok(GetWasmModuleInfo(ref mut v)) => v.poll_data(),
                    Ok(RegisterWasmModule(ref mut v)) => v.poll_data(),
                    Ok(RunWasmModule(ref mut v)) => v.poll_data(),
                    Err(_) => Ok(None.into()),
                }
            }

            fn poll_metadata(&mut self) -> futures::Poll<Option<http::HeaderMap>, grpc::Error> {
                use self::Kind::*;

                match self.kind {
                    Ok(CreateDatabase(ref mut v)) => v.poll_metadata(),
                    Ok(ListDatabases(ref mut v)) => v.poll_metadata(),
                    Ok(CreateCollection(ref mut v)) => v.poll_metadata(),
                    Ok(GetCollectionInfo(ref mut v)) => v.poll_metadata(),
                    Ok(ListCollections(ref mut v)) => v.poll_metadata(),
                    Ok(FindObject(ref mut v)) => v.poll_metadata(),
                    Ok(InsertObject(ref mut v)) => v.poll_metadata(),
                    Ok(GetWasmModuleInfo(ref mut v)) => v.poll_metadata(),
                    Ok(RegisterWasmModule(ref mut v)) => v.poll_metadata(),
                    Ok(RunWasmModule(ref mut v)) => v.poll_metadata(),
                    Err(ref status) => status.to_header_map().map(Some).map(Into::into),
                }
            }
        }

        impl<T> tower_h2::Body for ResponseBody<T>
        where
            T: ProtoDb,
        {
            type Data = bytes::Bytes;

            fn is_end_stream(&self) -> bool {
                grpc::Body::is_end_stream(self)
            }

            fn poll_data(&mut self) -> futures::Poll<Option<Self::Data>, h2::Error> {
                grpc::Body::poll_data(self).map_err(h2::Error::from)
            }

            fn poll_trailers(&mut self) -> futures::Poll<Option<http::HeaderMap>, h2::Error> {
                grpc::Body::poll_metadata(self).map_err(h2::Error::from)
            }
        }

        #[derive(Debug, Clone)]
        pub(super) enum Kind<
            CreateDatabase,
            ListDatabases,
            CreateCollection,
            GetCollectionInfo,
            ListCollections,
            FindObject,
            InsertObject,
            GetWasmModuleInfo,
            RegisterWasmModule,
            RunWasmModule,
        > {
            CreateDatabase(CreateDatabase),
            ListDatabases(ListDatabases),
            CreateCollection(CreateCollection),
            GetCollectionInfo(GetCollectionInfo),
            ListCollections(ListCollections),
            FindObject(FindObject),
            InsertObject(InsertObject),
            GetWasmModuleInfo(GetWasmModuleInfo),
            RegisterWasmModule(RegisterWasmModule),
            RunWasmModule(RunWasmModule),
        }

        pub mod methods {
            use super::super::super::collection;
            use super::super::super::database;
            use super::super::super::object;
            use super::super::super::wasm;
            use super::super::collection::{
                CreateCollectionRequest, CreateCollectionResponse, GetCollectionInfoRequest,
                GetCollectionInfoResponse, ListCollectionsRequest, ListCollectionsResponse,
            };
            use super::super::database::{
                CreateDatabaseRequest, CreateDatabaseResponse, ListDatabasesRequest,
                ListDatabasesResponse,
            };
            use super::super::object::{
                FindObjectRequest, FindObjectResponse, InsertObjectRequest, InsertObjectResponse,
            };
            use super::super::wasm::{
                GetModuleInfoRequest, GetModuleInfoResponse, RegisterModuleRequest,
                RegisterModuleResponse, RunModuleRequest, RunModuleResponse,
            };
            use super::super::ProtoDb;
            use tower_grpc::codegen::server::*;

            pub struct CreateDatabase<T>(pub T);

            impl<T> tower::Service<grpc::Request<database::CreateDatabaseRequest>> for CreateDatabase<T>
            where
                T: ProtoDb,
            {
                type Response = grpc::Response<database::CreateDatabaseResponse>;
                type Error = grpc::Error;
                type Future = T::CreateDatabaseFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(
                    &mut self,
                    request: grpc::Request<database::CreateDatabaseRequest>,
                ) -> Self::Future {
                    self.0.create_database(request)
                }
            }

            pub struct ListDatabases<T>(pub T);

            impl<T> tower::Service<grpc::Request<database::ListDatabasesRequest>> for ListDatabases<T>
            where
                T: ProtoDb,
            {
                type Response = grpc::Response<database::ListDatabasesResponse>;
                type Error = grpc::Error;
                type Future = T::ListDatabasesFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(
                    &mut self,
                    request: grpc::Request<database::ListDatabasesRequest>,
                ) -> Self::Future {
                    self.0.list_databases(request)
                }
            }

            pub struct CreateCollection<T>(pub T);

            impl<T> tower::Service<grpc::Request<collection::CreateCollectionRequest>> for CreateCollection<T>
            where
                T: ProtoDb,
            {
                type Response = grpc::Response<collection::CreateCollectionResponse>;
                type Error = grpc::Error;
                type Future = T::CreateCollectionFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(
                    &mut self,
                    request: grpc::Request<collection::CreateCollectionRequest>,
                ) -> Self::Future {
                    self.0.create_collection(request)
                }
            }

            pub struct GetCollectionInfo<T>(pub T);

            impl<T> tower::Service<grpc::Request<collection::GetCollectionInfoRequest>> for GetCollectionInfo<T>
            where
                T: ProtoDb,
            {
                type Response = grpc::Response<collection::GetCollectionInfoResponse>;
                type Error = grpc::Error;
                type Future = T::GetCollectionInfoFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(
                    &mut self,
                    request: grpc::Request<collection::GetCollectionInfoRequest>,
                ) -> Self::Future {
                    self.0.get_collection_info(request)
                }
            }

            pub struct ListCollections<T>(pub T);

            impl<T> tower::Service<grpc::Request<collection::ListCollectionsRequest>> for ListCollections<T>
            where
                T: ProtoDb,
            {
                type Response = grpc::Response<collection::ListCollectionsResponse>;
                type Error = grpc::Error;
                type Future = T::ListCollectionsFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(
                    &mut self,
                    request: grpc::Request<collection::ListCollectionsRequest>,
                ) -> Self::Future {
                    self.0.list_collections(request)
                }
            }

            pub struct FindObject<T>(pub T);

            impl<T> tower::Service<grpc::Request<object::FindObjectRequest>> for FindObject<T>
            where
                T: ProtoDb,
            {
                type Response = grpc::Response<object::FindObjectResponse>;
                type Error = grpc::Error;
                type Future = T::FindObjectFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(
                    &mut self,
                    request: grpc::Request<object::FindObjectRequest>,
                ) -> Self::Future {
                    self.0.find_object(request)
                }
            }

            pub struct InsertObject<T>(pub T);

            impl<T> tower::Service<grpc::Request<object::InsertObjectRequest>> for InsertObject<T>
            where
                T: ProtoDb,
            {
                type Response = grpc::Response<object::InsertObjectResponse>;
                type Error = grpc::Error;
                type Future = T::InsertObjectFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(
                    &mut self,
                    request: grpc::Request<object::InsertObjectRequest>,
                ) -> Self::Future {
                    self.0.insert_object(request)
                }
            }

            pub struct GetWasmModuleInfo<T>(pub T);

            impl<T> tower::Service<grpc::Request<wasm::GetModuleInfoRequest>> for GetWasmModuleInfo<T>
            where
                T: ProtoDb,
            {
                type Response = grpc::Response<wasm::GetModuleInfoResponse>;
                type Error = grpc::Error;
                type Future = T::GetWasmModuleInfoFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(
                    &mut self,
                    request: grpc::Request<wasm::GetModuleInfoRequest>,
                ) -> Self::Future {
                    self.0.get_wasm_module_info(request)
                }
            }

            pub struct RegisterWasmModule<T>(pub T);

            impl<T> tower::Service<grpc::Request<wasm::RegisterModuleRequest>> for RegisterWasmModule<T>
            where
                T: ProtoDb,
            {
                type Response = grpc::Response<wasm::RegisterModuleResponse>;
                type Error = grpc::Error;
                type Future = T::RegisterWasmModuleFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(
                    &mut self,
                    request: grpc::Request<wasm::RegisterModuleRequest>,
                ) -> Self::Future {
                    self.0.register_wasm_module(request)
                }
            }

            pub struct RunWasmModule<T>(pub T);

            impl<T> tower::Service<grpc::Request<wasm::RunModuleRequest>> for RunWasmModule<T>
            where
                T: ProtoDb,
            {
                type Response = grpc::Response<wasm::RunModuleResponse>;
                type Error = grpc::Error;
                type Future = T::RunWasmModuleFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: grpc::Request<wasm::RunModuleRequest>) -> Self::Future {
                    self.0.run_wasm_module(request)
                }
            }
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
    pub struct GetCollectionInfoRequest {
        #[prost(string, tag = "1")]
        pub database: String,
        #[prost(string, tag = "2")]
        pub collection: String,
    }
    #[derive(Clone, PartialEq, Message)]
    pub struct GetCollectionInfoResponse {
        #[prost(enumeration = "get_collection_info_response::ErrorCode", tag = "1")]
        pub error_code: i32,
        #[prost(message, optional, tag = "2")]
        pub schema: ::std::option::Option<::prost_types::DescriptorProto>,
    }
    pub mod get_collection_info_response {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
        pub enum ErrorCode {
            NoError = 0,
            InternalError = 1,
            InvalidDatabase = 2,
            InvalidCollection = 3,
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
    pub struct GetModuleInfoRequest {
        #[prost(string, tag = "1")]
        pub database: String,
        #[prost(string, tag = "2")]
        pub name: String,
    }
    #[derive(Clone, PartialEq, Message)]
    pub struct GetModuleInfoResponse {
        #[prost(enumeration = "get_module_info_response::ErrorCode", tag = "1")]
        pub error_code: i32,
        #[prost(message, optional, tag = "2")]
        pub result_schema: ::std::option::Option<::prost_types::DescriptorProto>,
    }
    pub mod get_module_info_response {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
        pub enum ErrorCode {
            NoError = 0,
            InternalError = 1,
            InvalidDatabase = 2,
            InvalidModule = 3,
        }
    }
    #[derive(Clone, PartialEq, Message)]
    pub struct RegisterModuleRequest {
        #[prost(string, tag = "1")]
        pub database: String,
        #[prost(string, tag = "2")]
        pub name: String,
        #[prost(message, optional, tag = "3")]
        pub metadata: ::std::option::Option<register_module_request::ModuleMetadata>,
        #[prost(bytes, tag = "4")]
        pub wasm: Vec<u8>,
        #[prost(message, optional, tag = "5")]
        pub result_schema: ::std::option::Option<::prost_types::DescriptorProto>,
    }
    pub mod register_module_request {
        #[derive(Clone, PartialEq, Message)]
        pub struct ModuleMetadata {
            #[prost(string, tag = "1")]
            pub name: String,
            #[prost(message, optional, tag = "2")]
            pub bindgen_import_hashes: ::std::option::Option<module_metadata::BindgenImportHashes>,
        }
        pub mod module_metadata {
            #[derive(Clone, PartialEq, Message)]
            pub struct BindgenImportHashes {
                #[prost(string, tag = "1")]
                pub log: String,
                #[prost(string, tag = "2")]
                pub find_object: String,
                #[prost(string, tag = "3")]
                pub find_objects_iter: String,
                #[prost(string, tag = "4")]
                pub find_objects_iter_next: String,
            }
        }
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

// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait ProtoDB {
    fn CreateDatabase(&self, p: super::database_create::CreateDatabase) -> ::grpc::result::GrpcResult<super::database_create::CreateDatabaseResponse>;

    fn DeleteDatabase(&self, p: super::database_delete::DeleteDatabase) -> ::grpc::result::GrpcResult<super::database_delete::DeleteDatabaseResponse>;

    fn ListDatabases(&self, p: super::database_list::ListDatabases) -> ::grpc::result::GrpcResult<super::database_list::ListDatabasesResponse>;

    fn DefineDatabase(&self, p: super::database_define::DefineDatabase) -> ::grpc::result::GrpcResult<super::database_define::DefineDatabaseResponse>;

    fn CreateCollection(&self, p: super::collection_create::CreateCollection) -> ::grpc::result::GrpcResult<super::collection_create::CreateCollectionResponse>;

    fn DeleteCollection(&self, p: super::collection_delete::DeleteCollection) -> ::grpc::result::GrpcResult<super::collection_delete::DeleteCollectionResponse>;

    fn ListCollections(&self, p: super::collection_list::ListCollections) -> ::grpc::result::GrpcResult<super::collection_list::ListCollectionsResponse>;

    fn DefineCollection(&self, p: super::collection_define::DefineCollection) -> ::grpc::result::GrpcResult<super::collection_define::DefineCollectionResponse>;

    fn Find(&self, p: super::find::Find) -> ::grpc::result::GrpcResult<super::find::FindResponse>;

    fn Insert(&self, p: super::insert::Insert) -> ::grpc::result::GrpcResult<super::insert::InsertResponse>;
}

pub trait ProtoDBAsync {
    fn CreateDatabase(&self, p: super::database_create::CreateDatabase) -> ::grpc::futures_grpc::GrpcFuture<super::database_create::CreateDatabaseResponse>;

    fn DeleteDatabase(&self, p: super::database_delete::DeleteDatabase) -> ::grpc::futures_grpc::GrpcFuture<super::database_delete::DeleteDatabaseResponse>;

    fn ListDatabases(&self, p: super::database_list::ListDatabases) -> ::grpc::futures_grpc::GrpcFuture<super::database_list::ListDatabasesResponse>;

    fn DefineDatabase(&self, p: super::database_define::DefineDatabase) -> ::grpc::futures_grpc::GrpcFuture<super::database_define::DefineDatabaseResponse>;

    fn CreateCollection(&self, p: super::collection_create::CreateCollection) -> ::grpc::futures_grpc::GrpcFuture<super::collection_create::CreateCollectionResponse>;

    fn DeleteCollection(&self, p: super::collection_delete::DeleteCollection) -> ::grpc::futures_grpc::GrpcFuture<super::collection_delete::DeleteCollectionResponse>;

    fn ListCollections(&self, p: super::collection_list::ListCollections) -> ::grpc::futures_grpc::GrpcFuture<super::collection_list::ListCollectionsResponse>;

    fn DefineCollection(&self, p: super::collection_define::DefineCollection) -> ::grpc::futures_grpc::GrpcFuture<super::collection_define::DefineCollectionResponse>;

    fn Find(&self, p: super::find::Find) -> ::grpc::futures_grpc::GrpcFuture<super::find::FindResponse>;

    fn Insert(&self, p: super::insert::Insert) -> ::grpc::futures_grpc::GrpcFuture<super::insert::InsertResponse>;
}

// sync client

pub struct ProtoDBClient {
    async_client: ProtoDBAsyncClient,
}

impl ProtoDBClient {
    pub fn new(host: &str, port: u16) -> ::grpc::result::GrpcResult<Self> {
        ProtoDBAsyncClient::new(host, port).map(|c| {
            ProtoDBClient {
                async_client: c,
            }
        })
    }
}

impl ProtoDB for ProtoDBClient {
    fn CreateDatabase(&self, p: super::database_create::CreateDatabase) -> ::grpc::result::GrpcResult<super::database_create::CreateDatabaseResponse> {
        ::futures::Future::wait(self.async_client.CreateDatabase(p))
    }

    fn DeleteDatabase(&self, p: super::database_delete::DeleteDatabase) -> ::grpc::result::GrpcResult<super::database_delete::DeleteDatabaseResponse> {
        ::futures::Future::wait(self.async_client.DeleteDatabase(p))
    }

    fn ListDatabases(&self, p: super::database_list::ListDatabases) -> ::grpc::result::GrpcResult<super::database_list::ListDatabasesResponse> {
        ::futures::Future::wait(self.async_client.ListDatabases(p))
    }

    fn DefineDatabase(&self, p: super::database_define::DefineDatabase) -> ::grpc::result::GrpcResult<super::database_define::DefineDatabaseResponse> {
        ::futures::Future::wait(self.async_client.DefineDatabase(p))
    }

    fn CreateCollection(&self, p: super::collection_create::CreateCollection) -> ::grpc::result::GrpcResult<super::collection_create::CreateCollectionResponse> {
        ::futures::Future::wait(self.async_client.CreateCollection(p))
    }

    fn DeleteCollection(&self, p: super::collection_delete::DeleteCollection) -> ::grpc::result::GrpcResult<super::collection_delete::DeleteCollectionResponse> {
        ::futures::Future::wait(self.async_client.DeleteCollection(p))
    }

    fn ListCollections(&self, p: super::collection_list::ListCollections) -> ::grpc::result::GrpcResult<super::collection_list::ListCollectionsResponse> {
        ::futures::Future::wait(self.async_client.ListCollections(p))
    }

    fn DefineCollection(&self, p: super::collection_define::DefineCollection) -> ::grpc::result::GrpcResult<super::collection_define::DefineCollectionResponse> {
        ::futures::Future::wait(self.async_client.DefineCollection(p))
    }

    fn Find(&self, p: super::find::Find) -> ::grpc::result::GrpcResult<super::find::FindResponse> {
        ::futures::Future::wait(self.async_client.Find(p))
    }

    fn Insert(&self, p: super::insert::Insert) -> ::grpc::result::GrpcResult<super::insert::InsertResponse> {
        ::futures::Future::wait(self.async_client.Insert(p))
    }
}

// async client

pub struct ProtoDBAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_CreateDatabase: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::database_create::CreateDatabase, super::database_create::CreateDatabaseResponse>>,
    method_DeleteDatabase: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::database_delete::DeleteDatabase, super::database_delete::DeleteDatabaseResponse>>,
    method_ListDatabases: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::database_list::ListDatabases, super::database_list::ListDatabasesResponse>>,
    method_DefineDatabase: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::database_define::DefineDatabase, super::database_define::DefineDatabaseResponse>>,
    method_CreateCollection: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::collection_create::CreateCollection, super::collection_create::CreateCollectionResponse>>,
    method_DeleteCollection: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::collection_delete::DeleteCollection, super::collection_delete::DeleteCollectionResponse>>,
    method_ListCollections: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::collection_list::ListCollections, super::collection_list::ListCollectionsResponse>>,
    method_DefineCollection: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::collection_define::DefineCollection, super::collection_define::DefineCollectionResponse>>,
    method_Find: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::find::Find, super::find::FindResponse>>,
    method_Insert: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::insert::Insert, super::insert::InsertResponse>>,
}

impl ProtoDBAsyncClient {
    pub fn new(host: &str, port: u16) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port).map(|c| {
            ProtoDBAsyncClient {
                grpc_client: c,
                method_CreateDatabase: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/protodb.server.ProtoDB/CreateDatabase".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_DeleteDatabase: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/protodb.server.ProtoDB/DeleteDatabase".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_ListDatabases: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/protodb.server.ProtoDB/ListDatabases".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_DefineDatabase: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/protodb.server.ProtoDB/DefineDatabase".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_CreateCollection: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/protodb.server.ProtoDB/CreateCollection".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_DeleteCollection: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/protodb.server.ProtoDB/DeleteCollection".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_ListCollections: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/protodb.server.ProtoDB/ListCollections".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_DefineCollection: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/protodb.server.ProtoDB/DefineCollection".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Find: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/protodb.server.ProtoDB/Find".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Insert: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/protodb.server.ProtoDB/Insert".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl ProtoDBAsync for ProtoDBAsyncClient {
    fn CreateDatabase(&self, p: super::database_create::CreateDatabase) -> ::grpc::futures_grpc::GrpcFuture<super::database_create::CreateDatabaseResponse> {
        self.grpc_client.call_unary(p, self.method_CreateDatabase.clone())
    }

    fn DeleteDatabase(&self, p: super::database_delete::DeleteDatabase) -> ::grpc::futures_grpc::GrpcFuture<super::database_delete::DeleteDatabaseResponse> {
        self.grpc_client.call_unary(p, self.method_DeleteDatabase.clone())
    }

    fn ListDatabases(&self, p: super::database_list::ListDatabases) -> ::grpc::futures_grpc::GrpcFuture<super::database_list::ListDatabasesResponse> {
        self.grpc_client.call_unary(p, self.method_ListDatabases.clone())
    }

    fn DefineDatabase(&self, p: super::database_define::DefineDatabase) -> ::grpc::futures_grpc::GrpcFuture<super::database_define::DefineDatabaseResponse> {
        self.grpc_client.call_unary(p, self.method_DefineDatabase.clone())
    }

    fn CreateCollection(&self, p: super::collection_create::CreateCollection) -> ::grpc::futures_grpc::GrpcFuture<super::collection_create::CreateCollectionResponse> {
        self.grpc_client.call_unary(p, self.method_CreateCollection.clone())
    }

    fn DeleteCollection(&self, p: super::collection_delete::DeleteCollection) -> ::grpc::futures_grpc::GrpcFuture<super::collection_delete::DeleteCollectionResponse> {
        self.grpc_client.call_unary(p, self.method_DeleteCollection.clone())
    }

    fn ListCollections(&self, p: super::collection_list::ListCollections) -> ::grpc::futures_grpc::GrpcFuture<super::collection_list::ListCollectionsResponse> {
        self.grpc_client.call_unary(p, self.method_ListCollections.clone())
    }

    fn DefineCollection(&self, p: super::collection_define::DefineCollection) -> ::grpc::futures_grpc::GrpcFuture<super::collection_define::DefineCollectionResponse> {
        self.grpc_client.call_unary(p, self.method_DefineCollection.clone())
    }

    fn Find(&self, p: super::find::Find) -> ::grpc::futures_grpc::GrpcFuture<super::find::FindResponse> {
        self.grpc_client.call_unary(p, self.method_Find.clone())
    }

    fn Insert(&self, p: super::insert::Insert) -> ::grpc::futures_grpc::GrpcFuture<super::insert::InsertResponse> {
        self.grpc_client.call_unary(p, self.method_Insert.clone())
    }
}

// sync server

pub struct ProtoDBServer {
    async_server: ProtoDBAsyncServer,
}

struct ProtoDBServerHandlerToAsync {
    handler: ::std::sync::Arc<ProtoDB + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl ProtoDBAsync for ProtoDBServerHandlerToAsync {
    fn CreateDatabase(&self, p: super::database_create::CreateDatabase) -> ::grpc::futures_grpc::GrpcFuture<super::database_create::CreateDatabaseResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.CreateDatabase(p)
        })
    }

    fn DeleteDatabase(&self, p: super::database_delete::DeleteDatabase) -> ::grpc::futures_grpc::GrpcFuture<super::database_delete::DeleteDatabaseResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.DeleteDatabase(p)
        })
    }

    fn ListDatabases(&self, p: super::database_list::ListDatabases) -> ::grpc::futures_grpc::GrpcFuture<super::database_list::ListDatabasesResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.ListDatabases(p)
        })
    }

    fn DefineDatabase(&self, p: super::database_define::DefineDatabase) -> ::grpc::futures_grpc::GrpcFuture<super::database_define::DefineDatabaseResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.DefineDatabase(p)
        })
    }

    fn CreateCollection(&self, p: super::collection_create::CreateCollection) -> ::grpc::futures_grpc::GrpcFuture<super::collection_create::CreateCollectionResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.CreateCollection(p)
        })
    }

    fn DeleteCollection(&self, p: super::collection_delete::DeleteCollection) -> ::grpc::futures_grpc::GrpcFuture<super::collection_delete::DeleteCollectionResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.DeleteCollection(p)
        })
    }

    fn ListCollections(&self, p: super::collection_list::ListCollections) -> ::grpc::futures_grpc::GrpcFuture<super::collection_list::ListCollectionsResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.ListCollections(p)
        })
    }

    fn DefineCollection(&self, p: super::collection_define::DefineCollection) -> ::grpc::futures_grpc::GrpcFuture<super::collection_define::DefineCollectionResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.DefineCollection(p)
        })
    }

    fn Find(&self, p: super::find::Find) -> ::grpc::futures_grpc::GrpcFuture<super::find::FindResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Find(p)
        })
    }

    fn Insert(&self, p: super::insert::Insert) -> ::grpc::futures_grpc::GrpcFuture<super::insert::InsertResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Insert(p)
        })
    }
}

impl ProtoDBServer {
    pub fn new<H : ProtoDB + Send + Sync + 'static>(port: u16, h: H) -> Self {
        let h = ProtoDBServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        ProtoDBServer {
            async_server: ProtoDBAsyncServer::new(port, h),
        }
    }
}

// async server

pub struct ProtoDBAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl ProtoDBAsyncServer {
    pub fn new<H : ProtoDBAsync + 'static + Sync + Send + 'static>(port: u16, h: H) -> Self {
        let handler_arc = ::std::sync::Arc::new(h);
        let service_definition = ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/protodb.server.ProtoDB/CreateDatabase".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.CreateDatabase(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/protodb.server.ProtoDB/DeleteDatabase".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.DeleteDatabase(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/protodb.server.ProtoDB/ListDatabases".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.ListDatabases(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/protodb.server.ProtoDB/DefineDatabase".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.DefineDatabase(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/protodb.server.ProtoDB/CreateCollection".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.CreateCollection(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/protodb.server.ProtoDB/DeleteCollection".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.DeleteCollection(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/protodb.server.ProtoDB/ListCollections".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.ListCollections(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/protodb.server.ProtoDB/DefineCollection".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.DefineCollection(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/protodb.server.ProtoDB/Find".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Find(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/protodb.server.ProtoDB/Insert".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Insert(p))
                    },
                ),
            ],
        );
        ProtoDBAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(port, service_definition),
        }
    }
}

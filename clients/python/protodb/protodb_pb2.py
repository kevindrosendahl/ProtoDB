# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: protodb/protodb.proto

import sys
_b=sys.version_info[0]<3 and (lambda x:x) or (lambda x:x.encode('latin1'))
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
from google.protobuf import descriptor_pb2
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from protodb.collection import create_pb2 as protodb_dot_collection_dot_create__pb2
from protodb.collection import get_info_pb2 as protodb_dot_collection_dot_get__info__pb2
from protodb.collection import list_pb2 as protodb_dot_collection_dot_list__pb2
from protodb.database import create_pb2 as protodb_dot_database_dot_create__pb2
from protodb.database import list_pb2 as protodb_dot_database_dot_list__pb2
from protodb.index import create_pb2 as protodb_dot_index_dot_create__pb2
from protodb.index import get_pb2 as protodb_dot_index_dot_get__pb2
from protodb.index import list_pb2 as protodb_dot_index_dot_list__pb2
from protodb.object import find_pb2 as protodb_dot_object_dot_find__pb2
from protodb.object import insert_pb2 as protodb_dot_object_dot_insert__pb2
from protodb.wasm import get_info_pb2 as protodb_dot_wasm_dot_get__info__pb2
from protodb.wasm import register_pb2 as protodb_dot_wasm_dot_register__pb2
from protodb.wasm import run_pb2 as protodb_dot_wasm_dot_run__pb2


DESCRIPTOR = _descriptor.FileDescriptor(
  name='protodb/protodb.proto',
  package='protodb',
  syntax='proto3',
  serialized_pb=_b('\n\x15protodb/protodb.proto\x12\x07protodb\x1a\x1fprotodb/collection/create.proto\x1a!protodb/collection/get_info.proto\x1a\x1dprotodb/collection/list.proto\x1a\x1dprotodb/database/create.proto\x1a\x1bprotodb/database/list.proto\x1a\x1aprotodb/index/create.proto\x1a\x17protodb/index/get.proto\x1a\x18protodb/index/list.proto\x1a\x19protodb/object/find.proto\x1a\x1bprotodb/object/insert.proto\x1a\x1bprotodb/wasm/get_info.proto\x1a\x1bprotodb/wasm/register.proto\x1a\x16protodb/wasm/run.proto2\xd7\t\n\x07ProtoDB\x12m\n\x10\x43reateCollection\x12+.protodb.collection.CreateCollectionRequest\x1a,.protodb.collection.CreateCollectionResponse\x12p\n\x11GetCollectionInfo\x12,.protodb.collection.GetCollectionInfoRequest\x1a-.protodb.collection.GetCollectionInfoResponse\x12j\n\x0fListCollections\x12*.protodb.collection.ListCollectionsRequest\x1a+.protodb.collection.ListCollectionsResponse\x12\x63\n\x0e\x43reateDatabase\x12\'.protodb.database.CreateDatabaseRequest\x1a(.protodb.database.CreateDatabaseResponse\x12`\n\rListDatabases\x12&.protodb.database.ListDatabasesRequest\x1a\'.protodb.database.ListDatabasesResponse\x12T\n\x0b\x43reateIndex\x12!.protodb.index.CreateIndexRequest\x1a\".protodb.index.CreateIndexResponse\x12K\n\x08GetIndex\x12\x1e.protodb.index.GetIndexRequest\x1a\x1f.protodb.index.GetIndexResponse\x12T\n\x0bListIndexes\x12!.protodb.index.ListIndexesRequest\x1a\".protodb.index.ListIndexesResponse\x12S\n\nFindObject\x12!.protodb.object.FindObjectRequest\x1a\".protodb.object.FindObjectResponse\x12Y\n\x0cInsertObject\x12#.protodb.object.InsertObjectRequest\x1a$.protodb.object.InsertObjectResponse\x12\\\n\x11GetWasmModuleInfo\x12\".protodb.wasm.GetModuleInfoRequest\x1a#.protodb.wasm.GetModuleInfoResponse\x12_\n\x12RegisterWasmModule\x12#.protodb.wasm.RegisterModuleRequest\x1a$.protodb.wasm.RegisterModuleResponse\x12P\n\rRunWasmModule\x12\x1e.protodb.wasm.RunModuleRequest\x1a\x1f.protodb.wasm.RunModuleResponseb\x06proto3')
  ,
  dependencies=[protodb_dot_collection_dot_create__pb2.DESCRIPTOR,protodb_dot_collection_dot_get__info__pb2.DESCRIPTOR,protodb_dot_collection_dot_list__pb2.DESCRIPTOR,protodb_dot_database_dot_create__pb2.DESCRIPTOR,protodb_dot_database_dot_list__pb2.DESCRIPTOR,protodb_dot_index_dot_create__pb2.DESCRIPTOR,protodb_dot_index_dot_get__pb2.DESCRIPTOR,protodb_dot_index_dot_list__pb2.DESCRIPTOR,protodb_dot_object_dot_find__pb2.DESCRIPTOR,protodb_dot_object_dot_insert__pb2.DESCRIPTOR,protodb_dot_wasm_dot_get__info__pb2.DESCRIPTOR,protodb_dot_wasm_dot_register__pb2.DESCRIPTOR,protodb_dot_wasm_dot_run__pb2.DESCRIPTOR,])



_sym_db.RegisterFileDescriptor(DESCRIPTOR)



_PROTODB = _descriptor.ServiceDescriptor(
  name='ProtoDB',
  full_name='protodb.ProtoDB',
  file=DESCRIPTOR,
  index=0,
  options=None,
  serialized_start=411,
  serialized_end=1650,
  methods=[
  _descriptor.MethodDescriptor(
    name='CreateCollection',
    full_name='protodb.ProtoDB.CreateCollection',
    index=0,
    containing_service=None,
    input_type=protodb_dot_collection_dot_create__pb2._CREATECOLLECTIONREQUEST,
    output_type=protodb_dot_collection_dot_create__pb2._CREATECOLLECTIONRESPONSE,
    options=None,
  ),
  _descriptor.MethodDescriptor(
    name='GetCollectionInfo',
    full_name='protodb.ProtoDB.GetCollectionInfo',
    index=1,
    containing_service=None,
    input_type=protodb_dot_collection_dot_get__info__pb2._GETCOLLECTIONINFOREQUEST,
    output_type=protodb_dot_collection_dot_get__info__pb2._GETCOLLECTIONINFORESPONSE,
    options=None,
  ),
  _descriptor.MethodDescriptor(
    name='ListCollections',
    full_name='protodb.ProtoDB.ListCollections',
    index=2,
    containing_service=None,
    input_type=protodb_dot_collection_dot_list__pb2._LISTCOLLECTIONSREQUEST,
    output_type=protodb_dot_collection_dot_list__pb2._LISTCOLLECTIONSRESPONSE,
    options=None,
  ),
  _descriptor.MethodDescriptor(
    name='CreateDatabase',
    full_name='protodb.ProtoDB.CreateDatabase',
    index=3,
    containing_service=None,
    input_type=protodb_dot_database_dot_create__pb2._CREATEDATABASEREQUEST,
    output_type=protodb_dot_database_dot_create__pb2._CREATEDATABASERESPONSE,
    options=None,
  ),
  _descriptor.MethodDescriptor(
    name='ListDatabases',
    full_name='protodb.ProtoDB.ListDatabases',
    index=4,
    containing_service=None,
    input_type=protodb_dot_database_dot_list__pb2._LISTDATABASESREQUEST,
    output_type=protodb_dot_database_dot_list__pb2._LISTDATABASESRESPONSE,
    options=None,
  ),
  _descriptor.MethodDescriptor(
    name='CreateIndex',
    full_name='protodb.ProtoDB.CreateIndex',
    index=5,
    containing_service=None,
    input_type=protodb_dot_index_dot_create__pb2._CREATEINDEXREQUEST,
    output_type=protodb_dot_index_dot_create__pb2._CREATEINDEXRESPONSE,
    options=None,
  ),
  _descriptor.MethodDescriptor(
    name='GetIndex',
    full_name='protodb.ProtoDB.GetIndex',
    index=6,
    containing_service=None,
    input_type=protodb_dot_index_dot_get__pb2._GETINDEXREQUEST,
    output_type=protodb_dot_index_dot_get__pb2._GETINDEXRESPONSE,
    options=None,
  ),
  _descriptor.MethodDescriptor(
    name='ListIndexes',
    full_name='protodb.ProtoDB.ListIndexes',
    index=7,
    containing_service=None,
    input_type=protodb_dot_index_dot_list__pb2._LISTINDEXESREQUEST,
    output_type=protodb_dot_index_dot_list__pb2._LISTINDEXESRESPONSE,
    options=None,
  ),
  _descriptor.MethodDescriptor(
    name='FindObject',
    full_name='protodb.ProtoDB.FindObject',
    index=8,
    containing_service=None,
    input_type=protodb_dot_object_dot_find__pb2._FINDOBJECTREQUEST,
    output_type=protodb_dot_object_dot_find__pb2._FINDOBJECTRESPONSE,
    options=None,
  ),
  _descriptor.MethodDescriptor(
    name='InsertObject',
    full_name='protodb.ProtoDB.InsertObject',
    index=9,
    containing_service=None,
    input_type=protodb_dot_object_dot_insert__pb2._INSERTOBJECTREQUEST,
    output_type=protodb_dot_object_dot_insert__pb2._INSERTOBJECTRESPONSE,
    options=None,
  ),
  _descriptor.MethodDescriptor(
    name='GetWasmModuleInfo',
    full_name='protodb.ProtoDB.GetWasmModuleInfo',
    index=10,
    containing_service=None,
    input_type=protodb_dot_wasm_dot_get__info__pb2._GETMODULEINFOREQUEST,
    output_type=protodb_dot_wasm_dot_get__info__pb2._GETMODULEINFORESPONSE,
    options=None,
  ),
  _descriptor.MethodDescriptor(
    name='RegisterWasmModule',
    full_name='protodb.ProtoDB.RegisterWasmModule',
    index=11,
    containing_service=None,
    input_type=protodb_dot_wasm_dot_register__pb2._REGISTERMODULEREQUEST,
    output_type=protodb_dot_wasm_dot_register__pb2._REGISTERMODULERESPONSE,
    options=None,
  ),
  _descriptor.MethodDescriptor(
    name='RunWasmModule',
    full_name='protodb.ProtoDB.RunWasmModule',
    index=12,
    containing_service=None,
    input_type=protodb_dot_wasm_dot_run__pb2._RUNMODULEREQUEST,
    output_type=protodb_dot_wasm_dot_run__pb2._RUNMODULERESPONSE,
    options=None,
  ),
])
_sym_db.RegisterServiceDescriptor(_PROTODB)

DESCRIPTOR.services_by_name['ProtoDB'] = _PROTODB

# @@protoc_insertion_point(module_scope)

# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: protod/protod.proto

import sys
_b=sys.version_info[0]<3 and (lambda x:x) or (lambda x:x.encode('latin1'))
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
from google.protobuf import descriptor_pb2
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from protod import database_create_pb2 as protod_dot_database__create__pb2


DESCRIPTOR = _descriptor.FileDescriptor(
  name='protod/protod.proto',
  package='protodb.protod',
  syntax='proto3',
  serialized_pb=_b('\n\x13protod/protod.proto\x12\x0eprotodb.protod\x1a\x1cprotod/database_create.proto2i\n\x06ProtoD\x12_\n\x0e\x43reateDatabase\x12%.protodb.protod.CreateDatabaseRequest\x1a&.protodb.protod.CreateDatabaseResponseb\x06proto3')
  ,
  dependencies=[protod_dot_database__create__pb2.DESCRIPTOR,])



_sym_db.RegisterFileDescriptor(DESCRIPTOR)



_PROTOD = _descriptor.ServiceDescriptor(
  name='ProtoD',
  full_name='protodb.protod.ProtoD',
  file=DESCRIPTOR,
  index=0,
  options=None,
  serialized_start=69,
  serialized_end=174,
  methods=[
  _descriptor.MethodDescriptor(
    name='CreateDatabase',
    full_name='protodb.protod.ProtoD.CreateDatabase',
    index=0,
    containing_service=None,
    input_type=protod_dot_database__create__pb2._CREATEDATABASEREQUEST,
    output_type=protod_dot_database__create__pb2._CREATEDATABASERESPONSE,
    options=None,
  ),
])
_sym_db.RegisterServiceDescriptor(_PROTOD)

DESCRIPTOR.services_by_name['ProtoD'] = _PROTOD

# @@protoc_insertion_point(module_scope)

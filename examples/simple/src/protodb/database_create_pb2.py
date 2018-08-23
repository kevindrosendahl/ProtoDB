# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: protodb/database_create.proto

import sys
_b=sys.version_info[0]<3 and (lambda x:x) or (lambda x:x.encode('latin1'))
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
from google.protobuf import descriptor_pb2
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='protodb/database_create.proto',
  package='protodb',
  syntax='proto3',
  serialized_pb=_b('\n\x1dprotodb/database_create.proto\x12\x07protodb\"%\n\x15\x43reateDatabaseRequest\x12\x0c\n\x04name\x18\x01 \x01(\t\"\xa8\x01\n\x16\x43reateDatabaseResponse\x12\x0f\n\x07success\x18\x01 \x01(\x08\x12\x41\n\x0c\x66\x61ilure_code\x18\x02 \x01(\x0e\x32+.protodb.CreateDatabaseResponse.FailureCode\":\n\x0b\x46\x61ilureCode\x12\x0c\n\x08NO_ERROR\x10\x00\x12\x0e\n\nINVALID_DB\x10\x01\x12\r\n\tDB_EXISTS\x10\x02\x62\x06proto3')
)



_CREATEDATABASERESPONSE_FAILURECODE = _descriptor.EnumDescriptor(
  name='FailureCode',
  full_name='protodb.CreateDatabaseResponse.FailureCode',
  filename=None,
  file=DESCRIPTOR,
  values=[
    _descriptor.EnumValueDescriptor(
      name='NO_ERROR', index=0, number=0,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='INVALID_DB', index=1, number=1,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='DB_EXISTS', index=2, number=2,
      options=None,
      type=None),
  ],
  containing_type=None,
  options=None,
  serialized_start=192,
  serialized_end=250,
)
_sym_db.RegisterEnumDescriptor(_CREATEDATABASERESPONSE_FAILURECODE)


_CREATEDATABASEREQUEST = _descriptor.Descriptor(
  name='CreateDatabaseRequest',
  full_name='protodb.CreateDatabaseRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='name', full_name='protodb.CreateDatabaseRequest.name', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=42,
  serialized_end=79,
)


_CREATEDATABASERESPONSE = _descriptor.Descriptor(
  name='CreateDatabaseResponse',
  full_name='protodb.CreateDatabaseResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='success', full_name='protodb.CreateDatabaseResponse.success', index=0,
      number=1, type=8, cpp_type=7, label=1,
      has_default_value=False, default_value=False,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='failure_code', full_name='protodb.CreateDatabaseResponse.failure_code', index=1,
      number=2, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
    _CREATEDATABASERESPONSE_FAILURECODE,
  ],
  options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=82,
  serialized_end=250,
)

_CREATEDATABASERESPONSE.fields_by_name['failure_code'].enum_type = _CREATEDATABASERESPONSE_FAILURECODE
_CREATEDATABASERESPONSE_FAILURECODE.containing_type = _CREATEDATABASERESPONSE
DESCRIPTOR.message_types_by_name['CreateDatabaseRequest'] = _CREATEDATABASEREQUEST
DESCRIPTOR.message_types_by_name['CreateDatabaseResponse'] = _CREATEDATABASERESPONSE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

CreateDatabaseRequest = _reflection.GeneratedProtocolMessageType('CreateDatabaseRequest', (_message.Message,), dict(
  DESCRIPTOR = _CREATEDATABASEREQUEST,
  __module__ = 'protodb.database_create_pb2'
  # @@protoc_insertion_point(class_scope:protodb.CreateDatabaseRequest)
  ))
_sym_db.RegisterMessage(CreateDatabaseRequest)

CreateDatabaseResponse = _reflection.GeneratedProtocolMessageType('CreateDatabaseResponse', (_message.Message,), dict(
  DESCRIPTOR = _CREATEDATABASERESPONSE,
  __module__ = 'protodb.database_create_pb2'
  # @@protoc_insertion_point(class_scope:protodb.CreateDatabaseResponse)
  ))
_sym_db.RegisterMessage(CreateDatabaseResponse)


# @@protoc_insertion_point(module_scope)
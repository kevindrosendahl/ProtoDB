# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: protodb/database/create.proto

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
  name='protodb/database/create.proto',
  package='protodb.database',
  syntax='proto3',
  serialized_pb=_b('\n\x1dprotodb/database/create.proto\x12\x10protodb.database\"%\n\x15\x43reateDatabaseRequest\x12\x0c\n\x04name\x18\x01 \x01(\t\"\xa4\x01\n\x16\x43reateDatabaseResponse\x12\x46\n\nerror_code\x18\x01 \x01(\x0e\x32\x32.protodb.database.CreateDatabaseResponse.ErrorCode\"B\n\tErrorCode\x12\x0c\n\x08NO_ERROR\x10\x00\x12\x12\n\x0eINTERNAL_ERROR\x10\x01\x12\x13\n\x0f\x44\x41TABASE_EXISTS\x10\x02\x62\x06proto3')
)



_CREATEDATABASERESPONSE_ERRORCODE = _descriptor.EnumDescriptor(
  name='ErrorCode',
  full_name='protodb.database.CreateDatabaseResponse.ErrorCode',
  filename=None,
  file=DESCRIPTOR,
  values=[
    _descriptor.EnumValueDescriptor(
      name='NO_ERROR', index=0, number=0,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='INTERNAL_ERROR', index=1, number=1,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='DATABASE_EXISTS', index=2, number=2,
      options=None,
      type=None),
  ],
  containing_type=None,
  options=None,
  serialized_start=189,
  serialized_end=255,
)
_sym_db.RegisterEnumDescriptor(_CREATEDATABASERESPONSE_ERRORCODE)


_CREATEDATABASEREQUEST = _descriptor.Descriptor(
  name='CreateDatabaseRequest',
  full_name='protodb.database.CreateDatabaseRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='name', full_name='protodb.database.CreateDatabaseRequest.name', index=0,
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
  serialized_start=51,
  serialized_end=88,
)


_CREATEDATABASERESPONSE = _descriptor.Descriptor(
  name='CreateDatabaseResponse',
  full_name='protodb.database.CreateDatabaseResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='error_code', full_name='protodb.database.CreateDatabaseResponse.error_code', index=0,
      number=1, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
    _CREATEDATABASERESPONSE_ERRORCODE,
  ],
  options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=91,
  serialized_end=255,
)

_CREATEDATABASERESPONSE.fields_by_name['error_code'].enum_type = _CREATEDATABASERESPONSE_ERRORCODE
_CREATEDATABASERESPONSE_ERRORCODE.containing_type = _CREATEDATABASERESPONSE
DESCRIPTOR.message_types_by_name['CreateDatabaseRequest'] = _CREATEDATABASEREQUEST
DESCRIPTOR.message_types_by_name['CreateDatabaseResponse'] = _CREATEDATABASERESPONSE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

CreateDatabaseRequest = _reflection.GeneratedProtocolMessageType('CreateDatabaseRequest', (_message.Message,), dict(
  DESCRIPTOR = _CREATEDATABASEREQUEST,
  __module__ = 'protodb.database.create_pb2'
  # @@protoc_insertion_point(class_scope:protodb.database.CreateDatabaseRequest)
  ))
_sym_db.RegisterMessage(CreateDatabaseRequest)

CreateDatabaseResponse = _reflection.GeneratedProtocolMessageType('CreateDatabaseResponse', (_message.Message,), dict(
  DESCRIPTOR = _CREATEDATABASERESPONSE,
  __module__ = 'protodb.database.create_pb2'
  # @@protoc_insertion_point(class_scope:protodb.database.CreateDatabaseResponse)
  ))
_sym_db.RegisterMessage(CreateDatabaseResponse)


# @@protoc_insertion_point(module_scope)

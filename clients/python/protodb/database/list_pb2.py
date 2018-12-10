# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: protodb/database/list.proto

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
  name='protodb/database/list.proto',
  package='protodb.database',
  syntax='proto3',
  serialized_pb=_b('\n\x1bprotodb/database/list.proto\x12\x10protodb.database\"\x16\n\x14ListDatabasesRequest\"\xa0\x01\n\x15ListDatabasesResponse\x12\x45\n\nerror_code\x18\x01 \x01(\x0e\x32\x31.protodb.database.ListDatabasesResponse.ErrorCode\x12\x11\n\tdatabases\x18\x02 \x03(\t\"-\n\tErrorCode\x12\x0c\n\x08NO_ERROR\x10\x00\x12\x12\n\x0eINTERNAL_ERROR\x10\x01\x62\x06proto3')
)



_LISTDATABASESRESPONSE_ERRORCODE = _descriptor.EnumDescriptor(
  name='ErrorCode',
  full_name='protodb.database.ListDatabasesResponse.ErrorCode',
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
  ],
  containing_type=None,
  options=None,
  serialized_start=189,
  serialized_end=234,
)
_sym_db.RegisterEnumDescriptor(_LISTDATABASESRESPONSE_ERRORCODE)


_LISTDATABASESREQUEST = _descriptor.Descriptor(
  name='ListDatabasesRequest',
  full_name='protodb.database.ListDatabasesRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
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
  serialized_start=49,
  serialized_end=71,
)


_LISTDATABASESRESPONSE = _descriptor.Descriptor(
  name='ListDatabasesResponse',
  full_name='protodb.database.ListDatabasesResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='error_code', full_name='protodb.database.ListDatabasesResponse.error_code', index=0,
      number=1, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='databases', full_name='protodb.database.ListDatabasesResponse.databases', index=1,
      number=2, type=9, cpp_type=9, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
    _LISTDATABASESRESPONSE_ERRORCODE,
  ],
  options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=74,
  serialized_end=234,
)

_LISTDATABASESRESPONSE.fields_by_name['error_code'].enum_type = _LISTDATABASESRESPONSE_ERRORCODE
_LISTDATABASESRESPONSE_ERRORCODE.containing_type = _LISTDATABASESRESPONSE
DESCRIPTOR.message_types_by_name['ListDatabasesRequest'] = _LISTDATABASESREQUEST
DESCRIPTOR.message_types_by_name['ListDatabasesResponse'] = _LISTDATABASESRESPONSE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

ListDatabasesRequest = _reflection.GeneratedProtocolMessageType('ListDatabasesRequest', (_message.Message,), dict(
  DESCRIPTOR = _LISTDATABASESREQUEST,
  __module__ = 'protodb.database.list_pb2'
  # @@protoc_insertion_point(class_scope:protodb.database.ListDatabasesRequest)
  ))
_sym_db.RegisterMessage(ListDatabasesRequest)

ListDatabasesResponse = _reflection.GeneratedProtocolMessageType('ListDatabasesResponse', (_message.Message,), dict(
  DESCRIPTOR = _LISTDATABASESRESPONSE,
  __module__ = 'protodb.database.list_pb2'
  # @@protoc_insertion_point(class_scope:protodb.database.ListDatabasesResponse)
  ))
_sym_db.RegisterMessage(ListDatabasesResponse)


# @@protoc_insertion_point(module_scope)
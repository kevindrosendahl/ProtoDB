# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: protodb/index/list.proto

import sys
_b=sys.version_info[0]<3 and (lambda x:x) or (lambda x:x.encode('latin1'))
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
from google.protobuf import descriptor_pb2
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from protodb.index import index_pb2 as protodb_dot_index_dot_index__pb2


DESCRIPTOR = _descriptor.FileDescriptor(
  name='protodb/index/list.proto',
  package='protodb.index',
  syntax='proto3',
  serialized_pb=_b('\n\x18protodb/index/list.proto\x12\rprotodb.index\x1a\x19protodb/index/index.proto\":\n\x12ListIndexesRequest\x12\x10\n\x08\x64\x61tabase\x18\x01 \x01(\t\x12\x12\n\ncollection\x18\x02 \x01(\t\"\xdb\x01\n\x13ListIndexesResponse\x12@\n\nerror_code\x18\x01 \x01(\x0e\x32,.protodb.index.ListIndexesResponse.ErrorCode\x12%\n\x07indexes\x18\x02 \x03(\x0b\x32\x14.protodb.index.Index\"[\n\tErrorCode\x12\x0c\n\x08NO_ERROR\x10\x00\x12\x12\n\x0eINTERNAL_ERROR\x10\x01\x12\x14\n\x10INVALID_DATABASE\x10\x02\x12\x16\n\x12INVALID_COLLECTION\x10\x03\x62\x06proto3')
  ,
  dependencies=[protodb_dot_index_dot_index__pb2.DESCRIPTOR,])



_LISTINDEXESRESPONSE_ERRORCODE = _descriptor.EnumDescriptor(
  name='ErrorCode',
  full_name='protodb.index.ListIndexesResponse.ErrorCode',
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
      name='INVALID_DATABASE', index=2, number=2,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='INVALID_COLLECTION', index=3, number=3,
      options=None,
      type=None),
  ],
  containing_type=None,
  options=None,
  serialized_start=259,
  serialized_end=350,
)
_sym_db.RegisterEnumDescriptor(_LISTINDEXESRESPONSE_ERRORCODE)


_LISTINDEXESREQUEST = _descriptor.Descriptor(
  name='ListIndexesRequest',
  full_name='protodb.index.ListIndexesRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='database', full_name='protodb.index.ListIndexesRequest.database', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='collection', full_name='protodb.index.ListIndexesRequest.collection', index=1,
      number=2, type=9, cpp_type=9, label=1,
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
  serialized_start=70,
  serialized_end=128,
)


_LISTINDEXESRESPONSE = _descriptor.Descriptor(
  name='ListIndexesResponse',
  full_name='protodb.index.ListIndexesResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='error_code', full_name='protodb.index.ListIndexesResponse.error_code', index=0,
      number=1, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='indexes', full_name='protodb.index.ListIndexesResponse.indexes', index=1,
      number=2, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
    _LISTINDEXESRESPONSE_ERRORCODE,
  ],
  options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=131,
  serialized_end=350,
)

_LISTINDEXESRESPONSE.fields_by_name['error_code'].enum_type = _LISTINDEXESRESPONSE_ERRORCODE
_LISTINDEXESRESPONSE.fields_by_name['indexes'].message_type = protodb_dot_index_dot_index__pb2._INDEX
_LISTINDEXESRESPONSE_ERRORCODE.containing_type = _LISTINDEXESRESPONSE
DESCRIPTOR.message_types_by_name['ListIndexesRequest'] = _LISTINDEXESREQUEST
DESCRIPTOR.message_types_by_name['ListIndexesResponse'] = _LISTINDEXESRESPONSE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

ListIndexesRequest = _reflection.GeneratedProtocolMessageType('ListIndexesRequest', (_message.Message,), dict(
  DESCRIPTOR = _LISTINDEXESREQUEST,
  __module__ = 'protodb.index.list_pb2'
  # @@protoc_insertion_point(class_scope:protodb.index.ListIndexesRequest)
  ))
_sym_db.RegisterMessage(ListIndexesRequest)

ListIndexesResponse = _reflection.GeneratedProtocolMessageType('ListIndexesResponse', (_message.Message,), dict(
  DESCRIPTOR = _LISTINDEXESRESPONSE,
  __module__ = 'protodb.index.list_pb2'
  # @@protoc_insertion_point(class_scope:protodb.index.ListIndexesResponse)
  ))
_sym_db.RegisterMessage(ListIndexesResponse)


# @@protoc_insertion_point(module_scope)

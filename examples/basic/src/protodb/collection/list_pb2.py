# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: protodb/collection/list.proto

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
  name='protodb/collection/list.proto',
  package='protodb.collection',
  syntax='proto3',
  serialized_pb=_b('\n\x1dprotodb/collection/list.proto\x12\x12protodb.collection\"*\n\x16ListCollectionsRequest\x12\x10\n\x08\x64\x61tabase\x18\x01 \x01(\t\"\xbe\x01\n\x17ListCollectionsResponse\x12I\n\nerror_code\x18\x01 \x01(\x0e\x32\x35.protodb.collection.ListCollectionsResponse.ErrorCode\x12\x13\n\x0b\x63ollections\x18\x02 \x03(\t\"C\n\tErrorCode\x12\x0c\n\x08NO_ERROR\x10\x00\x12\x12\n\x0eINTERNAL_ERROR\x10\x01\x12\x14\n\x10INVALID_DATABASE\x10\x02\x62\x06proto3')
)



_LISTCOLLECTIONSRESPONSE_ERRORCODE = _descriptor.EnumDescriptor(
  name='ErrorCode',
  full_name='protodb.collection.ListCollectionsResponse.ErrorCode',
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
  ],
  containing_type=None,
  options=None,
  serialized_start=221,
  serialized_end=288,
)
_sym_db.RegisterEnumDescriptor(_LISTCOLLECTIONSRESPONSE_ERRORCODE)


_LISTCOLLECTIONSREQUEST = _descriptor.Descriptor(
  name='ListCollectionsRequest',
  full_name='protodb.collection.ListCollectionsRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='database', full_name='protodb.collection.ListCollectionsRequest.database', index=0,
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
  serialized_start=53,
  serialized_end=95,
)


_LISTCOLLECTIONSRESPONSE = _descriptor.Descriptor(
  name='ListCollectionsResponse',
  full_name='protodb.collection.ListCollectionsResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='error_code', full_name='protodb.collection.ListCollectionsResponse.error_code', index=0,
      number=1, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='collections', full_name='protodb.collection.ListCollectionsResponse.collections', index=1,
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
    _LISTCOLLECTIONSRESPONSE_ERRORCODE,
  ],
  options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=98,
  serialized_end=288,
)

_LISTCOLLECTIONSRESPONSE.fields_by_name['error_code'].enum_type = _LISTCOLLECTIONSRESPONSE_ERRORCODE
_LISTCOLLECTIONSRESPONSE_ERRORCODE.containing_type = _LISTCOLLECTIONSRESPONSE
DESCRIPTOR.message_types_by_name['ListCollectionsRequest'] = _LISTCOLLECTIONSREQUEST
DESCRIPTOR.message_types_by_name['ListCollectionsResponse'] = _LISTCOLLECTIONSRESPONSE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

ListCollectionsRequest = _reflection.GeneratedProtocolMessageType('ListCollectionsRequest', (_message.Message,), dict(
  DESCRIPTOR = _LISTCOLLECTIONSREQUEST,
  __module__ = 'protodb.collection.list_pb2'
  # @@protoc_insertion_point(class_scope:protodb.collection.ListCollectionsRequest)
  ))
_sym_db.RegisterMessage(ListCollectionsRequest)

ListCollectionsResponse = _reflection.GeneratedProtocolMessageType('ListCollectionsResponse', (_message.Message,), dict(
  DESCRIPTOR = _LISTCOLLECTIONSRESPONSE,
  __module__ = 'protodb.collection.list_pb2'
  # @@protoc_insertion_point(class_scope:protodb.collection.ListCollectionsResponse)
  ))
_sym_db.RegisterMessage(ListCollectionsResponse)


# @@protoc_insertion_point(module_scope)

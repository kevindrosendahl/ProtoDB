# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: protodb/index/create.proto

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
  name='protodb/index/create.proto',
  package='protodb.index',
  syntax='proto3',
  serialized_pb=_b('\n\x1aprotodb/index/create.proto\x12\rprotodb.index\"I\n\x12\x43reateIndexRequest\x12\x10\n\x08\x64\x61tabase\x18\x01 \x01(\t\x12\x12\n\ncollection\x18\x02 \x01(\t\x12\r\n\x05\x66ield\x18\x03 \x01(\x05\"\xf0\x01\n\x13\x43reateIndexResponse\x12@\n\nerror_code\x18\x01 \x01(\x0e\x32,.protodb.index.CreateIndexResponse.ErrorCode\x12\n\n\x02id\x18\x02 \x01(\x04\"\x8a\x01\n\tErrorCode\x12\x0c\n\x08NO_ERROR\x10\x00\x12\x12\n\x0eINTERNAL_ERROR\x10\x01\x12\x14\n\x10INVALID_DATABASE\x10\x02\x12\x16\n\x12INVALID_COLLECTION\x10\x03\x12\x11\n\rINVALID_FIELD\x10\x04\x12\x1a\n\x16UNSUPPORTED_FIELD_TYPE\x10\x05\x62\x06proto3')
)



_CREATEINDEXRESPONSE_ERRORCODE = _descriptor.EnumDescriptor(
  name='ErrorCode',
  full_name='protodb.index.CreateIndexResponse.ErrorCode',
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
    _descriptor.EnumValueDescriptor(
      name='INVALID_FIELD', index=4, number=4,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='UNSUPPORTED_FIELD_TYPE', index=5, number=5,
      options=None,
      type=None),
  ],
  containing_type=None,
  options=None,
  serialized_start=223,
  serialized_end=361,
)
_sym_db.RegisterEnumDescriptor(_CREATEINDEXRESPONSE_ERRORCODE)


_CREATEINDEXREQUEST = _descriptor.Descriptor(
  name='CreateIndexRequest',
  full_name='protodb.index.CreateIndexRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='database', full_name='protodb.index.CreateIndexRequest.database', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='collection', full_name='protodb.index.CreateIndexRequest.collection', index=1,
      number=2, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='field', full_name='protodb.index.CreateIndexRequest.field', index=2,
      number=3, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
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
  serialized_start=45,
  serialized_end=118,
)


_CREATEINDEXRESPONSE = _descriptor.Descriptor(
  name='CreateIndexResponse',
  full_name='protodb.index.CreateIndexResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='error_code', full_name='protodb.index.CreateIndexResponse.error_code', index=0,
      number=1, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='id', full_name='protodb.index.CreateIndexResponse.id', index=1,
      number=2, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
    _CREATEINDEXRESPONSE_ERRORCODE,
  ],
  options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=121,
  serialized_end=361,
)

_CREATEINDEXRESPONSE.fields_by_name['error_code'].enum_type = _CREATEINDEXRESPONSE_ERRORCODE
_CREATEINDEXRESPONSE_ERRORCODE.containing_type = _CREATEINDEXRESPONSE
DESCRIPTOR.message_types_by_name['CreateIndexRequest'] = _CREATEINDEXREQUEST
DESCRIPTOR.message_types_by_name['CreateIndexResponse'] = _CREATEINDEXRESPONSE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

CreateIndexRequest = _reflection.GeneratedProtocolMessageType('CreateIndexRequest', (_message.Message,), dict(
  DESCRIPTOR = _CREATEINDEXREQUEST,
  __module__ = 'protodb.index.create_pb2'
  # @@protoc_insertion_point(class_scope:protodb.index.CreateIndexRequest)
  ))
_sym_db.RegisterMessage(CreateIndexRequest)

CreateIndexResponse = _reflection.GeneratedProtocolMessageType('CreateIndexResponse', (_message.Message,), dict(
  DESCRIPTOR = _CREATEINDEXRESPONSE,
  __module__ = 'protodb.index.create_pb2'
  # @@protoc_insertion_point(class_scope:protodb.index.CreateIndexResponse)
  ))
_sym_db.RegisterMessage(CreateIndexResponse)


# @@protoc_insertion_point(module_scope)

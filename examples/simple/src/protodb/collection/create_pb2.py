# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: protodb/collection/create.proto

import sys
_b=sys.version_info[0]<3 and (lambda x:x) or (lambda x:x.encode('latin1'))
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
from google.protobuf import descriptor_pb2
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from google.protobuf import descriptor_pb2 as google_dot_protobuf_dot_descriptor__pb2


DESCRIPTOR = _descriptor.FileDescriptor(
  name='protodb/collection/create.proto',
  package='protodb.collection',
  syntax='proto3',
  serialized_pb=_b('\n\x1fprotodb/collection/create.proto\x12\x12protodb.collection\x1a google/protobuf/descriptor.proto\"k\n\x17\x43reateCollectionRequest\x12\x10\n\x08\x64\x61tabase\x18\x01 \x01(\t\x12\x0c\n\x04name\x18\x02 \x01(\t\x12\x30\n\x06schema\x18\x03 \x01(\x0b\x32 .google.protobuf.DescriptorProto\"\x8e\x04\n\x18\x43reateCollectionResponse\x12N\n\x0c\x66\x61ilure_code\x18\x01 \x01(\x0e\x32\x38.protodb.collection.CreateCollectionResponse.FailureCode\x12N\n\x0cschema_error\x18\x02 \x01(\x0b\x32\x38.protodb.collection.CreateCollectionResponse.SchemaError\x1a\xe1\x01\n\x0bSchemaError\x12V\n\x04\x63ode\x18\x01 \x01(\x0e\x32H.protodb.collection.CreateCollectionResponse.SchemaError.SchemaErrorCode\x12\x0f\n\x07message\x18\x02 \x01(\t\"i\n\x0fSchemaErrorCode\x12\x13\n\x0fNO_SCHEMA_ERROR\x10\x00\x12\x14\n\x10MISSING_ID_FIELD\x10\x01\x12\x13\n\x0fINVALID_ID_TYPE\x10\x02\x12\x16\n\x12INVALID_FIELD_TYPE\x10\x03\"n\n\x0b\x46\x61ilureCode\x12\x0c\n\x08NO_ERROR\x10\x00\x12\x12\n\x0eINTERNAL_ERROR\x10\x01\x12\x14\n\x10INVALID_DATABASE\x10\x02\x12\x15\n\x11\x43OLLECTION_EXISTS\x10\x03\x12\x10\n\x0cSCHEMA_ERROR\x10\x04\x62\x06proto3')
  ,
  dependencies=[google_dot_protobuf_dot_descriptor__pb2.DESCRIPTOR,])



_CREATECOLLECTIONRESPONSE_SCHEMAERROR_SCHEMAERRORCODE = _descriptor.EnumDescriptor(
  name='SchemaErrorCode',
  full_name='protodb.collection.CreateCollectionResponse.SchemaError.SchemaErrorCode',
  filename=None,
  file=DESCRIPTOR,
  values=[
    _descriptor.EnumValueDescriptor(
      name='NO_SCHEMA_ERROR', index=0, number=0,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='MISSING_ID_FIELD', index=1, number=1,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='INVALID_ID_TYPE', index=2, number=2,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='INVALID_FIELD_TYPE', index=3, number=3,
      options=None,
      type=None),
  ],
  containing_type=None,
  options=None,
  serialized_start=508,
  serialized_end=613,
)
_sym_db.RegisterEnumDescriptor(_CREATECOLLECTIONRESPONSE_SCHEMAERROR_SCHEMAERRORCODE)

_CREATECOLLECTIONRESPONSE_FAILURECODE = _descriptor.EnumDescriptor(
  name='FailureCode',
  full_name='protodb.collection.CreateCollectionResponse.FailureCode',
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
      name='COLLECTION_EXISTS', index=3, number=3,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='SCHEMA_ERROR', index=4, number=4,
      options=None,
      type=None),
  ],
  containing_type=None,
  options=None,
  serialized_start=615,
  serialized_end=725,
)
_sym_db.RegisterEnumDescriptor(_CREATECOLLECTIONRESPONSE_FAILURECODE)


_CREATECOLLECTIONREQUEST = _descriptor.Descriptor(
  name='CreateCollectionRequest',
  full_name='protodb.collection.CreateCollectionRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='database', full_name='protodb.collection.CreateCollectionRequest.database', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='name', full_name='protodb.collection.CreateCollectionRequest.name', index=1,
      number=2, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='schema', full_name='protodb.collection.CreateCollectionRequest.schema', index=2,
      number=3, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
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
  serialized_start=89,
  serialized_end=196,
)


_CREATECOLLECTIONRESPONSE_SCHEMAERROR = _descriptor.Descriptor(
  name='SchemaError',
  full_name='protodb.collection.CreateCollectionResponse.SchemaError',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='code', full_name='protodb.collection.CreateCollectionResponse.SchemaError.code', index=0,
      number=1, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='message', full_name='protodb.collection.CreateCollectionResponse.SchemaError.message', index=1,
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
    _CREATECOLLECTIONRESPONSE_SCHEMAERROR_SCHEMAERRORCODE,
  ],
  options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=388,
  serialized_end=613,
)

_CREATECOLLECTIONRESPONSE = _descriptor.Descriptor(
  name='CreateCollectionResponse',
  full_name='protodb.collection.CreateCollectionResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='failure_code', full_name='protodb.collection.CreateCollectionResponse.failure_code', index=0,
      number=1, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='schema_error', full_name='protodb.collection.CreateCollectionResponse.schema_error', index=1,
      number=2, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[_CREATECOLLECTIONRESPONSE_SCHEMAERROR, ],
  enum_types=[
    _CREATECOLLECTIONRESPONSE_FAILURECODE,
  ],
  options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=199,
  serialized_end=725,
)

_CREATECOLLECTIONREQUEST.fields_by_name['schema'].message_type = google_dot_protobuf_dot_descriptor__pb2._DESCRIPTORPROTO
_CREATECOLLECTIONRESPONSE_SCHEMAERROR.fields_by_name['code'].enum_type = _CREATECOLLECTIONRESPONSE_SCHEMAERROR_SCHEMAERRORCODE
_CREATECOLLECTIONRESPONSE_SCHEMAERROR.containing_type = _CREATECOLLECTIONRESPONSE
_CREATECOLLECTIONRESPONSE_SCHEMAERROR_SCHEMAERRORCODE.containing_type = _CREATECOLLECTIONRESPONSE_SCHEMAERROR
_CREATECOLLECTIONRESPONSE.fields_by_name['failure_code'].enum_type = _CREATECOLLECTIONRESPONSE_FAILURECODE
_CREATECOLLECTIONRESPONSE.fields_by_name['schema_error'].message_type = _CREATECOLLECTIONRESPONSE_SCHEMAERROR
_CREATECOLLECTIONRESPONSE_FAILURECODE.containing_type = _CREATECOLLECTIONRESPONSE
DESCRIPTOR.message_types_by_name['CreateCollectionRequest'] = _CREATECOLLECTIONREQUEST
DESCRIPTOR.message_types_by_name['CreateCollectionResponse'] = _CREATECOLLECTIONRESPONSE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

CreateCollectionRequest = _reflection.GeneratedProtocolMessageType('CreateCollectionRequest', (_message.Message,), dict(
  DESCRIPTOR = _CREATECOLLECTIONREQUEST,
  __module__ = 'protodb.collection.create_pb2'
  # @@protoc_insertion_point(class_scope:protodb.collection.CreateCollectionRequest)
  ))
_sym_db.RegisterMessage(CreateCollectionRequest)

CreateCollectionResponse = _reflection.GeneratedProtocolMessageType('CreateCollectionResponse', (_message.Message,), dict(

  SchemaError = _reflection.GeneratedProtocolMessageType('SchemaError', (_message.Message,), dict(
    DESCRIPTOR = _CREATECOLLECTIONRESPONSE_SCHEMAERROR,
    __module__ = 'protodb.collection.create_pb2'
    # @@protoc_insertion_point(class_scope:protodb.collection.CreateCollectionResponse.SchemaError)
    ))
  ,
  DESCRIPTOR = _CREATECOLLECTIONRESPONSE,
  __module__ = 'protodb.collection.create_pb2'
  # @@protoc_insertion_point(class_scope:protodb.collection.CreateCollectionResponse)
  ))
_sym_db.RegisterMessage(CreateCollectionResponse)
_sym_db.RegisterMessage(CreateCollectionResponse.SchemaError)


# @@protoc_insertion_point(module_scope)

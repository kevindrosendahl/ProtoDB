# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: protodb/wasm/run.proto

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
  name='protodb/wasm/run.proto',
  package='protodb.wasm',
  syntax='proto3',
  serialized_pb=_b('\n\x16protodb/wasm/run.proto\x12\x0cprotodb.wasm\x1a google/protobuf/descriptor.proto\"2\n\x10RunModuleRequest\x12\x10\n\x08\x64\x61tabase\x18\x01 \x01(\t\x12\x0c\n\x04name\x18\x02 \x01(\t\"\xbb\x01\n\x11RunModuleResponse\x12=\n\nerror_code\x18\x01 \x01(\x0e\x32).protodb.wasm.RunModuleResponse.ErrorCode\x12\x0e\n\x06result\x18\x02 \x01(\x0c\"W\n\tErrorCode\x12\x0c\n\x08NO_ERROR\x10\x00\x12\x12\n\x0eINTERNAL_ERROR\x10\x01\x12\x14\n\x10INVALID_DATABASE\x10\x02\x12\x12\n\x0eINVALID_MODULE\x10\x03\x62\x06proto3')
  ,
  dependencies=[google_dot_protobuf_dot_descriptor__pb2.DESCRIPTOR,])



_RUNMODULERESPONSE_ERRORCODE = _descriptor.EnumDescriptor(
  name='ErrorCode',
  full_name='protodb.wasm.RunModuleResponse.ErrorCode',
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
      name='INVALID_MODULE', index=3, number=3,
      options=None,
      type=None),
  ],
  containing_type=None,
  options=None,
  serialized_start=227,
  serialized_end=314,
)
_sym_db.RegisterEnumDescriptor(_RUNMODULERESPONSE_ERRORCODE)


_RUNMODULEREQUEST = _descriptor.Descriptor(
  name='RunModuleRequest',
  full_name='protodb.wasm.RunModuleRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='database', full_name='protodb.wasm.RunModuleRequest.database', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='name', full_name='protodb.wasm.RunModuleRequest.name', index=1,
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
  serialized_start=74,
  serialized_end=124,
)


_RUNMODULERESPONSE = _descriptor.Descriptor(
  name='RunModuleResponse',
  full_name='protodb.wasm.RunModuleResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='error_code', full_name='protodb.wasm.RunModuleResponse.error_code', index=0,
      number=1, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='result', full_name='protodb.wasm.RunModuleResponse.result', index=1,
      number=2, type=12, cpp_type=9, label=1,
      has_default_value=False, default_value=_b(""),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
    _RUNMODULERESPONSE_ERRORCODE,
  ],
  options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=127,
  serialized_end=314,
)

_RUNMODULERESPONSE.fields_by_name['error_code'].enum_type = _RUNMODULERESPONSE_ERRORCODE
_RUNMODULERESPONSE_ERRORCODE.containing_type = _RUNMODULERESPONSE
DESCRIPTOR.message_types_by_name['RunModuleRequest'] = _RUNMODULEREQUEST
DESCRIPTOR.message_types_by_name['RunModuleResponse'] = _RUNMODULERESPONSE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

RunModuleRequest = _reflection.GeneratedProtocolMessageType('RunModuleRequest', (_message.Message,), dict(
  DESCRIPTOR = _RUNMODULEREQUEST,
  __module__ = 'protodb.wasm.run_pb2'
  # @@protoc_insertion_point(class_scope:protodb.wasm.RunModuleRequest)
  ))
_sym_db.RegisterMessage(RunModuleRequest)

RunModuleResponse = _reflection.GeneratedProtocolMessageType('RunModuleResponse', (_message.Message,), dict(
  DESCRIPTOR = _RUNMODULERESPONSE,
  __module__ = 'protodb.wasm.run_pb2'
  # @@protoc_insertion_point(class_scope:protodb.wasm.RunModuleResponse)
  ))
_sym_db.RegisterMessage(RunModuleResponse)


# @@protoc_insertion_point(module_scope)
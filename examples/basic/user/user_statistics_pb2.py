# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: user_statistics.proto

import sys
_b=sys.version_info[0]<3 and (lambda x:x) or (lambda x:x.encode('latin1'))
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='user_statistics.proto',
  package='protodb.examples.user_statistics',
  syntax='proto3',
  serialized_options=None,
  serialized_pb=_b('\n\x15user_statistics.proto\x12 protodb.examples.user_statistics\"y\n\x0eUserStatistics\x12\x11\n\tnum_users\x18\x01 \x01(\r\x12\x10\n\x08\x61ge_mean\x18\x02 \x01(\x01\x12\x13\n\x0b\x61ge_std_dev\x18\x03 \x01(\x01\x12\x14\n\x0c\x61ge_variance\x18\x04 \x01(\x01\x12\x17\n\x0f\x61ge_cardinality\x18\x05 \x01(\x04\x62\x06proto3')
)




_USERSTATISTICS = _descriptor.Descriptor(
  name='UserStatistics',
  full_name='protodb.examples.user_statistics.UserStatistics',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='num_users', full_name='protodb.examples.user_statistics.UserStatistics.num_users', index=0,
      number=1, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='age_mean', full_name='protodb.examples.user_statistics.UserStatistics.age_mean', index=1,
      number=2, type=1, cpp_type=5, label=1,
      has_default_value=False, default_value=float(0),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='age_std_dev', full_name='protodb.examples.user_statistics.UserStatistics.age_std_dev', index=2,
      number=3, type=1, cpp_type=5, label=1,
      has_default_value=False, default_value=float(0),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='age_variance', full_name='protodb.examples.user_statistics.UserStatistics.age_variance', index=3,
      number=4, type=1, cpp_type=5, label=1,
      has_default_value=False, default_value=float(0),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='age_cardinality', full_name='protodb.examples.user_statistics.UserStatistics.age_cardinality', index=4,
      number=5, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=59,
  serialized_end=180,
)

DESCRIPTOR.message_types_by_name['UserStatistics'] = _USERSTATISTICS
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

UserStatistics = _reflection.GeneratedProtocolMessageType('UserStatistics', (_message.Message,), dict(
  DESCRIPTOR = _USERSTATISTICS,
  __module__ = 'user_statistics_pb2'
  # @@protoc_insertion_point(class_scope:protodb.examples.user_statistics.UserStatistics)
  ))
_sym_db.RegisterMessage(UserStatistics)


# @@protoc_insertion_point(module_scope)

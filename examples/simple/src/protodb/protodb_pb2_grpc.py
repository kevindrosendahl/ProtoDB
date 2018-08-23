# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
import grpc

from protodb import database_create_pb2 as protodb_dot_database__create__pb2
from protodb import database_list_pb2 as protodb_dot_database__list__pb2


class ProtoDBStub(object):
  # missing associated documentation comment in .proto file
  pass

  def __init__(self, channel):
    """Constructor.

    Args:
      channel: A grpc.Channel.
    """
    self.CreateDatabase = channel.unary_unary(
        '/protodb.ProtoDB/CreateDatabase',
        request_serializer=protodb_dot_database__create__pb2.CreateDatabaseRequest.SerializeToString,
        response_deserializer=protodb_dot_database__create__pb2.CreateDatabaseResponse.FromString,
        )
    self.ListDatabases = channel.unary_unary(
        '/protodb.ProtoDB/ListDatabases',
        request_serializer=protodb_dot_database__list__pb2.ListDatabasesRequest.SerializeToString,
        response_deserializer=protodb_dot_database__list__pb2.ListDatabasesResponse.FromString,
        )


class ProtoDBServicer(object):
  # missing associated documentation comment in .proto file
  pass

  def CreateDatabase(self, request, context):
    # missing associated documentation comment in .proto file
    pass
    context.set_code(grpc.StatusCode.UNIMPLEMENTED)
    context.set_details('Method not implemented!')
    raise NotImplementedError('Method not implemented!')

  def ListDatabases(self, request, context):
    # missing associated documentation comment in .proto file
    pass
    context.set_code(grpc.StatusCode.UNIMPLEMENTED)
    context.set_details('Method not implemented!')
    raise NotImplementedError('Method not implemented!')


def add_ProtoDBServicer_to_server(servicer, server):
  rpc_method_handlers = {
      'CreateDatabase': grpc.unary_unary_rpc_method_handler(
          servicer.CreateDatabase,
          request_deserializer=protodb_dot_database__create__pb2.CreateDatabaseRequest.FromString,
          response_serializer=protodb_dot_database__create__pb2.CreateDatabaseResponse.SerializeToString,
      ),
      'ListDatabases': grpc.unary_unary_rpc_method_handler(
          servicer.ListDatabases,
          request_deserializer=protodb_dot_database__list__pb2.ListDatabasesRequest.FromString,
          response_serializer=protodb_dot_database__list__pb2.ListDatabasesResponse.SerializeToString,
      ),
  }
  generic_handler = grpc.method_handlers_generic_handler(
      'protodb.ProtoDB', rpc_method_handlers)
  server.add_generic_rpc_handlers((generic_handler,))

# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
import grpc

from protodb.collection import create_pb2 as protodb_dot_collection_dot_create__pb2
from protodb.collection import find_object_pb2 as protodb_dot_collection_dot_find__object__pb2
from protodb.collection import insert_object_pb2 as protodb_dot_collection_dot_insert__object__pb2
from protodb.collection import list_pb2 as protodb_dot_collection_dot_list__pb2
from protodb.database import create_pb2 as protodb_dot_database_dot_create__pb2
from protodb.database import list_pb2 as protodb_dot_database_dot_list__pb2


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
        request_serializer=protodb_dot_database_dot_create__pb2.CreateDatabaseRequest.SerializeToString,
        response_deserializer=protodb_dot_database_dot_create__pb2.CreateDatabaseResponse.FromString,
        )
    self.ListDatabases = channel.unary_unary(
        '/protodb.ProtoDB/ListDatabases',
        request_serializer=protodb_dot_database_dot_list__pb2.ListDatabasesRequest.SerializeToString,
        response_deserializer=protodb_dot_database_dot_list__pb2.ListDatabasesResponse.FromString,
        )
    self.CreateCollection = channel.unary_unary(
        '/protodb.ProtoDB/CreateCollection',
        request_serializer=protodb_dot_collection_dot_create__pb2.CreateCollectionRequest.SerializeToString,
        response_deserializer=protodb_dot_collection_dot_create__pb2.CreateCollectionResponse.FromString,
        )
    self.ListCollections = channel.unary_unary(
        '/protodb.ProtoDB/ListCollections',
        request_serializer=protodb_dot_collection_dot_list__pb2.ListCollectionsRequest.SerializeToString,
        response_deserializer=protodb_dot_collection_dot_list__pb2.ListCollectionsResponse.FromString,
        )
    self.InsertObject = channel.unary_unary(
        '/protodb.ProtoDB/InsertObject',
        request_serializer=protodb_dot_collection_dot_insert__object__pb2.InsertObjectRequest.SerializeToString,
        response_deserializer=protodb_dot_collection_dot_insert__object__pb2.InsertObjectResponse.FromString,
        )
    self.FindObject = channel.unary_unary(
        '/protodb.ProtoDB/FindObject',
        request_serializer=protodb_dot_collection_dot_find__object__pb2.FindObjectRequest.SerializeToString,
        response_deserializer=protodb_dot_collection_dot_find__object__pb2.FindObjectResponse.FromString,
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

  def CreateCollection(self, request, context):
    # missing associated documentation comment in .proto file
    pass
    context.set_code(grpc.StatusCode.UNIMPLEMENTED)
    context.set_details('Method not implemented!')
    raise NotImplementedError('Method not implemented!')

  def ListCollections(self, request, context):
    # missing associated documentation comment in .proto file
    pass
    context.set_code(grpc.StatusCode.UNIMPLEMENTED)
    context.set_details('Method not implemented!')
    raise NotImplementedError('Method not implemented!')

  def InsertObject(self, request, context):
    # missing associated documentation comment in .proto file
    pass
    context.set_code(grpc.StatusCode.UNIMPLEMENTED)
    context.set_details('Method not implemented!')
    raise NotImplementedError('Method not implemented!')

  def FindObject(self, request, context):
    # missing associated documentation comment in .proto file
    pass
    context.set_code(grpc.StatusCode.UNIMPLEMENTED)
    context.set_details('Method not implemented!')
    raise NotImplementedError('Method not implemented!')


def add_ProtoDBServicer_to_server(servicer, server):
  rpc_method_handlers = {
      'CreateDatabase': grpc.unary_unary_rpc_method_handler(
          servicer.CreateDatabase,
          request_deserializer=protodb_dot_database_dot_create__pb2.CreateDatabaseRequest.FromString,
          response_serializer=protodb_dot_database_dot_create__pb2.CreateDatabaseResponse.SerializeToString,
      ),
      'ListDatabases': grpc.unary_unary_rpc_method_handler(
          servicer.ListDatabases,
          request_deserializer=protodb_dot_database_dot_list__pb2.ListDatabasesRequest.FromString,
          response_serializer=protodb_dot_database_dot_list__pb2.ListDatabasesResponse.SerializeToString,
      ),
      'CreateCollection': grpc.unary_unary_rpc_method_handler(
          servicer.CreateCollection,
          request_deserializer=protodb_dot_collection_dot_create__pb2.CreateCollectionRequest.FromString,
          response_serializer=protodb_dot_collection_dot_create__pb2.CreateCollectionResponse.SerializeToString,
      ),
      'ListCollections': grpc.unary_unary_rpc_method_handler(
          servicer.ListCollections,
          request_deserializer=protodb_dot_collection_dot_list__pb2.ListCollectionsRequest.FromString,
          response_serializer=protodb_dot_collection_dot_list__pb2.ListCollectionsResponse.SerializeToString,
      ),
      'InsertObject': grpc.unary_unary_rpc_method_handler(
          servicer.InsertObject,
          request_deserializer=protodb_dot_collection_dot_insert__object__pb2.InsertObjectRequest.FromString,
          response_serializer=protodb_dot_collection_dot_insert__object__pb2.InsertObjectResponse.SerializeToString,
      ),
      'FindObject': grpc.unary_unary_rpc_method_handler(
          servicer.FindObject,
          request_deserializer=protodb_dot_collection_dot_find__object__pb2.FindObjectRequest.FromString,
          response_serializer=protodb_dot_collection_dot_find__object__pb2.FindObjectResponse.SerializeToString,
      ),
  }
  generic_handler = grpc.method_handlers_generic_handler(
      'protodb.ProtoDB', rpc_method_handlers)
  server.add_generic_rpc_handlers((generic_handler,))

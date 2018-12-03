import grpc

from google.protobuf.descriptor_pb2 import DescriptorProto

from protodb import protodb_pb2_grpc
from protodb.database.create_pb2 import CreateDatabaseRequest
from protodb.database.list_pb2 import ListDatabasesRequest
from protodb.collection.create_pb2 import CreateCollectionRequest
from protodb.collection.list_pb2 import ListCollectionsRequest
from protodb.object.insert_pb2 import InsertObjectRequest
from protodb.object.find_pb2 import FindObjectRequest


class Client:

    def __init__(self, url='localhost:10000'):
        channel = grpc.insecure_channel(url)
        self.stub = protodb_pb2_grpc.ProtoDBStub(channel)

    def create_database(self, name):
        request = CreateDatabaseRequest(name=name)
        return self.stub.CreateDatabase(request)

    def list_databases(self):
        request = ListDatabasesRequest()
        return self.stub.ListDatabases(request)

    def create_collection(self, database, name, descriptor):
        descriptor_proto = DescriptorProto()
        descriptor.CopyToProto(descriptor_proto)

        request = CreateCollectionRequest(database=database, name=name)
        request.schema.MergeFrom(descriptor_proto)

        return self.stub.CreateCollection(request)

    def list_collections(self, database):
        request = ListCollectionsRequest(database=database)
        return self.stub.ListCollections(request)

    def insert_user(self, database, collection, user):
        request = InsertObjectRequest(
            database=database,
            collection=collection,
            object=user.SerializeToString())
        return self.stub.InsertObject(request)

    def find_user(self, database, collection, id):
        request = FindObjectRequest(database=database, collection=collection, id=id)
        return self.stub.FindObject(request)

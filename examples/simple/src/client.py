import grpc

from google.protobuf.descriptor_pb2 import DescriptorProto

from protodb import protodb_pb2_grpc

from protodb.database.create_pb2 import CreateDatabaseRequest, CreateDatabaseResponse

from protodb.collection.create_pb2 import CreateCollectionRequest

from protodb.database.list_pb2 import ListDatabasesRequest, ListDatabasesResponse
from protodb.collection.list_pb2 import ListCollectionsRequest, ListCollectionsResponse
from protodb.collection.insert_object_pb2 import InsertObjectRequest, InsertObjectResponse
from protodb.collection.find_object_pb2 import FindObjectRequest, FindObjectResponse


class Client:

    def __init__(self, url='localhost:10000'):
        channel = grpc.insecure_channel(url)
        self.stub = protodb_pb2_grpc.ProtoDBStub(channel)

    def create_database(self, name):
        request = CreateDatabaseRequest()
        request.name = name
        return self.stub.CreateDatabase(request)

    def list_databases(self):
        request = ListDatabasesRequest()
        return self.stub.ListDatabases(request)

    def create_collection(self, database, name, descriptor):
        request = CreateCollectionRequest()
        request.database = database
        request.name = name

        descriptor_proto = DescriptorProto()
        descriptor.CopyToProto(descriptor_proto)
        request.schema.MergeFrom(descriptor_proto)

        return self.stub.CreateCollection(request)

    def list_collections(self, database):
        request = ListCollectionsRequest()
        request.database = database
        return self.stub.ListCollections(request)

    def insert_user(self, database, collection, user):
        request = InsertObjectRequest()
        request.database = database
        request.collection = collection
        request.object = user.SerializeToString()

        return self.stub.InsertObject(request)

    def find_user(self, database, collection, id):
        request = FindObjectRequest()
        request.database = database
        request.collection = collection
        request.id = id

        return self.stub.FindObject(request)


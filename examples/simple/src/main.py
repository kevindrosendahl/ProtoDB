import grpc

from google.protobuf.descriptor_pb2 import DescriptorProto

from protodb import protodb_pb2_grpc

from protodb.database.create_pb2 import CreateDatabaseRequest, CreateDatabaseResponse
from protodb.database.create_pb2 import _CREATEDATABASERESPONSE_FAILURECODE

from protodb.collection.create_pb2 import CreateCollectionRequest
from protodb.collection.create_pb2 import CreateCollectionResponse
from protodb.collection.create_pb2 import _CREATECOLLECTIONRESPONSE_FAILURECODE

from protodb.database.list_pb2 import ListDatabasesRequest, ListDatabasesResponse
from protodb.collection.list_pb2 import ListCollectionsRequest, ListCollectionsResponse
from protodb.collection.list_pb2 import _LISTCOLLECTIONSRESPONSE_FAILURECODE

from user_pb2 import User

def create_database(stub, db_name):
    request = CreateDatabaseRequest()
    request.name = db_name
    response = stub.CreateDatabase(request)

    if response.success:
        print('create database succeeded!')
        return

    if response.failure_code == CreateDatabaseResponse.DATABASE_EXISTS:
        print('database already exists')
        return

    failure_code_str = _CREATEDATABASERESPONSE_FAILURECODE.values_by_number[
         response.failure_code].name
    print('create database failed: {}', failure_code_str)


def list_databases(stub):
    request = ListDatabasesRequest()
    response = stub.ListDatabases(request)

    print('databases:')
    for database in response.databases:
        print ('  {}'.format(database))


def create_collection(stub, db_name, collection_name, schema):
    create_collection_request = CreateCollectionRequest()
    create_collection_request.database = db_name
    create_collection_request.name = collection_name

    descriptor_proto = DescriptorProto()
    schema.CopyToProto(descriptor_proto)
    create_collection_request.schema.MergeFrom(descriptor_proto)
    create_collection_response = stub.CreateCollection(create_collection_request)

    if create_collection_response.success:
        print('create collection succeeded!')
    else:
        if create_collection_response.failure_code == CreateCollectionResponse.COLLECTION_EXISTS:
            print('collection already exists')
        elif create_collection_response.failure_code == CreateCollectionResponse.INVALID_DATABASE:
            print('invalid database')
        else:
            failure_code_str = _CREATECOLLECTIONRESPONSE_FAILURECODE.values_by_number[
                create_collection_response.failure_code].name
            print('create collection failed: {}'.format(failure_code_str))


def list_collections(stub, db_name):
    list_collections_request = ListCollectionsRequest()
    list_collections_request.database = db_name

    list_collections_response = stub.ListCollections(list_collections_request)

    if list_collections_response.success:
        print('collections:')
        for collection in list_collections_response.collections:
            print('  {}'.format(collection))
    else:
        if list_collections_response.failure_code == ListCollectionsResponse.INVALID_DATABASE:
            print('collection already exists')
        else:
            failure_code_str = _LISTCOLLECTIONSRESPONSE_FAILURECODE.values_by_number[
                list_collections_response.failure_code].name
            print('list collections failed: ' + failure_code_str)

if __name__ == '__main__':
    with grpc.insecure_channel('localhost:10000') as channel:
        stub = protodb_pb2_grpc.ProtoDBStub(channel)
        create_database(stub, 'foo')
        list_databases(stub)
        create_collection(stub, 'foo', 'users', User.DESCRIPTOR)
        list_collections(stub, 'foo')
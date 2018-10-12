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
from protodb.collection.insert_object_pb2 import InsertObjectRequest, InsertObjectResponse
from protodb.collection.insert_object_pb2 import _INSERTOBJECTRESPONSE_FAILURECODE
from protodb.collection.find_object_pb2 import FindObjectRequest, FindObjectResponse
from protodb.collection.find_object_pb2 import _FINDOBJECTRESPONSE_FAILURECODE

from user_pb2 import User

def create_database(stub, db_name):
    request = CreateDatabaseRequest()
    request.name = db_name

    resposne = stub.CreateDatabase(request)

    if resposne.success:
        print('create database succeeded!')
        return

    if resposne.failure_code == CreateDatabaseResponse.DATABASE_EXISTS:
        print('database already exists')
        return

    failure_code_str = _CREATEDATABASERESPONSE_FAILURECODE.values_by_number[
         resposne.failure_code].name
    print('create database failed: {}', failure_code_str)


def list_databases(stub):
    request = ListDatabasesRequest()

    response = stub.ListDatabases(request)

    print()
    print('databases:')
    for database in response.databases:
        print ('  {}'.format(database))


def create_collection(stub, db_name, collection_name, schema):
    request = CreateCollectionRequest()
    request.database = db_name
    request.name = collection_name

    descriptor_proto = DescriptorProto()
    schema.CopyToProto(descriptor_proto)
    request.schema.MergeFrom(descriptor_proto)

    response = stub.CreateCollection(request)

    print()

    if response.success:
        print('create collection succeeded!')
        return

    if response.failure_code == CreateCollectionResponse.COLLECTION_EXISTS:
        print('collection already exists')
        return

    if response.failure_code == CreateCollectionResponse.INVALID_DATABASE:
        print('invalid database')
        return

    failure_code_str = _CREATECOLLECTIONRESPONSE_FAILURECODE.values_by_number[
        response.failure_code].name
    print('create collection failed: {}'.format(failure_code_str))
    print(response.schema_error)


def list_collections(stub, db_name):
    request = ListCollectionsRequest()
    request.database = db_name

    response = stub.ListCollections(request)

    print()
    if response.success:
        print('collections:')
        for collection in response.collections:
            print('  {}'.format(collection))
        return

    failure_code_str = _LISTCOLLECTIONSRESPONSE_FAILURECODE.values_by_number[
        response.failure_code].name
    print('list collections failed: ' + failure_code_str)


def insert_user(stub, database, collection, id, first_name, last_name, age):
    user = User()
    user.id = id
    user.first_name = first_name
    user.last_name = last_name
    user.age = age

    request = InsertObjectRequest()
    request.database = database
    request.collection = collection
    request.object = user.SerializeToString()

    response = stub.InsertObject(request)

    print()
    if response.success:
        print("user {} successfully inserted".format(id))
        return

    if response.failure_code != InsertObjectResponse.OBJECT_ERROR:
        failure_code_str = _INSERTOBJECTRESPONSE_FAILURECODE.values_by_number[
            response.failure_code].name
        print('insert failed: ' + failure_code_str)
        return

    print("user {} already exists".format(id))

def find_user(stub, database, collection, id):
    request = FindObjectRequest()
    request.database = database
    request.collection = collection
    request.id = id

    response = stub.FindObject(request)

    print()
    if response.success:
        user = User()
        user.ParseFromString(response.object)
        print("found user {}:".format(id))
        print("  id: {}".format(user.id))
        print("  first name: {}".format(user.first_name))
        print("  last name: {}".format(user.last_name))
        print("  age: {}".format(user.age))
        return

    print("find user failed: {}".format(response))


if __name__ == '__main__':
    with grpc.insecure_channel('localhost:10000') as channel:
        stub = protodb_pb2_grpc.ProtoDBStub(channel)
        create_database(stub, 'foo')
        list_databases(stub)
        create_collection(stub, 'foo', 'users', User.DESCRIPTOR)
        list_collections(stub, 'foo')
        insert_user(stub, 'foo', 'users', 1, 'john', 'doe', 30)
        find_user(stub, 'foo', 'users', 1)
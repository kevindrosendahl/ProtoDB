import grpc

from google.protobuf.descriptor_pb2 import DescriptorProto

from server.database_list_pb2 import ListDatabases

from server.database_create_pb2 import CreateDatabase, CreateDatabaseResponse
from server.database_create_pb2 import _CREATEDATABASERESPONSE_FAILURECODE

from server.collection_create_pb2 import CreateCollection
from server.collection_create_pb2 import CreateCollectionResponse
from server.collection_create_pb2 import _CREATECOLLECTIONRESPONSE_FAILURECODE

from server.insert_pb2 import Insert, InsertResponse
from server.insert_pb2 import _INSERTRESPONSE_FAILURECODE

from server.find_pb2 import Find, FindResponse, Query
from server.find_pb2 import _FINDRESPONSE_FAILURECODE

from server import server_pb2
from user_pb2 import User


def list_databases(stub):
    list_databases_response = stub.ListDatabases(ListDatabases())
    for database in list_databases_response.database:
        print "got database: " + database


def create_database(stub, db_name):
    create_database_request = CreateDatabase()
    create_database_request.name = db_name
    create_database_response = stub.CreateDatabase(create_database_request)

    if create_database_response.success:
        print "create database succeeded!"
    else:
        if create_database_response.failure_code == CreateDatabaseResponse.DB_EXISTS:
            print "database already exists"
        else:
            failure_code_str = _CREATEDATABASERESPONSE_FAILURECODE.values_by_number[
                create_database_response.failure_code].name
            print "create database failed: " + failure_code_str

def create_collection(stub, db_name, collection_name, schema):
    create_collection_request = CreateCollection()
    create_collection_request.database = db_name
    create_collection_request.name = collection_name

    descriptor_proto = DescriptorProto()
    schema.CopyToProto(descriptor_proto)
    print descriptor_proto
    create_collection_request.schema.MergeFrom(descriptor_proto)
    create_collection_response = stub.CreateCollection(create_collection_request)

    if create_collection_response.success:
        print "create collection succeeded!"
    else:
        if create_collection_response.failure_code == CreateCollectionResponse.DB_EXISTS:
            print "collection already exists"
        else:
            failure_code_str = _CREATECOLLECTIONRESPONSE_FAILURECODE.values_by_number[
                create_collection_response.failure_code].name
            print "create collection failed: " + failure_code_str

def insert_user(stub, db_name, collection_name, first_name, last_name, age):
    user = User()
    user.first_name = first_name
    user.last_name = last_name
    user.age = age

    insert_request = Insert()
    insert_request.database = db_name
    insert_request.collection = collection_name
    insert_request.data = user.SerializeToString()

    insert_response = stub.Insert(insert_request)

    if insert_response.success:
        print "insert succeeded!"
        print "_id: " + str(insert_response._id)
    else:
        failure_code_str = _INSERTRESPONSE_FAILURECODE.values_by_number[
            insert_response.failure_code].name
        print "insert failed: " + failure_code_str

def find_all(stub, db_name, collection_name):
    find_request = Find()
    find_request.database = db_name
    find_request.collection = collection_name

    find_response = stub.Find(find_request)

    if find_response.success:
        print "find all succeeded!"
        print "num found: " + str(find_response.num_found)
        user = User()
        for user_data in find_response.data:
            user.Clear()
            user.MergeFromString(user_data)
            print user
    else:
        failure_code_str = _FINDRESPONSE_FAILURECODE.values_by_number[
            find_response.failure_code].name
        print "find all failed: " + failure_code_str

def find_one(stub, db_name, collection_name, _id):
    find_request = Find()
    find_request.database = db_name
    find_request.collection = collection_name

    query = Query()
    query_field_option = Query.QueryFieldOptions()
    query_field_option.field = "_id"
    query_field_option.uint64_value = _id
    query_field_option.operator = Query.QueryFieldOptions.EQUAL
    query.query_field_options.extend([query_field_option])

    find_request.query.MergeFrom(query)

    find_response = stub.Find(find_request)

    if find_response.success:
        print "find one succeeded!"
        print "num found: " + str(find_response.num_found)
        user = User()
        for user_data in find_response.data:
            user.Clear()
            user.MergeFromString(user_data)
            print user
    else:
        failure_code_str = _FINDRESPONSE_FAILURECODE.values_by_number[
            find_response.failure_code].name
        print "find one failed: " + failure_code_str


def run():
    channel = grpc.insecure_channel('localhost:8888')
    stub = server_pb2.ProtoDBStub(channel)

    list_databases(stub)

    create_database(stub, "foo")

    create_collection(stub, "foo", "users", User.DESCRIPTOR)

    insert_user(stub, "foo", "users", "kevin", "rosendahl", 25)
    insert_user(stub, "foo", "users", "gary", "mcmuppet", 42)

    find_all(stub, "foo", "users")

    find_one(stub, "foo", "users", 1)

if __name__ == '__main__':
    run()

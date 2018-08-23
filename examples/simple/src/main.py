import grpc

from protodb import protodb_pb2_grpc

from protodb.database_create_pb2 import CreateDatabaseRequest, CreateDatabaseResponse
from protodb.database_create_pb2 import _CREATEDATABASERESPONSE_FAILURECODE

from protodb.database_list_pb2 import ListDatabasesRequest, ListDatabasesResponse


def create_database(stub, db_name):
    request = CreateDatabaseRequest()
    request.name = db_name
    response = stub.CreateDatabase(request)

    if response.success:
        print('create database succeeded!')
        return

    if response.failure_code == CreateDatabaseResponse.DB_EXISTS:
        print('database already exists')
        return

    failure_code_str = _CREATEDATABASERESPONSE_FAILURECODE.values_by_number[
         response.failure_code].name
    print('create database failed: {}', failure_code_str)


def list_databases(stub):
    request = ListDatabasesRequest()
    response = stub.ListDatabases(request)

    print('got databases: {}', response.database)


if __name__ == '__main__':
    with grpc.insecure_channel('localhost:10000') as channel:
        stub = protodb_pb2_grpc.ProtoDBStub(channel)
        create_database(stub, 'foo')
        list_databases(stub)


import grpc

from protodb import protodb_pb2_grpc

from protodb.database_create_pb2 import CreateDatabaseRequest, CreateDatabaseResponse
from protodb.database_create_pb2 import _CREATEDATABASERESPONSE_FAILURECODE


def create_database(stub, db_name):
    create_database_request = CreateDatabaseRequest()
    create_database_request.name = db_name
    create_database_response = stub.CreateDatabase(create_database_request)

    if create_database_response.success:
        print('create database succeeded!')
        return

    if create_database_response.failure_code == CreateDatabaseResponse.DB_EXISTS:
        print('database already exists')
        return

    failure_code_str = _CREATEDATABASERESPONSE_FAILURECODE.values_by_number[
         create_database_response.failure_code].name
    print('create database failed: {}', failure_code_str)


if __name__ == '__main__':
    with grpc.insecure_channel('localhost:10000') as channel:
        stub = protodb_pb2_grpc.ProtoDBStub(channel)
        create_database(stub, 'foo')


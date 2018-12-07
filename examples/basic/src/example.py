from random import randint

from protodb.database.create_pb2 import CreateDatabaseResponse
from protodb.database.list_pb2 import ListDatabasesResponse
from protodb.collection.create_pb2 import CreateCollectionResponse
from protodb.collection.list_pb2 import ListCollectionsResponse
from protodb.object.insert_pb2 import InsertObjectResponse
from protodb.object.find_pb2 import FindObjectResponse

from client import Client
from user_pb2 import User


DATABASE_NAME = 'dev'
COLLECTION_NAME = 'users'


def main():
    client = Client()

    create_database(client)
    list_databases(client)

    create_users_collection(client)
    list_collections(client)

    insert_user(client, id=1, first_name='john', last_name='doe', age=40)
    insert_user(client, id=2, first_name='jane', last_name='doe', age=40)
    insert_user(client, id=2, first_name='jane', last_name='doe', age=40)

    print('\ninserting 1000 users')
    for i in range(3, 1001):
        age = randint(25, 75)
        insert_user(client, verbose=False, id=i, first_name='jane', last_name='doe', age=age)

    find_user(client, 1)
    find_user(client, 1000)
    find_user(client, 1001)


def create_database(client):
    print('creating database')
    response = client.create_database(DATABASE_NAME)

    if response.error_code == CreateDatabaseResponse.NO_ERROR:
        print('create database succeeded!')
        return

    error_code_str = CreateDatabaseResponse.ErrorCode.Name(response.error_code)
    print('create database failed: {}'.format(error_code_str))


def list_databases(client):
    print('\nlisting databases')
    response = client.list_databases()

    if response.error_code == ListDatabasesResponse.NO_ERROR:
        print('databases:')
        for database in response.databases:
            print ('  {}'.format(database))

        return

    error_code_str = ListDatabasesResponse.ErrorCode.Name(response.error_code)
    print('list database failed: {}'.format(error_code_str))


def create_users_collection(client):
    print('\ncreating users collection')
    response = client.create_collection(DATABASE_NAME, COLLECTION_NAME, User.DESCRIPTOR)

    if response.error_code == CreateCollectionResponse.NO_ERROR:
        print('create collection succeeded!')
        return

    error_code_str = CreateCollectionResponse.ErrorCode.Name(response.error_code)
    print('create collection failed: {}'.format(error_code_str))
    print(response.schema_error)


def list_collections(client):
    print('listing collections')
    response = client.list_collections(DATABASE_NAME)

    if response.error_code == ListCollectionsResponse.NO_ERROR:
        print('collections:')
        for collection in response.collections:
            print('  {}'.format(collection))
        return

    error_code_str = ListCollectionsResponse.ErrorCode.Name(response.error_code)
    print('list collections failed: ' + error_code_str)


def insert_user(client, verbose=True, **user_kwargs):
    id = user_kwargs['id']

    if verbose:
        print('\ninserting user {}'.format(id))

    user = User(**user_kwargs)

    response = client.insert_user(DATABASE_NAME, COLLECTION_NAME, user)

    if response.error_code == InsertObjectResponse.NO_ERROR:
        if verbose:
            print('user {} successfully inserted'.format(id))
        return

    if response.error_code != InsertObjectResponse.OBJECT_ERROR:
        error_code_str = InsertObjectResponse.ErrorCode.Name(response.error_code)
        print('insert user {} failed: '.format(id) + error_code_str)
        return

    print('insert user {} failed: '.format(id) + response.object_error)


def find_user(client, id):
    print('\nfinding user {}'.format(id))
    response = client.find_user(DATABASE_NAME, COLLECTION_NAME, id)

    if response.error_code == FindObjectResponse.NO_ERROR:
        user = User()
        user.ParseFromString(response.object)
        print('''found user {}:
    id: {}
    first name: {}
    last name: {}
    age: {}'''.format(id,
                      user.id,
                      user.first_name,
                      user.last_name,
                      user.age))
        return

    error_code_str = FindObjectResponse.ErrorCode.Name(response.error_code)
    print('find user failed: ' + error_code_str)


if __name__ == '__main__':
    main()

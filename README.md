ProtoDB
=======

[![Build Status](https://travis-ci.org/kevindrosendahl/ProtoDB.svg?branch=master)](https://travis-ci.org/kevindrosendahl/ProtoDB)

## What is this?
ProtoDB is an ***experimental*** database exploring:
- writing a database in Rust
- using protocol buffers as the schema definition and storage format
- using grpc as the transport layer
- exposing some database internals via WebAssembly

## Current Status

The full ProtoDB API can be found in [protodb.proto](proto/protodb/protodb.proto).

ProtoDB supports
- database creation
- collection creation
    - schemas defined by `DescriptorProto`
    - currently schemas may only include [scalar types](https://developers.google.com/protocol-buffers/docs/proto3#scalar) or [enumerations](https://developers.google.com/protocol-buffers/docs/proto3#enum), and must contain a `uint64` `id` field.
- object insertion
- object retrieval by `id`
- index creation
    - only on integer type fields
    - only ascending
    - only on a single field
- WebAssembly stored procedures (see [below](#webassembly))

## Examples

For the examples, we define a simple [`User`](examples/protos/user.proto) protocol buffer message:

```proto
message User {
     uint64 id = 1;

     string first_name = 2;
     string last_name = 3;
     uint32 age = 4;
     string email_address = 5;
}
```

### Basic Python script

[`example.py`](examples/basic/example.py) provides a simple script showing how to do create a database `dev` with a collection `users` (which has `User` as its schema), as well as inserting and retrieving some users.

Note specifically that the `User` class was generated from `user.proto`, and that the `protodb.client.Client` used is mostly auto generated from `protodb.proto`.

`example.py` shows how simple it is to store and retrieve objects with ProtoDB:

```python
def insert_user(client, id):
    ...
    user = User(id=id, first_name=first_name, last_name=last_name, age=age, email_address=email_address)

    response = client.insert_object(DATABASE_NAME, COLLECTION_NAME, user)

    if response.error_code == InsertObjectResponse.NO_ERROR:
        print('user {} successfully inserted'.format(id))
    ...


def find_user(client, id):
    response = client.find_object(DATABASE_NAME, COLLECTION_NAME, id)

    if response.error_code == FindObjectResponse.NO_ERROR:
        user = User()
        user.ParseFromString(response.object)
        
        format_str = '''found user {}:
    id: {}
    first name: {}
    last name: {}
    age: {}
    email address: {}'''
        print(format_str.format(id,
                                user.id,
                                user.first_name,
                                user.last_name,
                                user.age,
                                user.email_address))
```

[![asciicast](https://asciinema.org/a/216904.svg)](https://asciinema.org/a/216904)

### WebAssembly

ProtoDB allows you to write WebAssembly modules that interface with ProtoDB and run them on the server.

In addition to the `User` protocol buffer message, we also define [`UserStatistics`](examples/protos/user_statistics.proto):

```proto
message UserStatistics {
    uint32 num_users = 1;

    double age_mean = 2;
    double age_std_dev = 3;
    double age_variance = 4;
    uint64 age_cardinality = 5;

    string youngest_email_address = 6;
}
```

We write our WebAssembly module as a [Rust crate](examples/wasm), with a `run` method that takes in a trait object that provides access to the database.

As we see below, this example iterates over an index with id `1`, which produces tuples of the user's age and their id. It looks up the youngest user's object and gets its age, and calculates some statistics about the distribution of ages across users. It then stores this information in a `UserStatistics` struct and returns the encoded bytes for that message.

```rust
fn run(&mut self, protodb: impl ProtoDB) -> Vec<u8> {
    ...
    for (value, id) in protodb.index_iter("users", 1, None) {
        if youngest_email_address.is_none() {
            let user = protodb.find_object("users", id).unwrap();
            let user = users::User::decode(user).unwrap();
            youngest_email_address = Some(user.email_address);
        }

        num_users += 1;
        frequencies.add(value);
        online_states.add(value);
    }
    ...
    let statistics = users::UserStatistics {
        num_users,
        age_cardinality: frequencies.cardinality(),
        age_mean: online_states.mean(),
        age_std_dev: online_states.stddev(),
        age_variance: online_states.variance(),
        youngest_email_address: youngest_email_address.unwrap(),
    };
    let mut ret = Vec::with_capacity(statistics.encoded_len());
    statistics.encode(&mut ret).unwrap();
    ret
}
```

Below we create the index first by opening a Python interpreter and using the ProtoDB client to create an index on field `4` (`age`). We then register the WebAssembly module with ProtoDB and run it using `protoctl`.

Registering the WebAssembly module consists of compiling the crate into WebAssembly binary, collecting some metadata, and storing the information in the database.

[![asciicast](https://asciinema.org/a/216912.svg)](https://asciinema.org/a/216912)

## Building

ProtoDB requires a Rust 2018 compatible compiler (>=1.31.0).

## Components

### ProtoDB
ProtoDB is the actual database.

To run ProtoDB with some helpful logging, run

```bash
$ RUST_LOG=protodb=debug cargo run --release
```

### protoctl

`protoctl` is a command line tool for interacting with a ProtoDB instance.

First build protoctl, then run it by executing the produced binary:

```bash
$ cargo build -p protoctl --release
$ ./target/release/protoctl --help
protoctl 0.1.0
Kevin Rosendahl <kevindrosendahl@gmail.com>

USAGE:
  protoctl <SUBCOMMAND>

FLAGS:
  -h, --help       Prints help information
  -V, --version    Prints version information

SUBCOMMANDS:
  collection
  database
  help          Prints this message or the help of the given subcommand(s)
  object
  wasm
```

### Clients

Currently, there are provided [Python](clients/python) and [Rust](clients/rust) clients.

### WebAssembly

The [`protodb-wasm`](crates/wasm) crate provides a library for writing WebAssembly modules that can be run from within ProtoDB. See the [example](#webassembly) for more information.

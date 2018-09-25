ProtoDB
=======


## What is this?
ProtoDB is an experimental database exploring:
- writing a database in Rust
- using protocol buffers as the schema definition and storage format
- using grpc as the transport layer

## Implementation roadmap

The `master` branch contains a first stab at implementing `ProtoDB` which was able to create collections with a schema, and persist validated protobufs in the collections.

However, the branch became stable and was no longer able to build. In addition, the code was poorly factored and designed. This branch represents a new start.

Short term goals:
- implement in-memory storage engine
  - likely want to write a higher level key-value storage engine that contains an inner `KVStorageEngine` implementation and implements `StorageEngine`
  - this way it could be used for other future `KVStorageEngine`s such as `RocksDB`
- database/collection CRUD
- simple CRUD on objects in a single collection
  - insert
  - find(id)/find()
  - update(id, fields)
  - delete(id)

Longer term goals:
- persistence
  - probably use `RocksDB`
  - keep an eye on [`sled`](https://github.com/spacejam/sled)
  - use these to flesh out `StorageEngine` and `KVStorageEngine`


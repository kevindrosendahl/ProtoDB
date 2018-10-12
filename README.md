ProtoDB
=======

[![Build Status](https://travis-ci.org/kevindrosendahl/ProtoDB.svg?branch=master)](https://travis-ci.org/kevindrosendahl/ProtoDB)

## What is this?
ProtoDB is an ***experimental*** database exploring:
- writing a database in Rust
- using protocol buffers as the schema definition and storage format
- using grpc as the transport layer
- exposing some database internals via WebAssembly

## Status

Currently, ProtoDB supports simple insertion and find by `id`, backed by either an in-memory storage engine or `RocksDB`.

Currently schemas may only include [scalar types](https://developers.google.com/protocol-buffers/docs/proto3#scalar) or [enumerations](https://developers.google.com/protocol-buffers/docs/proto3#enum).

Please see the [example](examples/python/simple).

## Roadmap

The current short term goals are:
- finish RocksDB integration (currently database/collection info is not persisted, but objects are)
- add `Index` trait and implement for scalar types

From there, the next area of focus will be attempting to allow users to write Rust code that can consume an `Index`, compile it to WebAssembly, register it with the database, and run it from within the database in a VM.

Once this is either working or deemed too hard currently, concurrency control will likely be the next focus.
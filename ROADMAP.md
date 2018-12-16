# Roadmap

This document contains some ideas about features or fixes I would like to add/make.

## Namespaces

Instead of having a strict two-layered database -> collection hierarchy, make it (reasonably) deeply nestable.

For example, instead of creating the `users` collection in the `prod` database, you would just make `/prod/users`.

## More DDL operations

Delete namespaces/indexes.

## Mutable objects

Allow object fields to be updated.

## Non-integer type indexes

Can use something like the following to index floating point fields: https://github.com/google/guava/issues/2834

Could pretty easily support exact-match on string/bytes.

## Multi field indexes

## Embedded objects/arrays

Support these in namespace schemas.

## Multi-object retrieval

No supported way to do even a `find_all` right now. Should support `find_all` at least, and should support paginated responses.

## Higher level generation

Could based off of a schema, generate:
- clients
  - `user = client.get_user(5)` instead of `user.ParseFromString(client.find_object('dev', 'users', 5).object)`
- API servers
  - generate a gRPC server that can have plugable middleware proxying access to database objects
  
## Example app

Create a Todo app or something actually using ProtoDB and maybe `grpc-web`.
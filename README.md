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
- indexes and query planner
  - more interesting queries
- concurrency
  - add lock manager
  - multi granular lock implemented already in [`src/util/sync/multi_granular_lock.rs`](https://github.com/kevindrosendahl/ProtoDB/blob/revamp/src/util/sync/multi_granular_lock.rs)

Open ended goals:
- cross-collection relations
  - could potentially use protobuf options. e.g.:
```proto
message User {
    option (protodb).collection.name = "users";
    uint64 id = 1 [(protodb).primary_key = true];
    
    string first_name = 2;
    string last_name = 3;
    
    bool verified = 4;
}

message Post {
    option (protodb).collection.name = "posts";
    uint64 id = 1 [(protodb).primary_key = true];
    
    User author = 2 [(protodb).foreign_key.collection = "users", 
                     (protodb).foreign_key.field = 1];
    string content = 3;

    message Comment {
        User author = 1 [(protodb).foreign_key.collection = "users", 
                         (protodb).foreign_key.field = 1,
                         (protodb).foreign_key.lazy = true];
        string content = 2;
    }
    repeated Comment comments = 4;
}
```
- schema changes
- schema specific generated db client. for the above:
```python
import my_generated_client as client

db = client.my_db
user = db.users.get(1)
posts = db.posts.posts_for_author(user)

for post in posts:
    for comment in post.comments:
        author = comment.author.load()
        print(
            "comment {} on post {} is by {} {}".format(
                comment.id,
                post.id,
                author.first_name,
                author.last_name,
            )
        )

for user in db.users.watch().filter(lambda u: u.verified):
    print("new verified user: {} {}".format(user.first_name, user.last_name))
```
- replication
- sharding
- server push
  - allow clients to watch values and be notified when values change
- plugin model
  - how much of the above can/should be implemented as plugins
- webassembly stored procedures

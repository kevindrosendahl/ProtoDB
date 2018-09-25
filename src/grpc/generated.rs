mod generated {
    include!(concat!(env!("OUT_DIR"), "/protodb.rs"));
}

pub use self::generated::*;

pub mod request {
    pub use super::generated::{
        CreateCollectionRequest as CreateCollection,
        CreateDatabaseRequest as CreateDatabase,
        ListDatabasesRequest as ListDatabases,
    };
}

pub mod response {
    pub use super::generated::{
        CreateCollectionResponse as CreateCollection,
        CreateDatabaseResponse as CreateDatabase,
        ListDatabasesResponse as ListDatabases,
    };
}

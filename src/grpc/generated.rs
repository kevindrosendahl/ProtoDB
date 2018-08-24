mod generated {
    include!(concat!(env!("OUT_DIR"), "/protodb.rs"));
}

pub use self::generated::*;

pub mod request {
    pub use super::generated::{
        CreateDatabaseRequest as CreateDatabase, ListDatabasesRequest as ListDatabases,
    };
}

pub mod response {
    pub use super::generated::{
        CreateDatabaseResponse as CreateDatabase, ListDatabasesResponse as ListDatabases,
    };
}

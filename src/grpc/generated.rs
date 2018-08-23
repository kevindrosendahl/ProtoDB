mod generated {
    include!(concat!(env!("OUT_DIR"), "/protodb.rs"));
}

pub use self::generated::*;

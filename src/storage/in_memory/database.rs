use std::sync::{Arc, Mutex};

pub struct Database {
    collection_id_counter: Arc<Mutex<u64>>,
}
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex, RwLock},
};

use prost_types::DescriptorProto;

pub struct Database {
    collection_id_counter: Arc<Mutex<u64>>,
    collections: Arc<RwLock<BTreeMap<String, Collection>>>,
}

pub struct Collection {
    name: string,
    schema: DescriptorProto,
}

use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use super::collection::Collection;

#[derive(Default)]
pub struct Database {
    pub collections: Arc<RwLock<BTreeMap<String, Collection>>>,
}

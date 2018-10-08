use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use super::collection::Collection;

#[derive(Default)]
pub(crate) struct Database {
    pub name: String,
    pub collections: Arc<RwLock<BTreeMap<String, Collection>>>,
}

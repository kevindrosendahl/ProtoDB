pub mod errors;

use crate::{schema::DecodedIdObject, storage::errors::InternalStorageEngineError};

use protodb_schema::encoding::FieldValue;

pub trait IndexAccessMethod {
    fn build(
        &self,
        objects: Box<dyn Iterator<Item = Result<Vec<u8>, InternalStorageEngineError>>>,
    ) -> Result<(), errors::BuildIndexError>;

    // FIXME: this should not be the responsibility of the AccessMethod, maybe add back IndexCatalogEntry.
    fn field(&self) -> i32;

    fn insert(&self, obj: DecodedIdObject) -> Result<(), errors::IndexInsertError>;

    fn iter(&self, mode: IteratorMode) -> Box<dyn Iterator<Item = (FieldValue, u64)>>;
}

pub enum Direction {
    Backward,
    Forward,
}

pub struct IteratorMode {
    pub direction: Direction,
    pub from: Option<FieldValue>,
}

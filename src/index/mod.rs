pub mod errors;

use crate::schema::DecodedIdObject;

pub trait IndexAccessMethod {
    fn build(&self) -> Result<(), errors::BuildIndexError>;

    fn insert(&self, obj: DecodedIdObject) -> Result<(), errors::IndexInsertError>;

    fn iter(&self) -> Box<dyn IndexAccessMethodIterator<Item = u64>>;
}

pub enum Direction {
    Backward,
    Forward,
}

pub enum IteratorMode {
    Direction(Direction),
    From((u64, Direction)),
}

pub trait IndexAccessMethodIterator: Iterator<Item = u64> {
    fn set_mode(&mut self, mode: IteratorMode);
}

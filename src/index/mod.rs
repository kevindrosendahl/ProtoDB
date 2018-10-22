pub mod errors;

use crate::storage::errors::InternalStorageEngineError;

// TODO: this is really a sorted index interface, should probably split that up into a
//       pub trait SortedIndexAccessMethod<T> : IndexAccessMethod where T: Ord
//       and remove the T: Ord on this trait.
pub trait IndexAccessMethod<T>
where
    T: Ord,
{
    fn insert(&self, id: u64, val: T) -> Result<(), errors::IndexInsertError>;

    fn find_one(&self, val: T) -> Result<Option<u64>, InternalStorageEngineError>;

    fn find(&self, val: T) -> Box<dyn Iterator<Item = u64>>;

    fn iter(&self) -> Box<dyn IndexAccessMethodIterator<Item = (u64, T)>>;
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

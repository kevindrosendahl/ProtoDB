extern crate wasm_bindgen;
pub use wasm_bindgen::prelude::*;

pub mod prelude {
    pub use wasm_bindgen::prelude::*;
}

pub trait Module {
    // FIXME: should return a Result<Vec<u8>, Error>
    // TODO: should also have an iterator implementation (maybe IteratorModule? is this an AggregationModule?)
    fn run(&mut self, protodb: impl ProtoDB) -> Vec<u8>;
}

pub trait ProtoDB {
    fn find_objects(&self, collection: &str) -> Box<dyn Iterator<Item = Vec<u8>>>;

    fn find_object(&self, collection: &str, id: u64) -> Option<Vec<u8>>;
}

#[wasm_bindgen]
extern "C" {
    pub fn log(message: &str);

    fn find_objects_iter(collection: &str) -> usize;

    // FIXME: should return a Result<Option<Vec<u8>>, Error>
    fn find_objects_iter_next(id: usize) -> Option<Vec<u8>>;

    fn find_object(collection: &str, id: u64) -> Option<Vec<u8>>;
}

#[doc(hidden)]
pub struct ProtoDBImpl;

impl ProtoDB for ProtoDBImpl {
    fn find_objects(&self, collection: &str) -> Box<dyn Iterator<Item = Vec<u8>>> {
        let id = find_objects_iter(collection);
        let iter = ProtoDBFindObjectsIterator { id };
        Box::new(iter) as Box<dyn Iterator<Item = Vec<u8>>>
    }

    fn find_object(&self, collection: &str, id: u64) -> Option<Vec<u8>> {
        find_object(collection, id)
    }
}

struct ProtoDBFindObjectsIterator {
    id: usize,
}

impl Iterator for ProtoDBFindObjectsIterator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        find_objects_iter_next(self.id)
    }
}

#[macro_export]
macro_rules! protodb_wasm_module {
	( $ty:ty, $impl:expr ) => {
		use std::cell::RefCell;

		fn __assert_valid_module() where $ty: Module {
			// This error means that your supplied type does not implement protodb_wasm::Module
		}

		thread_local!(static MODULE: RefCell<$ty> = RefCell::new($impl));

		#[wasm_bindgen]
		pub fn run() -> Vec<u8> {
		    let protodb = protodb_wasm::ProtoDBImpl;
			MODULE.with(|module_cell| {
				module_cell.borrow_mut().run(protodb)
			})
		}
	}
}

extern crate wasm_bindgen;

pub mod prelude {
    pub use wasm_bindgen::prelude::*;
}

pub trait Module {
    fn run(&mut self, protodb: impl ProtoDB) -> Vec<u8>;
}

pub trait ProtoDB {
    fn get_object(&self, collection: &str, id: u64) -> Option<Vec<u8>>;
}

#[macro_export]
macro_rules! protodb_wasm_module {
	( $ty:ty, $impl:expr ) => {
		use std::cell::RefCell;

		#[wasm_bindgen]
		extern "C" {
		    fn get_object(collection: &str, id: u64) -> Option<Vec<u8>>;

		    fn log(message: &str);
		}

		struct ProtoDBImpl;

		impl ProtoDB for ProtoDBImpl {
		    fn get_object(&self, collection: &str, id: u64) -> Option<Vec<u8>> {
		        log("in get object");
		        get_object(collection, id)
		    }
		}

		fn __assert_valid_module() where $ty: Module {
			// This error means that your supplied type does not implement protodb_wasm::Module
		}

		thread_local!(static MODULE: RefCell<$ty> = RefCell::new($impl));

		#[wasm_bindgen]
		pub fn run() -> Vec<u8> {
		    log("getting impl");
		     let protodb = ProtoDBImpl{};
		    log("got impl");
			MODULE.with(|module_cell| {
			    log("calling run");
				module_cell.borrow_mut().run(protodb)
			})
		}
	}
}

extern crate wasm_bindgen;

pub mod prelude {
    pub use wasm_bindgen::prelude::*;
}

pub trait Module {
    fn run(&mut self) -> Vec<u8>;
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
			MODULE.with(|module_cell| {
				module_cell.borrow_mut().run()
			})
		}
	}
}

extern crate protodb_wasm;
extern crate wasm_bindgen;

use protodb_wasm::prelude::*;
use protodb_wasm::{protodb_wasm_module, Module};

struct MyModule;

impl MyModule {
    fn new() -> Self {
        MyModule
    }
}
impl Module for MyModule {
    fn run(&mut self) -> Vec<u8> {
        vec![1, 2, 3, 4]
    }
}

protodb_wasm_module!(MyModule, MyModule::new());

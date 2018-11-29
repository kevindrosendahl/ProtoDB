extern crate protodb_wasm;
extern crate wasm_bindgen;

use protodb_wasm::prelude::*;
use protodb_wasm::{protodb_wasm_module, Module, ProtoDB};

struct MyModule;

impl MyModule {
    fn new() -> Self {
        MyModule
    }
}
impl Module for MyModule {
    fn run(&mut self, protodb: impl ProtoDB) -> Vec<u8> {
        log("they called run");
        protodb.get_object("users", 1).unwrap()
    }
}

protodb_wasm_module!(MyModule, MyModule::new());

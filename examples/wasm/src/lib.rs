extern crate protodb_wasm;
extern crate wasm_bindgen;

use protodb_wasm::log;
use protodb_wasm::prelude::*;
use protodb_wasm::{protodb_wasm_module, Module, ProtoDB};

struct MyModule;

impl Module for MyModule {
    fn run(&mut self, protodb: impl ProtoDB) -> Vec<u8> {
        log("trying to find object 1");
        protodb.find_object("users", 1).unwrap()
    }
}

protodb_wasm_module!(MyModule, MyModule {});

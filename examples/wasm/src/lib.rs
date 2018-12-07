extern crate protodb_wasm;
extern crate wasm_bindgen;

use protodb_wasm::log;
use protodb_wasm::prelude::*;
use protodb_wasm::{protodb_wasm_module, Module, ProtoDB};

struct MyModule;

impl Module for MyModule {
    fn run(&mut self, protodb: impl ProtoDB) -> Vec<u8> {
        log("trying to find objects");

        let mut count = 0;
        for (idx, object) in protodb.find_objects("users").enumerate() {
            log(&format!("object #{}: {:?}", idx, object));
            count += 1;
        }

        vec![count]
    }
}

protodb_wasm_module!(MyModule, MyModule {});

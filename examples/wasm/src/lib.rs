extern crate prost;
extern crate protodb_wasm;
extern crate wasm_bindgen;
#[macro_use]
extern crate prost_derive;

use prost::Message;
use protodb_wasm::log;
use protodb_wasm::prelude::*;
use protodb_wasm::{protodb_wasm_module, Module, ProtoDB};

pub mod users {
    include!(concat!(env!("OUT_DIR"), "/protodb.examples.user.rs"));
    include!(concat!(
        env!("OUT_DIR"),
        "/protodb.examples.user_statistics.rs"
    ));
}

struct UserAgeAverage;

impl Module for UserAgeAverage {
    fn run(&mut self, protodb: impl ProtoDB) -> Vec<u8> {
        let mut num_users = 0;
        let mut age = 0;
//        let mut user = users::User::default();

        for (idx, object) in protodb.find_objects("users").enumerate() {
//            user.clear();
//            user.merge(object).unwrap_or_else(|err| {
//                log(&format!("got error decoding user {}: {:?}", idx, err));
//                panic!(err)
//            });

            num_users += 1;
//            age += user.age;
            age += 1;
        }

        let average_age = (age as f64) / (num_users as f64);

        log(&format!("found {} users", num_users));
        log(&format!("average age: {}", average_age));

        let statistics = users::UserStatistics {
            num_users,
            average_age,
        };
        let mut ret = Vec::with_capacity(statistics.encoded_len());
        statistics.encode(&mut ret).unwrap();
        ret
    }
}

protodb_wasm_module!(UserAgeAverage, UserAgeAverage {});

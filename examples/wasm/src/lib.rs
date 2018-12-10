extern crate prost;
extern crate protodb_wasm;
extern crate wasm_bindgen;
#[macro_use]
extern crate prost_derive;

use prost::Message;
use protodb_wasm::log;
use protodb_wasm::prelude::*;
use protodb_wasm::{protodb_wasm_module, Module, ProtoDB};
use stats::{Frequencies, OnlineStats};

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
        let mut frequencies = Frequencies::new();
        let mut online_states = OnlineStats::new();

        let mut num_users = 0;
        let mut youngest_email_address = None;
        for (value, id) in protodb.index_iter("users", 1, None) {
            if youngest_email_address.is_none() {
                let user = protodb.find_object("users", id).unwrap();
                let user = users::User::decode(user).unwrap();
                youngest_email_address = Some(user.email_address);
            }

            num_users += 1;
            frequencies.add(value);
            online_states.add(value);
        }

        log(&format!("found {} users", num_users));

        let statistics = users::UserStatistics {
            num_users,
            age_cardinality: frequencies.cardinality(),
            age_mean: online_states.mean(),
            age_std_dev: online_states.stddev(),
            age_variance: online_states.variance(),
            youngest_email_address: youngest_email_address.unwrap(),
        };
        let mut ret = Vec::with_capacity(statistics.encoded_len());
        statistics.encode(&mut ret).unwrap();
        ret
    }
}

protodb_wasm_module!(UserAgeAverage, UserAgeAverage {});

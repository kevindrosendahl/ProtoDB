extern crate libprotodb;

use libprotodb::server::schema::Schema;
use libprotodb::server::util;
use libprotodb::server::protos::collection;

fn main() {
    let descriptor =
        util::protobuf::descriptor_from_file_descriptor(collection::file_descriptor_proto(),
                                                        "Collection")
            .unwrap();
    let schema = Schema::new(descriptor);
    println!("Hello, world!");
    println!("Schema: {:?}", schema);
}

extern crate tower_grpc_build;
use std::fs;

fn main() {
    // Build protod
    let protos: Vec<String> = fs::read_dir("./proto/protodb")
        .unwrap()
        .map(|r| r.unwrap())
        .map(|d| {
            format!(
                "proto/protodb/{}",
                d.path().file_name().unwrap().to_str().unwrap()
            )
        })
        .collect();

    tower_grpc_build::Config::new()
        .enable_server(true)
        .build(&protos, &[String::from("proto")])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}

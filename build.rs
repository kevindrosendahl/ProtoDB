extern crate tower_grpc_build;
use std::fs;

fn main() {
    // Build protod
    let protod_paths = fs::read_dir("./proto/protod");
    let protod_files : Vec<String> = protod_paths.unwrap()
        .map(|r| r.unwrap())
        .map(|d| format!("proto/protod/{}", d.path().file_name().unwrap().to_str().unwrap()))
        .collect();

    tower_grpc_build::Config::new()
        .enable_server(true)
        .build(&protod_files, &[String::from("proto")])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}
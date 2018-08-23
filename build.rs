extern crate tower_grpc_build;

fn main() {
    // Build protod
    tower_grpc_build::Config::new()
        .enable_server(true)
        .build(&["proto/protod/database_create.proto", "proto/protod/protod.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}
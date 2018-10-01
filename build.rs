extern crate tower_grpc_build;
use std::fs;

fn main() {
    // Build protod
    let protos = read_protos_dirs(&vec![
        "./proto/protodb".to_string(),
        "./proto/protodb/collection".to_string(),
        "./proto/protodb/database".to_string(),
    ]);

    tower_grpc_build::Config::new()
        .enable_server(true)
        .enable_client(false)
        .build(&protos, &["proto".to_string()])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}

fn read_protos_dirs(dirs: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    for dir in dirs {
        result.extend(read_protos_dir(dir.to_string()))
    }

    result
}

fn read_protos_dir(dir: String) -> Vec<String> {
    fs::read_dir(dir.clone())
        .unwrap()
        .map(|r| r.unwrap())
        .filter(|d| d.file_type().unwrap().is_file())
        .map(|d| {
            format!(
                "{}/{}",
                dir,
                d.path().file_name().unwrap().to_str().unwrap(),
            )
        }).collect()
}

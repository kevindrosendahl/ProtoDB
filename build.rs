extern crate tower_grpc_build;
use std::{env, fs, io::prelude::*, process::Command};

fn main() {
    // Build protod
    let subdirectories = vec!["collection", "database", "index", "object", "wasm"];
    let mut dirs: Vec<String> = vec!["./proto/protodb".to_string()];
    dirs.extend::<Vec<String>>(
        subdirectories
            .iter()
            .map(|d| format!("./proto/protodb/{}", d))
            .collect(),
    );
    let protos = read_protos_dirs(&dirs);

    tower_grpc_build::Config::new()
        .enable_server(true)
        .enable_client(false)
        .build(&protos, &["proto".to_string()])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));

    fix_generated_grpc(&subdirectories);
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
        })
        .collect()
}

// temporary hack to get around https://github.com/tower-rs/tower-grpc/issues/85
fn fix_generated_grpc(subdirectories: &Vec<&str>) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut protodb_file = fs::File::open(format!("{}/protodb.rs", out_dir)).unwrap();
    let mut contents = String::new();
    protodb_file.read_to_string(&mut contents).unwrap();

    let split_pattern = "use ::tower_grpc::codegen::server::*;";
    let split: Vec<&str> = contents.split(split_pattern).collect();
    assert_eq!(split.len(), 4);

    let second = insert_imports(split[1].to_string(), subdirectories, 1);
    let third = insert_imports(split[2].to_string(), subdirectories, 2);
    let fourth = insert_imports(split[3].to_string(), subdirectories, 3);

    let sections = vec![split[0].to_string(), second, third, fourth];
    let mut fixed = sections.join(split_pattern);

    for dir in subdirectories.iter() {
        let mut file = fs::File::open(format!("{}/protodb.{}.rs", out_dir, dir)).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        fixed.push_str(&format!(
            "
pub mod {} {{
{}
}}",
            dir, contents
        ));
    }

    let output_path = format!(
        "{}/src/transport/grpc/generated/protodb.rs",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );
    let mut buffer = fs::File::create(output_path.clone()).unwrap();
    buffer.write(fixed.as_bytes()).unwrap();

    Command::new("rustfmt")
        .args(&[&output_path])
        .status()
        .expect("failed to run cargo fmt");
}

fn insert_imports(content: String, subdirectories: &Vec<&str>, level: i32) -> String {
    let mut split: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    for dir in subdirectories.iter() {
        let supers = (0..level).map(|_| "super::").collect::<String>();
        split.insert(1, format!("use {}{};", supers, dir))
    }

    return split.join("\n");
}

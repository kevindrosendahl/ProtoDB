use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

use crate::CLIENT;

use prost::Message;
use prost_types::FileDescriptorSet;
use protodb_client::generated::protodb::wasm::register_module_request::{
    module_metadata::BindgenImportHashes, ModuleMetadata,
};
use protodb_schema::{descriptor_fields, encoding::decoded_object};
use regex::Regex;

lazy_static! {
    static ref FIND_OBJECT_BINDGEN_RE: Regex =
        Regex::new("__wbg_findobject_([a-f0-9]{16})").unwrap();
    static ref FIND_OBJECT_ITER_BINDGEN_RE: Regex =
        Regex::new("__wbg_findobjectsiter_([a-f0-9]{16})").unwrap();
    static ref FIND_OBJECT_ITER_NEXT_BINDGEN_RE: Regex =
        Regex::new("__wbg_findobjectsiternext_([a-f0-9]{16})").unwrap();
    static ref INDEX_ITER_BINDGEN_RE: Regex = Regex::new("__wbg_indexiter_([a-f0-9]{16})").unwrap();
    static ref INDEX_ITER_NEXT_VALUE_BINDGEN_RE: Regex =
        Regex::new("__wbg_indexiternextvalue_([a-f0-9]{16})").unwrap();
    static ref INDEX_ITER_NEXT_ID_BINDGEN_RE: Regex =
        Regex::new("__wbg_indexiternextid_([a-f0-9]{16})").unwrap();
    static ref LOG_BINDGEN_RE: Regex = Regex::new("__wbg_log_([a-f0-9]{16})").unwrap();
}

#[derive(Debug, StructOpt)]
pub enum Wasm {
    #[structopt(name = "register")]
    Register {
        #[structopt(long = "database", short = "d")]
        database: String,

        #[structopt(long = "name", short = "n")]
        name: String,

        #[structopt(long = "crate", short = "s", parse(from_os_str))]
        crate_: PathBuf,

        #[structopt(long = "package", short = "p")]
        package: Option<String>,

        #[structopt(long = "schema-file", parse(from_os_str))]
        result_schema_file: PathBuf,

        #[structopt(long = "schema-message")]
        result_schema_message: String,

        #[structopt(long = "include", short = "i", parse(from_os_str))]
        result_includes: Vec<PathBuf>,
    },

    #[structopt(name = "run")]
    Run {
        #[structopt(long = "database", short = "d")]
        database: String,

        #[structopt(long = "name", short = "n")]
        name: String,
    },
}

pub fn run_wasm(wasm: Wasm) {
    match wasm {
        Wasm::Register {
            database,
            name,
            crate_,
            package,
            result_schema_file,
            result_schema_message,
            result_includes,
        } => register_module(
            database,
            name,
            &crate_,
            package,
            &result_schema_file,
            &result_schema_message,
            result_includes,
        ),
        Wasm::Run { database, name } => run_wasm_module(database, name),
    }
}

fn register_module(
    database: String,
    name: String,
    crate_: &PathBuf,
    package: Option<String>,
    result_schema_file: &PathBuf,
    result_schema_message: &str,
    result_includes: Vec<PathBuf>,
) {
    env::set_current_dir(crate_.clone()).unwrap();

    let tmp = tempdir::TempDir::new("protoctl-register-wasm-module").unwrap();
    let path = tmp.path();

    let mut cmd = Command::new(which::which("cargo").unwrap());
    cmd.arg("build")
        .arg("--target=wasm32-unknown-unknown")
        .arg("--release");

    let crate_name = match package {
        Some(package) => {
            cmd.arg(&format!("--package={}", package));
            package
        }
        None => crate_
            .as_path()
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap(),
    };
    let crate_name = crate_name.replace("-", "_");

    assert_eq!(cmd.spawn().unwrap().wait().unwrap().success(), true,);

    let mut cmd = Command::new(which::which("wasm-bindgen").unwrap());

    assert_eq!(
        cmd.arg(&format!(
            "target/wasm32-unknown-unknown/release/{}.wasm",
            crate_name,
        ))
        .arg("--out-dir")
        .arg(path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success(),
        true,
    );

    let mut file = File::open(path.join(&format!("{}_bg.wasm", crate_name))).unwrap();
    let mut wasm = Vec::new();
    file.read_to_end(&mut wasm).unwrap();

    let mut file = File::open(path.join(&format!("{}.js", crate_name))).unwrap();
    let mut js = String::new();
    file.read_to_string(&mut js).unwrap();

    let find_object_bindgen_hash = FIND_OBJECT_BINDGEN_RE
        .captures(&js)
        .and_then(|c| c.get(1))
        .and_then(|m| Some(m.as_str()))
        .or(Some(""))
        .unwrap();

    let find_object_iter_bindgen_hash = FIND_OBJECT_ITER_BINDGEN_RE
        .captures(&js)
        .and_then(|c| c.get(1))
        .and_then(|m| Some(m.as_str()))
        .or(Some(""))
        .unwrap();

    let find_object_iter_next_bindgen_hash = FIND_OBJECT_ITER_NEXT_BINDGEN_RE
        .captures(&js)
        .and_then(|c| c.get(1))
        .and_then(|m| Some(m.as_str()))
        .or(Some(""))
        .unwrap();

    let index_iter_bindgen_hash = INDEX_ITER_BINDGEN_RE
        .captures(&js)
        .and_then(|c| c.get(1))
        .and_then(|m| Some(m.as_str()))
        .or(Some(""))
        .unwrap();

    let index_iter_next_value_bindgen_hash = INDEX_ITER_NEXT_VALUE_BINDGEN_RE
        .captures(&js)
        .and_then(|c| c.get(1))
        .and_then(|m| Some(m.as_str()))
        .or(Some(""))
        .unwrap();

    let index_iter_next_id_bindgen_hash = INDEX_ITER_NEXT_ID_BINDGEN_RE
        .captures(&js)
        .and_then(|c| c.get(1))
        .and_then(|m| Some(m.as_str()))
        .or(Some(""))
        .unwrap();

    let log_bindgen_hash = LOG_BINDGEN_RE
        .captures(&js)
        .and_then(|c| c.get(1))
        .and_then(|m| Some(m.as_str()))
        .or(Some(""))
        .unwrap();

    let metadata = ModuleMetadata {
        name: format!("./{}", crate_name),
        bindgen_import_hashes: Some(BindgenImportHashes {
            log: log_bindgen_hash.to_string(),
            find_object: find_object_bindgen_hash.to_string(),
            find_objects_iter: find_object_iter_bindgen_hash.to_string(),
            find_objects_iter_next: find_object_iter_next_bindgen_hash.to_string(),
            index_iter: index_iter_bindgen_hash.to_string(),
            index_iter_next_value: index_iter_next_value_bindgen_hash.to_string(),
            index_iter_next_id: index_iter_next_id_bindgen_hash.to_string(),
        }),
    };

    //    let descriptor_set = tmp.path().join("descriptor-set");
    let descriptor_set = std::path::Path::new("/tmp/descriptor-set");

    let mut cmd = Command::new(prost_build::protoc());
    cmd.arg("--include_imports")
        .arg("--include_source_info")
        .arg("-o")
        .arg(&descriptor_set);

    for include in result_includes {
        cmd.arg("-I").arg(include);
    }

    cmd.arg(result_schema_file.clone());

    let output = cmd.output().unwrap();
    if !output.status.success() {
        panic!("protoc failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    let mut buf = Vec::new();
    fs::File::open(descriptor_set)
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    let descriptor_set = FileDescriptorSet::decode(&buf).unwrap();
    let file_descriptor = descriptor_set
        .file
        .iter()
        .find(|&descriptor| descriptor.name() == result_schema_file.as_os_str().to_str().unwrap())
        .unwrap();

    let descriptor = file_descriptor
        .message_type
        .iter()
        .find(|&message| message.name() == result_schema_message)
        .unwrap()
        .clone();

    CLIENT
        .with(|c| {
            c.borrow_mut()
                .register_wasm_module(database, name, metadata, wasm, descriptor)
        })
        .and_then(|response| {
            use protodb_client::generated::protodb::wasm::register_module_response::ErrorCode;

            match response.error_code() {
                ErrorCode::NoError => (),
                ErrorCode::InternalError => {
                    println!("error registering wasm module: internal error")
                }
                ErrorCode::InvalidDatabase => println!("invalid database"),
                ErrorCode::ModuleExists => println!("module already exists"),
            }
            Ok(())
        })
        .map_err(|err| println!("error registering wasm module: {:?}", err))
        .unwrap();
}

fn run_wasm_module(database: String, name: String) {
    CLIENT
        .with(|c| {
            c.borrow_mut()
                .run_wasm_module(database.clone(), name.clone())
        })
        .and_then(|response| {
            use protodb_client::generated::protodb::wasm::run_module_response::ErrorCode;
            match response.error_code() {
                ErrorCode::NoError => Ok(response.result),
                ErrorCode::InternalError => panic!("error running wasm module: internal error"),
                ErrorCode::InvalidDatabase => panic!("invalid database"),
                ErrorCode::InvalidModule => panic!("invalid module"),
            }
        })
        .and_then(|result| {
            CLIENT
                .with(|c| c.borrow_mut().get_wasm_module_info(database, name))
                .and_then(|info| {
                    let descriptor = info.result_schema.unwrap();
                    let (descriptor_fields, _) = descriptor_fields(&descriptor).unwrap();
                    let object = decoded_object(&descriptor_fields, &result).unwrap();
                    println!("response: ");

                    for (tag, value) in object.fields_iter() {
                        let (name, _, _) = descriptor_fields.info(tag).unwrap();
                        println!("  {}: {}", name, value);
                    }

                    Ok(())
                })
        })
        .map_err(|err| println!("error running wasm module: {:?}", err))
        .unwrap();
}

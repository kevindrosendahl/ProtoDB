use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

use crate::{
    transport::grpc::generated::protodb::wasm::register_module_request::{
        module_metadata::BindgenImportHashes, ModuleMetadata,
    },
    CLIENT,
};

use regex::Regex;

lazy_static! {
    static ref FIND_OBJECT_BINDGEN_RE: Regex =
        Regex::new("__wbg_findobject_([a-f0-9]{16})").unwrap();
    static ref FIND_OBJECT_ITER_BINDGEN_RE: Regex =
        Regex::new("__wbg_findobjectsiter_([a-f0-9]{16})").unwrap();
    static ref FIND_OBJECT_ITER_NEXT_BINDGEN_RE: Regex =
        Regex::new("__wbg_findobjectsiternext_([a-f0-9]{16})").unwrap();
    static ref LOG_BINDGEN_RE: Regex = Regex::new("__wbg_log_([a-f0-9]{16})").unwrap();
}

#[derive(Debug, StructOpt)]
pub enum Wasm {
    #[structopt(name = "register")]
    Register {
        database: String,

        name: String,

        #[structopt(name = "crate", parse(from_os_str))]
        crate_: PathBuf,

        #[structopt(long = "package", short = "p")]
        package: Option<String>,
    },

    #[structopt(name = "run")]
    Run { database: String, name: String },
}

pub fn run_wasm(wasm: Wasm) {
    match wasm {
        Wasm::Register {
            database,
            name,
            crate_,
            package,
        } => register_module(database, name, crate_, package),
        Wasm::Run { database, name } => run_wasm_module(database, name),
    }
}

fn register_module(database: String, name: String, crate_: PathBuf, package: Option<String>) {
    env::set_current_dir(crate_.clone()).unwrap();

    let tmp = tempdir::TempDir::new("protoctl-register-wasm-module").unwrap();
    let path = tmp.path();

    let mut cmd = Command::new(which::which("cargo").unwrap());
    cmd.arg("build").arg("--target=wasm32-unknown-unknown").arg("--release");

    let crate_name = match package {
        Some(package) => {
            cmd.arg(&format!("--package={}", package));
            package
        }
        None => String::from(
            crate_
                .as_path()
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap(),
        ),
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

    let log_bindgen_hash = LOG_BINDGEN_RE
        .captures(&js)
        .and_then(|c| c.get(1))
        .and_then(|m| Some(m.as_str()))
        .or(Some(""))
        .unwrap();

    let metadata = ModuleMetadata {
        name: format!("./{}", crate_name),
        bindgen_import_hashes: Some(BindgenImportHashes {
            find_object: find_object_bindgen_hash.to_string(),
            find_objects_iter: find_object_iter_bindgen_hash.to_string(),
            find_objects_iter_next: find_object_iter_next_bindgen_hash.to_string(),
            log: log_bindgen_hash.to_string(),
        }),
    };

    CLIENT
        .with(|c| c.borrow_mut().register_wasm_module(database, name, metadata, wasm))
        .and_then(|response| {
            use crate::transport::grpc::generated::protodb::wasm::register_module_response::ErrorCode;

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
        .with(|c| c.borrow_mut().run_wasm_module(database, name))
        .and_then(|response| {
            use crate::transport::grpc::generated::protodb::wasm::run_module_response::ErrorCode;
            match response.error_code() {
                ErrorCode::NoError => {
                    println!("result: {:?}", response.result)
                }
                ErrorCode::InternalError => println!("error running wasm module: internal error"),
                ErrorCode::InvalidDatabase => println!("invalid database"),
                ErrorCode::InvalidModule => println!("invalid module"),
            }
            Ok(())
        })
        .map_err(|err| println!("error running wasm module: {:?}", err))
        .unwrap();
}

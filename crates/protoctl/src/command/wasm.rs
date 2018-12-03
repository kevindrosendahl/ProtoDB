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
    },
}

pub fn run_wasm(wasm: Wasm) {
    match wasm {
        Wasm::Register {
            database,
            name,
            crate_,
        } => register_module(database, name, crate_),
    }
}

fn register_module(database: String, name: String, crate_: PathBuf) {
    env::set_current_dir(crate_).unwrap();

    let mut cmd = Command::new(which::which("cargo").unwrap());
    assert_eq!(
        cmd.arg("build")
            .arg("--target=wasm32-unknown-unknown")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success(),
        true,
    );

    let _tmp = tempdir::TempDir::new("protoctl-register-wasm-module").unwrap();
    let mut cmd = Command::new(which::which("wasm-bindgen").unwrap());

    // TODO: get this from Cargo.toml
    let crate_name = "wasm";
    let path = std::path::Path::new("/tmp/protodb/wasm");
    assert_eq!(
        cmd.arg(&format!(
            "target/wasm32-unknown-unknown/debug/{}.wasm",
            crate_name
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
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    let log_bindgen_hash = LOG_BINDGEN_RE
        .captures(&js)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    let metadata = ModuleMetadata {
        name: format!("./{}", crate_name),
        bindgen_import_hashes: Some(BindgenImportHashes {
            find_object: find_object_bindgen_hash.to_string(),
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

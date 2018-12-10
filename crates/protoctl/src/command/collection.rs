use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

use crate::CLIENT;

use prost::Message;
use prost_types::FileDescriptorSet;

#[derive(Debug, StructOpt)]
pub enum Collection {
    #[structopt(name = "create")]
    Create {
        database: String,

        name: String,

        #[structopt(parse(from_os_str))]
        schema_file: PathBuf,

        schema_message: String,

        #[structopt(long = "include", short = "i", parse(from_os_str))]
        includes: Vec<PathBuf>,
    },

    #[structopt(name = "info")]
    Info {
        database: String,

        collection: String,
    },

    #[structopt(name = "list")]
    List { database: String },
}

pub fn run_collection(collection: Collection) {
    match collection {
        Collection::Create {
            database,
            name,
            schema_file,
            schema_message,
            includes,
        } => create_collection(database, name, &schema_file, &schema_message, &includes),
        Collection::Info {
            database,
            collection,
        } => get_collection_info(database, collection),
        Collection::List { database } => list_collections(database),
    }
}

fn create_collection(
    database: String,
    name: String,
    schema_file: &PathBuf,
    schema_message: &str,
    includes: &[PathBuf],
) {
    let tmp = tempdir::TempDir::new("protoctl-create-collection").unwrap();
    let descriptor_set = tmp.path().join("descriptor-set");

    let mut cmd = Command::new(prost_build::protoc());
    cmd.arg("--include_imports")
        .arg("--include_source_info")
        .arg("-o")
        .arg(&descriptor_set);

    for include in includes {
        cmd.arg("-I").arg(include);
    }

    cmd.arg(schema_file.clone());

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
        .find(|&descriptor| descriptor.name() == schema_file.file_name().unwrap().to_str().unwrap())
        .unwrap();

    let descriptor = file_descriptor
        .message_type
        .iter()
        .find(|&message| message.name() == schema_message)
        .unwrap()
        .clone();

    CLIENT
        .with(|c| c.borrow_mut().create_collection(database, name, descriptor))
        .and_then(|response| {
            use protodb_client::generated::protodb::collection::create_collection_response::ErrorCode;
            use protodb_client::generated::protodb::collection::create_collection_response::schema_error::SchemaErrorCode;

            match response.error_code() {
                ErrorCode::NoError => println!("successfully created collection"),
                ErrorCode::InternalError => println!("error creating collection: internal error"),
                ErrorCode::InvalidDatabase => println!("invalid database"),
                ErrorCode::CollectionExists => println!("collection already exists"),
                ErrorCode::SchemaError => {
                    let schema_error = response.schema_error.unwrap();
                    match schema_error.code() {
                        SchemaErrorCode::NoSchemaError => panic!("got schema error but no schema error set"),
                        SchemaErrorCode::MissingIdField => println!("error creating collection: schema missing id field"),
                        SchemaErrorCode::InvalidIdType => println!("error creating collection: schema had invalid id field type"),
                        SchemaErrorCode::InvalidFieldType => println!("error creating collection: schema had invalid field: {}", schema_error.message),
                        SchemaErrorCode::EncodingError => println!("error creating collection: encoding error: {}", schema_error.message),
                    }
                }
            }
            Ok(())
        })
        .map_err(|err| println!("error creating collection: {:?}", err))
        .unwrap();
}

fn list_collections(database: String) {
    CLIENT
        .with(|c| c.borrow_mut().list_collections(database))
        .and_then(|response| {
            println!("collections: {:?}", response.collections);
            Ok(())
        })
        .map_err(|err| println!("error listing databases: {:?}", err))
        .unwrap();
}

#[allow(clippy::needless_pass_by_value)]
fn get_collection_info(database: String, collection: String) {
    CLIENT
        .with(|c| c.borrow_mut().get_collection_info(database.clone(), collection.clone()))
        .and_then(|response| {
            use protodb_client::generated::protodb::collection::get_collection_info_response::ErrorCode;

            match response.error_code() {
                ErrorCode::NoError => {
                    println!("database: {}", database);
                    println!("collection: {}", collection);

                    let schema = response.schema.unwrap();
                    println!("schema:");
                    println!("  name: {}", schema.name());
                    println!("  fields:");

                    for field in schema.field.iter() {
                        println!("    {} ({}): {:?}", field.name(), field.number(), field.type_());
                    }
                },
                ErrorCode::InternalError => println!("error getting collection info: internal error"),
                ErrorCode::InvalidDatabase => println!("invalid database"),
                ErrorCode::InvalidCollection => println!("invalid collection"),
            };

            Ok(())
        })
        .map_err(|err| println!("error getting collection info: {:?}", err))
        .unwrap();
}

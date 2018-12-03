use crate::CLIENT;

use protodb_schema::Schema;

#[derive(Debug, StructOpt)]
pub enum Object {
    #[structopt(name = "find")]
    Find {
        database: String,

        collection: String,

        id: u64,
    },
}

pub fn run_object(object: Object) {
    match object {
        Object::Find {
            database,
            collection,
            id,
        } => find_object(database, collection, id),
    }
}

fn find_object(database: String, collection: String, id: u64) {
    CLIENT
        .with(|c| c.borrow_mut().get_collection_info(database.clone(), collection.clone()))
        .map_err(|err| format!("{:?}", err))
        .and_then(|response| {
            use crate::transport::grpc::generated::protodb::collection::get_collection_info_response::ErrorCode;

            match response.error_code() {
                ErrorCode::NoError => {
                    Ok(response.schema.unwrap())
                },
                ErrorCode::InternalError => Err("error getting collection info: internal error".to_string()),
                ErrorCode::InvalidDatabase => Err("invalid database".to_string()),
                ErrorCode::InvalidCollection => Err("invalid collection".to_string()),
            }
        })
        .and_then(|schema| {
            CLIENT.with(|c| c.borrow_mut().find_object(database, collection, id))
                .map_err(|err| format!("{:?}", err))
                .and_then(|response| {
                    use crate::transport::grpc::generated::protodb::object::find_object_response::ErrorCode;

                    match response.error_code() {
                        ErrorCode::NoError => {
                            Ok((schema, response.object))
                        },
                        ErrorCode::InternalError => Err("error getting collection info: internal error".to_string()),
                        ErrorCode::InvalidDatabase => Err("invalid database".to_string()),
                        ErrorCode::InvalidCollection => Err("invalid collection".to_string()),
                        ErrorCode::InvalidId => Err("object not found".to_string()),
                    }
                })
        })
        .and_then(|(schema, object_bytes)| {
            let schema = Schema::new(&schema).unwrap();
            let object = schema.decoded_object(&object_bytes).unwrap();
            println!("object {}:", object.id);

            for (tag, value) in object.fields_iter() {
                let (name, _, _) = schema.fields.get(tag).unwrap();
                println!("  {}: {}", name, value);
            }

            Ok(())
        })
        .map_err(|err| println!("error finding object: {}", err))
        .unwrap();
}

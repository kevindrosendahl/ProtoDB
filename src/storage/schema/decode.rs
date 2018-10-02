use std::io::Cursor;

use super::{errors::ObjectError, Schema};

use prost::encoding;

impl Schema {
    pub fn decode_object(&self, object: &[u8]) -> Result<(), ObjectError> {
        let mut buf = Cursor::new(object);
        let (key, wire_type) = encoding::decode_key(&mut buf)?;
        println!("{} {:?}", key, wire_type);
        Ok(())
    }
}

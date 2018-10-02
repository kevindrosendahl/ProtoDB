use std::io::Cursor;

use super::{errors::ObjectError, Schema};

use prost::encoding;

impl Schema {
    pub fn decode_object(data: &[u8]) -> Result<(), ObjectError> {
        let mut buf = Cursor::new(data);
        let (key, wire_type) = encoding::decode_key(&mut buf)?;
        println!("{} {:?}", key, wire_type);
        Ok(())
    }
}

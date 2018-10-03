use std::io::Cursor;

use super::{errors::ObjectError, Schema};

use bytes::{Buf, BufMut};
use prost::encoding;
use prost_types::DescriptorProto;

impl Schema {
    //    pub fn decode_object<B, C>(&self, object: &[u8]) -> DecodeObject<B, C> where B: Buf, C: Buf{
    //        let mut buf = Cursor::new(object);
    //        DecodeObject{
    //            object_buf: &buf,
    //        }
    //        let (key, wire_type) = encoding::decode_key(&mut buf)?;
    //        println!("{} {:?}", key, wire_type);
    //        let val = match wire_type {
    //            encoding::WireType::Varint => encoding::decode_varint(&mut buf),
    //            _ => Ok(0),
    //        }?;
    //                encoding::merge_loop()
    //        println!("{}", val);
    //        Ok(())
    //    }
    pub fn decode_object(&self, object: &[u8]) -> Result<(), ObjectError> {
        //        let mut buf = Cursor::new(object);
        //        let (key, wire_type) = encoding::decode_key(&mut buf)?;
        //        println!("{} {:?}", key, wire_type);
        //        let val = match wire_type {
        //            encoding::WireType::Varint => encoding::decode_varint(&mut buf),
        //            _ => Ok(0),
        //        }?;
        //        encoding::merge_loop()
        //        println!("{}", val);
        Ok(())
    }
}

//pub struct DecodeObject<'a, B: Buf, C: BufMut> {
//    object_buf: &'a mut B,
//    write_buf: C,
//    descriptor: &'a DescriptorProto,
//}
//
//impl<'a, B: Buf> Iterator for DecodeObject<'a, B> {
//    type Item = Result<(encoding::WireType, &'a [u8]), ObjectError>;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        let (key, wire_type) = match encoding::decode_key(self.object_buf) {
//            Ok((key, wire_type)) => (key, wire_type),
//            Err(err) => return Some(Err(ObjectError::DecodeError(err))),
//        };
//
//        println!("{} {:?}", key, wire_type);
//        let result = match wire_type {
//            encoding::WireType::Varint => encoding::decode_varint(self.object_buf),
//            _ => Ok(0),
//        };
//
//        encoding::uint64::
//
//        let val = match result {
//            Ok(val) => val,
//            Err(err) => return Some(Err(ObjectError::DecodeError(err))),
//        };
//
//        println!("{}", val);
//        Some(byteorder::LittleEndian)
//    }
//}

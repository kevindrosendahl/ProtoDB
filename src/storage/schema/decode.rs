use std::io::Cursor;

use super::{errors::ObjectError, Schema};

use byteorder::{LittleEndian, ReadBytesExt};
use bytes::{Buf, BufMut};
use prost::encoding;
use prost_types::{field_descriptor_proto::Type, DescriptorProto};

impl Schema {
    pub fn decode_object<'a>(&'a self, object: &'a [u8]) -> DecodeObject<'a> {
        DecodeObject::new(object, self)
    }
}

pub struct DecodeObject<'a> {
    object_buf: Cursor<&'a [u8]>,
    object_bytes: &'a [u8],
    len: usize,
    schema: &'a Schema,
}

pub struct FieldInfo<'a> {
    pub tag: i32,
    pub wire_type: encoding::WireType,
    pub value: FieldValue<'a>,
}

#[derive(Debug)]
pub enum FieldValue<'a> {
    Double(f64),
    Float(f32),
    Int32(i32),
    Int64(i64),
    Uint32(u32),
    Uint64(u64),
    Sint32(i32),
    Sint64(i64),
    Fixed64(u64),
    Fixed32(u32),
    Sfixed32(i32),
    Sfixed64(i64),
    Bool(bool),
    String(&'a [u8]),
    Bytes(&'a [u8]),
    Enum(u64),
}

impl<'a> DecodeObject<'a> {
    fn new(object: &'a [u8], schema: &'a Schema) -> DecodeObject<'a> {
        DecodeObject {
            object_buf: Cursor::new(object),
            object_bytes: &object,
            len: object.len(),
            schema,
        }
    }
}

macro_rules! wire_type_mismatch_err {
    (  $tag:ident, $type:ident, $wire_type:ident ) => {
        Some(Err(ObjectError::SchemaDecodeError(format!(
            "error decoding field {} (type {:?}): unexpected wire_type {:?}",
            $tag, $type, $wire_type
        ))))
    };
}

macro_rules! decode_err {
    (  $tag:ident, $type:ident, $err:ident ) => {
        Some(Err(ObjectError::SchemaDecodeError(format!(
            "error decoding field {} (type {:?}): {:?}",
            $tag, $type, $err
        ))))
    };
}

impl<'a> Iterator for DecodeObject<'a> {
    type Item = Result<FieldInfo<'a>, ObjectError>;

    fn next(&mut self) -> Option<Self::Item> {
        // decode fields until we are done, hit an error, or find a field
        // that's defined in our schema
        loop {
            // if the buffer's empty, we're done decoding
            if self.object_buf.remaining() == 0 {
                return None;
            }

            // grab the next key and wiretype from the buffer
            let (key, wire_type) = match encoding::decode_key(&mut self.object_buf) {
                Ok((key, wire_type)) => (key, wire_type),
                Err(err) => return Some(Err(ObjectError::ProstDecodeError(err))),
            };
            let tag = key as i32;

            // check to see if this field is part of the schema
            let type_ = match self.schema.fields.get(&tag) {
                Some((_, type_)) => Some(type_),
                None => None,
            };

            let offset = self.len - self.object_buf.remaining();
            let value = match wire_type {
                // If the field's value is a varint, use the encoding library to decode it.
                // This will advance the buffer.
                encoding::WireType::Varint => {
                    match encoding::decode_varint(&mut self.object_buf) {
                        Ok(val) => {
                            // if the field was not part of the schema, go on to the next field
                            if type_.is_none() {
                                continue;
                            }

                            let type_ = type_.unwrap();
                            match type_ {
                                Type::Int32 => FieldValue::Int32(val as i32),
                                Type::Int64 => FieldValue::Int64(val as i64),
                                Type::Uint32 => FieldValue::Uint32(val as u32),
                                Type::Uint64 => FieldValue::Uint64(val),
                                Type::Sint32 => FieldValue::Sint32(val as i32),
                                Type::Sint64 => FieldValue::Sint64(val as i64),
                                Type::Bool => FieldValue::Bool(val != 0),
                                Type::Enum => FieldValue::Enum(val),
                                _ => return wire_type_mismatch_err!(tag, type_, wire_type),
                            }
                        }
                        Err(err) => return Some(Err(ObjectError::ProstDecodeError(err))),
                    }
                }
                // If the field's value is a fixed size or length delimited, advance
                // the buffer the appropriate amount so that on the next iteration it will
                // be properly set, and return the proper value
                encoding::WireType::ThirtyTwoBit => {
                    self.object_buf.advance(4);

                    // if the field was not part of the schema, go on to the next field
                    if type_.is_none() {
                        continue;
                    }

                    let mut reader = Cursor::new(&self.object_bytes[offset..offset + 4]);
                    let type_ = type_.unwrap();
                    match type_ {
                        Type::Float => {
                            match reader.read_f32::<LittleEndian>() {
                                Ok(val) => FieldValue::Float(val),
                                Err(err) => return decode_err!(tag, type_, err),
                            }
                        },
                        Type::Fixed32 => {
                            match reader.read_u32::<LittleEndian>() {
                                Ok(val) => FieldValue::Fixed32(val),
                                Err(err) => return decode_err!(tag, type_, err),
                            }
                        },
                        Type::Sfixed32 => match reader.read_i32::<LittleEndian>() {
                            Ok(val) => FieldValue::Sfixed32(val),
                            Err(err) => return decode_err!(tag, type_, err),
                        }
                        _ => return wire_type_mismatch_err!(tag, type_, wire_type),
                    }
                }
                encoding::WireType::SixtyFourBit => {
                    self.object_buf.advance(8);

                    // if the field was not part of the schema, go on to the next field
                    if type_.is_none() {
                        continue;
                    }

                    let mut reader = Cursor::new(&self.object_bytes[offset..offset + 8]);
                    let type_ = type_.unwrap();
                    match type_ {
                        Type::Double => {
                            match reader.read_f32::<LittleEndian>() {
                                Ok(val) => FieldValue::Float(val),
                                Err(err) => return decode_err!(tag, type_, err),
                            }
                        },
                        Type::Fixed64 => {
                            match reader.read_u64::<LittleEndian>() {
                                Ok(val) => FieldValue::Fixed64(val),
                                Err(err) => return decode_err!(tag, type_, err),
                            }
                        },
                        Type::Sfixed64 => match reader.read_i64::<LittleEndian>() {
                            Ok(val) => FieldValue::Sfixed64(val),
                            Err(err) => return decode_err!(tag, type_, err),
                        }
                        _ => return wire_type_mismatch_err!(tag, type_, wire_type),
                    }
                }
                encoding::WireType::LengthDelimited => {
                    match encoding::decode_varint(&mut self.object_buf) {
                        Ok(len) => {
                            self.object_buf.advance(len as usize);

                            // if the field was not part of the schema, go on to the next field
                            if type_.is_none() {
                                continue;
                            }

                            let type_ = type_.unwrap();
                            let bytes = &self.object_bytes[offset..offset + len as usize];
                            match type_ {
                                Type::String => FieldValue::String(bytes),
                                Type::Bytes => FieldValue::Bytes(bytes),
                                _ => return wire_type_mismatch_err!(tag, type_, wire_type),
                            }
                        }
                        Err(err) => return Some(Err(ObjectError::ProstDecodeError(err))),
                    }
                }
            };

            return Some(Ok(FieldInfo {
                tag,
                wire_type,
                value,
            }));
        }
    }
}

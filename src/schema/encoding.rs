use std::io::Cursor;

use super::{
    errors::{ObjectError, SchemaError},
    Schema,
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use bytes::{Buf, BufMut};
use prost::{encoding, encoding::WireType};
use prost_types::field_descriptor_proto::Type;

macro_rules! varint_encoder {
    ( $val:ident ) => {{
        let mut writer = vec![];
        encoding::encode_varint($val as u64, &mut writer);
        writer
    }};
}

macro_rules! length_delimited_encoder {
    ( $val:ident ) => {{
        let mut writer = vec![];
        encoding::encode_varint($val.len() as u64, &mut writer);
        writer.put_slice($val);
        writer
    }};
}

macro_rules! little_endian_encoder {
    (  $val:ident, $encoder:ident, $size:expr ) => {{
        let mut writer = Vec::with_capacity($size);
        // FIXME: handle this error
        writer.$encoder::<LittleEndian>($val).unwrap();
        writer
    }};
}

macro_rules! varint_decoder {
    (  $buf:ident, $ty:path, $val_ty:path ) => {{
        let mut reader = Cursor::new($buf);
        // FIXME: handle this error
        let val = encoding::decode_varint(&mut reader).unwrap();
        $val_ty(val as $ty)
    }};
}

macro_rules! length_delimited_decoder {
    ( $buf:ident, $ty:path ) => {{
        let mut reader = Cursor::new($buf);
        // FIXME: handle this error
        let len = encoding::decode_varint(&mut reader).unwrap() as usize;
        // FIXME: check for {over/under}flow
        let position = reader.position() as usize;
        $ty(&$buf[position..position + len])
    }};
}

macro_rules! little_endian_decoder {
    (  $buf:ident, $decoder:ident, $ty:path ) => {{
        let mut reader = Cursor::new($buf);
        // FIXME: handle this error
        let val = reader.$decoder::<LittleEndian>().unwrap();
        $ty(val)
    }};
}

impl Schema {
    pub fn wire_type(&self, tag: i32) -> Option<WireType> {
        self.fields.get(&tag).map(|(_, type_)| {
            match type_ {
                Type::Int32
                | Type::Int64
                | Type::Uint32
                | Type::Uint64
                | Type::Sint32
                | Type::Sint64
                | Type::Bool
                | Type::Enum => WireType::Varint,
                Type::Fixed32 | Type::Sfixed32 | Type::Float => WireType::ThirtyTwoBit,
                Type::Fixed64 | Type::Sfixed64 | Type::Double => WireType::SixtyFourBit,
                Type::String | Type::Bytes => WireType::LengthDelimited,
                // FIXME: handle more gracefully
                _ => panic!("unexpected type"),
            }
        })
    }

    pub fn decode_object<'a>(&'a self, object: &'a [u8]) -> DecodeObject<'a> {
        DecodeObject {
            object_buf: Cursor::new(object),
            object_bytes: &object,
            schema: self,
        }
    }

    pub fn encode_field(tag: i32, wire_type: WireType, value: &[u8], buf: &mut impl BufMut) {
        encoding::encode_key(tag as u32, wire_type, buf);
        buf.put(value);
    }

    pub fn encode_value(value: &FieldValue) -> Vec<u8> {
        // TODO: can probably macro this out a bit more
        match *value {
            FieldValue::Int32(val) => varint_encoder!(val),
            FieldValue::Int64(val) => varint_encoder!(val),
            FieldValue::Uint32(val) => varint_encoder!(val),
            FieldValue::Uint64(val) => varint_encoder!(val),
            FieldValue::Sint32(val) => varint_encoder!(val),
            FieldValue::Sint64(val) => varint_encoder!(val),
            FieldValue::Bool(val) => {
                let val = val as u32;
                varint_encoder!(val)
            }
            FieldValue::Enum(val) => varint_encoder!(val),

            FieldValue::Float(val) => little_endian_encoder!(val, write_f32, 4),
            FieldValue::Double(val) => little_endian_encoder!(val, write_f64, 8),
            FieldValue::Fixed32(val) => little_endian_encoder!(val, write_u32, 4),
            FieldValue::Fixed64(val) => little_endian_encoder!(val, write_u64, 8),
            FieldValue::Sfixed32(val) => little_endian_encoder!(val, write_i32, 4),
            FieldValue::Sfixed64(val) => little_endian_encoder!(val, write_i64, 8),

            FieldValue::String(val) => length_delimited_encoder!(val),
            FieldValue::Bytes(val) => length_delimited_encoder!(val),
        }
    }

    #[allow(dead_code)]
    fn decode_value(type_: Type, value: &[u8]) -> FieldValue {
        // TODO: can probably macro this out a bit more
        match type_ {
            Type::Int32 => varint_decoder!(value, i32, FieldValue::Int32),
            Type::Int64 => varint_decoder!(value, i64, FieldValue::Int64),
            Type::Uint32 => varint_decoder!(value, u32, FieldValue::Uint32),
            Type::Uint64 => varint_decoder!(value, u64, FieldValue::Uint64),
            Type::Sint32 => varint_decoder!(value, i32, FieldValue::Sint32),
            Type::Sint64 => varint_decoder!(value, i64, FieldValue::Sint64),
            Type::Bool => {
                let val = match varint_decoder!(value, u64, FieldValue::Uint64) {
                    FieldValue::Uint64(val) => val,
                    _ => panic!("value_decoder! did not return Uint64 for bool"),
                };
                let val = val != 0;
                FieldValue::Bool(val)
            }
            Type::Enum => varint_decoder!(value, u64, FieldValue::Enum),

            Type::Float => little_endian_decoder!(value, read_f32, FieldValue::Float),
            Type::Double => little_endian_decoder!(value, read_f64, FieldValue::Double),
            Type::Fixed32 => little_endian_decoder!(value, read_u32, FieldValue::Fixed32),
            Type::Fixed64 => little_endian_decoder!(value, read_u64, FieldValue::Fixed64),
            Type::Sfixed32 => little_endian_decoder!(value, read_i32, FieldValue::Sfixed32),
            Type::Sfixed64 => little_endian_decoder!(value, read_i64, FieldValue::Sfixed64),

            Type::String => length_delimited_decoder!(value, FieldValue::String),
            Type::Bytes => length_delimited_decoder!(value, FieldValue::Bytes),

            _ => panic!("unexpected type {:?}", type_),
        }
    }
}

pub struct DecodeObject<'a> {
    object_buf: Cursor<&'a [u8]>,
    object_bytes: &'a [u8],
    schema: &'a Schema,
}

#[derive(Debug)]
pub struct FieldInfo<'a> {
    pub tag: i32,
    pub wire_type: encoding::WireType,
    pub value: FieldValue<'a>,
}

#[derive(Debug)]
pub enum FieldValue<'a> {
    Float(f32),
    Double(f64),
    Int32(i32),
    Int64(i64),
    Uint32(u32),
    Uint64(u64),
    Sint32(i32),
    Sint64(i64),
    Fixed32(u32),
    Fixed64(u64),
    Sfixed32(i32),
    Sfixed64(i64),
    Bool(bool),
    String(&'a [u8]),
    Bytes(&'a [u8]),
    Enum(u64),
}

macro_rules! iter_err {
    ( $inner:expr ) => {
        Some(Err($inner))
    };
}

macro_rules! wire_type_mismatch_err {
    (  $tag:ident, $type:ident, $wire_type:ident ) => {
        iter_err!(ObjectError::SchemaError(SchemaError::EncodingError(
            format!(
                "error decoding field {} (type {:?}): unexpected wire_type {:?}",
                $tag, $type, $wire_type
            )
        )))
    };
}

macro_rules! decode_err {
    (  $tag:ident, $type:ident, $err:ident ) => {
        iter_err!(ObjectError::SchemaError(SchemaError::EncodingError(
            format!(
                "error decoding field {} (type {:?}): {:?}",
                $tag, $type, $err
            )
        )))
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

            let offset = self.object_bytes.len() - self.object_buf.remaining();
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
                        Err(err) => return iter_err!(ObjectError::ProstDecodeError(err)),
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
                        Type::Float => match reader.read_f32::<LittleEndian>() {
                            Ok(val) => FieldValue::Float(val),
                            Err(err) => return decode_err!(tag, type_, err),
                        },
                        Type::Fixed32 => match reader.read_u32::<LittleEndian>() {
                            Ok(val) => FieldValue::Fixed32(val),
                            Err(err) => return decode_err!(tag, type_, err),
                        },
                        Type::Sfixed32 => match reader.read_i32::<LittleEndian>() {
                            Ok(val) => FieldValue::Sfixed32(val),
                            Err(err) => return decode_err!(tag, type_, err),
                        },
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
                        Type::Double => match reader.read_f64::<LittleEndian>() {
                            Ok(val) => FieldValue::Double(val),
                            Err(err) => return decode_err!(tag, type_, err),
                        },
                        Type::Fixed64 => match reader.read_u64::<LittleEndian>() {
                            Ok(val) => FieldValue::Fixed64(val),
                            Err(err) => return decode_err!(tag, type_, err),
                        },
                        Type::Sfixed64 => match reader.read_i64::<LittleEndian>() {
                            Ok(val) => FieldValue::Sfixed64(val),
                            Err(err) => return decode_err!(tag, type_, err),
                        },
                        _ => return wire_type_mismatch_err!(tag, type_, wire_type),
                    }
                }
                encoding::WireType::LengthDelimited => {
                    match encoding::decode_varint(&mut self.object_buf) {
                        Ok(len) => {
                            // we just read a varint off the buffer, so increase the offset
                            // by however many bytes were read
                            let offset = offset + encoding::encoded_len_varint(len);

                            // advanced the buffer as many bytes as we're about to return
                            // from the underlying byte array to keep the bookkeeping correct
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
                        Err(err) => return iter_err!(ObjectError::ProstDecodeError(err)),
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

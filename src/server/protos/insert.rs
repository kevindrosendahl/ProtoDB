// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Insert {
    // message fields
    database: ::protobuf::SingularField<::std::string::String>,
    collection: ::protobuf::SingularField<::std::string::String>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Insert {}

impl Insert {
    pub fn new() -> Insert {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Insert {
        static mut instance: ::protobuf::lazy::Lazy<Insert> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Insert,
        };
        unsafe {
            instance.get(|| {
                Insert {
                    database: ::protobuf::SingularField::none(),
                    collection: ::protobuf::SingularField::none(),
                    data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string database = 1;

    pub fn clear_database(&mut self) {
        self.database.clear();
    }

    pub fn has_database(&self) -> bool {
        self.database.is_some()
    }

    // Param is passed by value, moved
    pub fn set_database(&mut self, v: ::std::string::String) {
        self.database = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_database(&mut self) -> &mut ::std::string::String {
        if self.database.is_none() {
            self.database.set_default();
        };
        self.database.as_mut().unwrap()
    }

    // Take field
    pub fn take_database(&mut self) -> ::std::string::String {
        self.database.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_database(&self) -> &str {
        match self.database.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string collection = 2;

    pub fn clear_collection(&mut self) {
        self.collection.clear();
    }

    pub fn has_collection(&self) -> bool {
        self.collection.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collection(&mut self, v: ::std::string::String) {
        self.collection = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_collection(&mut self) -> &mut ::std::string::String {
        if self.collection.is_none() {
            self.collection.set_default();
        };
        self.collection.as_mut().unwrap()
    }

    // Take field
    pub fn take_collection(&mut self) -> ::std::string::String {
        self.collection.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_collection(&self) -> &str {
        match self.collection.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bytes data = 3;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for Insert {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.database));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.collection));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.database {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.collection {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.data {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.database.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.collection.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.data.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Insert>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Insert {
    fn new() -> Insert {
        Insert::new()
    }

    fn descriptor_static(_: ::std::option::Option<Insert>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "database",
                    Insert::has_database,
                    Insert::get_database,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "collection",
                    Insert::has_collection,
                    Insert::get_collection,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "data",
                    Insert::has_data,
                    Insert::get_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Insert>(
                    "Insert",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Insert {
    fn clear(&mut self) {
        self.clear_database();
        self.clear_collection();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Insert {
    fn eq(&self, other: &Insert) -> bool {
        self.database == other.database &&
        self.collection == other.collection &&
        self.data == other.data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Insert {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InsertResponse {
    // message fields
    success: ::std::option::Option<bool>,
    _id: ::std::option::Option<u64>,
    failure_code: ::std::option::Option<InsertResponse_FailureCode>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InsertResponse {}

impl InsertResponse {
    pub fn new() -> InsertResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InsertResponse {
        static mut instance: ::protobuf::lazy::Lazy<InsertResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InsertResponse,
        };
        unsafe {
            instance.get(|| {
                InsertResponse {
                    success: ::std::option::Option::None,
                    _id: ::std::option::Option::None,
                    failure_code: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success(&self) -> bool {
        self.success.unwrap_or(false)
    }

    // optional uint64 _id = 2;

    pub fn clear__id(&mut self) {
        self._id = ::std::option::Option::None;
    }

    pub fn has__id(&self) -> bool {
        self._id.is_some()
    }

    // Param is passed by value, moved
    pub fn set__id(&mut self, v: u64) {
        self._id = ::std::option::Option::Some(v);
    }

    pub fn get__id(&self) -> u64 {
        self._id.unwrap_or(0)
    }

    // optional .protodb.server.InsertResponse.FailureCode failure_code = 3;

    pub fn clear_failure_code(&mut self) {
        self.failure_code = ::std::option::Option::None;
    }

    pub fn has_failure_code(&self) -> bool {
        self.failure_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_failure_code(&mut self, v: InsertResponse_FailureCode) {
        self.failure_code = ::std::option::Option::Some(v);
    }

    pub fn get_failure_code(&self) -> InsertResponse_FailureCode {
        self.failure_code.unwrap_or(InsertResponse_FailureCode::INVALID_DB)
    }
}

impl ::protobuf::Message for InsertResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.success = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self._id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.failure_code = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.success.is_some() {
            my_size += 2;
        };
        for value in &self._id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.failure_code {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self._id {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.failure_code {
            try!(os.write_enum(3, v.value()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<InsertResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InsertResponse {
    fn new() -> InsertResponse {
        InsertResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<InsertResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    InsertResponse::has_success,
                    InsertResponse::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "_id",
                    InsertResponse::has__id,
                    InsertResponse::get__id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "failure_code",
                    InsertResponse::has_failure_code,
                    InsertResponse::get_failure_code,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InsertResponse>(
                    "InsertResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InsertResponse {
    fn clear(&mut self) {
        self.clear_success();
        self.clear__id();
        self.clear_failure_code();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InsertResponse {
    fn eq(&self, other: &InsertResponse) -> bool {
        self.success == other.success &&
        self._id == other._id &&
        self.failure_code == other.failure_code &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InsertResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum InsertResponse_FailureCode {
    INVALID_DB = 0,
    INVALID_COLLECTION = 1,
}

impl ::protobuf::ProtobufEnum for InsertResponse_FailureCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<InsertResponse_FailureCode> {
        match value {
            0 => ::std::option::Option::Some(InsertResponse_FailureCode::INVALID_DB),
            1 => ::std::option::Option::Some(InsertResponse_FailureCode::INVALID_COLLECTION),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [InsertResponse_FailureCode] = &[
            InsertResponse_FailureCode::INVALID_DB,
            InsertResponse_FailureCode::INVALID_COLLECTION,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<InsertResponse_FailureCode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("InsertResponse_FailureCode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for InsertResponse_FailureCode {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2f, 0x69, 0x6e, 0x73, 0x65, 0x72, 0x74, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x62, 0x2e, 0x73,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x22, 0x58, 0x0a, 0x06, 0x49, 0x6e, 0x73, 0x65, 0x72, 0x74, 0x12,
    0x1a, 0x0a, 0x08, 0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x08, 0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x12, 0x1e, 0x0a, 0x0a, 0x63,
    0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0a, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x12, 0x0a, 0x04, 0x64,
    0x61, 0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x22,
    0xc1, 0x01, 0x0a, 0x0e, 0x49, 0x6e, 0x73, 0x65, 0x72, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x12, 0x0f, 0x0a, 0x03,
    0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x12, 0x4d, 0x0a,
    0x0c, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x2a, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x62, 0x2e, 0x73, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x2e, 0x49, 0x6e, 0x73, 0x65, 0x72, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x2e, 0x46, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x43, 0x6f, 0x64, 0x65, 0x52,
    0x0b, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x43, 0x6f, 0x64, 0x65, 0x22, 0x35, 0x0a, 0x0b,
    0x46, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x43, 0x6f, 0x64, 0x65, 0x12, 0x0e, 0x0a, 0x0a, 0x49,
    0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x5f, 0x44, 0x42, 0x10, 0x00, 0x12, 0x16, 0x0a, 0x12, 0x49,
    0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x5f, 0x43, 0x4f, 0x4c, 0x4c, 0x45, 0x43, 0x54, 0x49, 0x4f,
    0x4e, 0x10, 0x01, 0x4a, 0xea, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x13, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02,
    0x08, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00, 0x08, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x05, 0x02, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x05, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05,
    0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x14, 0x15,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x02, 0x18, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x06, 0x02, 0x05, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x09, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x06, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x07, 0x02, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x07, 0x02,
    0x06, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x07, 0x02, 0x07,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x07, 0x08, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x07, 0x0f, 0x10, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x01, 0x12, 0x04, 0x0a, 0x00, 0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12,
    0x03, 0x0a, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02,
    0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0b, 0x02, 0x0a, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x02, 0x06, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x07, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x0c, 0x02, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x0c, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c,
    0x09, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x0f, 0x10,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x04, 0x00, 0x12, 0x04, 0x0e, 0x02, 0x11, 0x03, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x07, 0x12, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x04, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x04, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0f, 0x11, 0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x01, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x10, 0x04, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x10, 0x04, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x10, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x02, 0x12, 0x03, 0x12, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x12, 0x02, 0x11, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12,
    0x03, 0x12, 0x02, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x12,
    0x0e, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x12, 0x1d, 0x1e,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}

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
pub struct Find {
    // message fields
    database: ::protobuf::SingularField<::std::string::String>,
    collection: ::protobuf::SingularField<::std::string::String>,
    query: ::protobuf::SingularPtrField<Query>,
    projection: ::protobuf::SingularPtrField<Projection>,
    cascade: ::protobuf::SingularPtrField<Cascade>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Find {}

impl Find {
    pub fn new() -> Find {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Find {
        static mut instance: ::protobuf::lazy::Lazy<Find> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Find,
        };
        unsafe {
            instance.get(|| {
                Find {
                    database: ::protobuf::SingularField::none(),
                    collection: ::protobuf::SingularField::none(),
                    query: ::protobuf::SingularPtrField::none(),
                    projection: ::protobuf::SingularPtrField::none(),
                    cascade: ::protobuf::SingularPtrField::none(),
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

    // optional .protodb.server.Query query = 3;

    pub fn clear_query(&mut self) {
        self.query.clear();
    }

    pub fn has_query(&self) -> bool {
        self.query.is_some()
    }

    // Param is passed by value, moved
    pub fn set_query(&mut self, v: Query) {
        self.query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query(&mut self) -> &mut Query {
        if self.query.is_none() {
            self.query.set_default();
        };
        self.query.as_mut().unwrap()
    }

    // Take field
    pub fn take_query(&mut self) -> Query {
        self.query.take().unwrap_or_else(|| Query::new())
    }

    pub fn get_query(&self) -> &Query {
        self.query.as_ref().unwrap_or_else(|| Query::default_instance())
    }

    // optional .protodb.server.Projection projection = 4;

    pub fn clear_projection(&mut self) {
        self.projection.clear();
    }

    pub fn has_projection(&self) -> bool {
        self.projection.is_some()
    }

    // Param is passed by value, moved
    pub fn set_projection(&mut self, v: Projection) {
        self.projection = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_projection(&mut self) -> &mut Projection {
        if self.projection.is_none() {
            self.projection.set_default();
        };
        self.projection.as_mut().unwrap()
    }

    // Take field
    pub fn take_projection(&mut self) -> Projection {
        self.projection.take().unwrap_or_else(|| Projection::new())
    }

    pub fn get_projection(&self) -> &Projection {
        self.projection.as_ref().unwrap_or_else(|| Projection::default_instance())
    }

    // optional .protodb.server.Cascade cascade = 5;

    pub fn clear_cascade(&mut self) {
        self.cascade.clear();
    }

    pub fn has_cascade(&self) -> bool {
        self.cascade.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cascade(&mut self, v: Cascade) {
        self.cascade = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cascade(&mut self) -> &mut Cascade {
        if self.cascade.is_none() {
            self.cascade.set_default();
        };
        self.cascade.as_mut().unwrap()
    }

    // Take field
    pub fn take_cascade(&mut self) -> Cascade {
        self.cascade.take().unwrap_or_else(|| Cascade::new())
    }

    pub fn get_cascade(&self) -> &Cascade {
        self.cascade.as_ref().unwrap_or_else(|| Cascade::default_instance())
    }
}

impl ::protobuf::Message for Find {
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.query));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.projection));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cascade));
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
        for value in &self.query {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.projection {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.cascade {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        if let Some(v) = self.query.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.projection.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cascade.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<Find>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Find {
    fn new() -> Find {
        Find::new()
    }

    fn descriptor_static(_: ::std::option::Option<Find>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "database",
                    Find::has_database,
                    Find::get_database,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "collection",
                    Find::has_collection,
                    Find::get_collection,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "query",
                    Find::has_query,
                    Find::get_query,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "projection",
                    Find::has_projection,
                    Find::get_projection,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cascade",
                    Find::has_cascade,
                    Find::get_cascade,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Find>(
                    "Find",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Find {
    fn clear(&mut self) {
        self.clear_database();
        self.clear_collection();
        self.clear_query();
        self.clear_projection();
        self.clear_cascade();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Find {
    fn eq(&self, other: &Find) -> bool {
        self.database == other.database &&
        self.collection == other.collection &&
        self.query == other.query &&
        self.projection == other.projection &&
        self.cascade == other.cascade &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Find {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Query {
    // message fields
    query_field_options: ::protobuf::RepeatedField<Query_QueryFieldOptions>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Query {}

impl Query {
    pub fn new() -> Query {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Query {
        static mut instance: ::protobuf::lazy::Lazy<Query> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Query,
        };
        unsafe {
            instance.get(|| {
                Query {
                    query_field_options: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .protodb.server.Query.QueryFieldOptions query_field_options = 18;

    pub fn clear_query_field_options(&mut self) {
        self.query_field_options.clear();
    }

    // Param is passed by value, moved
    pub fn set_query_field_options(&mut self, v: ::protobuf::RepeatedField<Query_QueryFieldOptions>) {
        self.query_field_options = v;
    }

    // Mutable pointer to the field.
    pub fn mut_query_field_options(&mut self) -> &mut ::protobuf::RepeatedField<Query_QueryFieldOptions> {
        &mut self.query_field_options
    }

    // Take field
    pub fn take_query_field_options(&mut self) -> ::protobuf::RepeatedField<Query_QueryFieldOptions> {
        ::std::mem::replace(&mut self.query_field_options, ::protobuf::RepeatedField::new())
    }

    pub fn get_query_field_options(&self) -> &[Query_QueryFieldOptions] {
        &self.query_field_options
    }
}

impl ::protobuf::Message for Query {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                18 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.query_field_options));
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
        for value in &self.query_field_options {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.query_field_options {
            try!(os.write_tag(18, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<Query>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Query {
    fn new() -> Query {
        Query::new()
    }

    fn descriptor_static(_: ::std::option::Option<Query>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "query_field_options",
                    Query::get_query_field_options,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Query>(
                    "Query",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Query {
    fn clear(&mut self) {
        self.clear_query_field_options();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Query {
    fn eq(&self, other: &Query) -> bool {
        self.query_field_options == other.query_field_options &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Query {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Query_QueryFieldOptions {
    // message fields
    field: ::protobuf::SingularField<::std::string::String>,
    operator: ::std::option::Option<Query_QueryFieldOptions_Operator>,
    // message oneof groups
    value: ::std::option::Option<Query_QueryFieldOptions_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Query_QueryFieldOptions {}

#[derive(Clone,PartialEq)]
pub enum Query_QueryFieldOptions_oneof_value {
    float_value(f32),
    double_value(f64),
    int32_value(i32),
    int64_value(i64),
    uint32_value(u32),
    uint64_value(u64),
    sint32_value(i32),
    sint64_value(i64),
    fixed32_value(u32),
    fixed64_value(u64),
    sfixed32_value(i32),
    sfixed64_value(i64),
    bool_value(bool),
    string_value(::std::string::String),
    bytes_value(::std::vec::Vec<u8>),
}

impl Query_QueryFieldOptions {
    pub fn new() -> Query_QueryFieldOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Query_QueryFieldOptions {
        static mut instance: ::protobuf::lazy::Lazy<Query_QueryFieldOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Query_QueryFieldOptions,
        };
        unsafe {
            instance.get(|| {
                Query_QueryFieldOptions {
                    field: ::protobuf::SingularField::none(),
                    operator: ::std::option::Option::None,
                    value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string field = 1;

    pub fn clear_field(&mut self) {
        self.field.clear();
    }

    pub fn has_field(&self) -> bool {
        self.field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field(&mut self, v: ::std::string::String) {
        self.field = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field(&mut self) -> &mut ::std::string::String {
        if self.field.is_none() {
            self.field.set_default();
        };
        self.field.as_mut().unwrap()
    }

    // Take field
    pub fn take_field(&mut self) -> ::std::string::String {
        self.field.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_field(&self) -> &str {
        match self.field.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .protodb.server.Query.QueryFieldOptions.Operator operator = 2;

    pub fn clear_operator(&mut self) {
        self.operator = ::std::option::Option::None;
    }

    pub fn has_operator(&self) -> bool {
        self.operator.is_some()
    }

    // Param is passed by value, moved
    pub fn set_operator(&mut self, v: Query_QueryFieldOptions_Operator) {
        self.operator = ::std::option::Option::Some(v);
    }

    pub fn get_operator(&self) -> Query_QueryFieldOptions_Operator {
        self.operator.unwrap_or(Query_QueryFieldOptions_Operator::EQUAL)
    }

    // optional float float_value = 3;

    pub fn clear_float_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_float_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::float_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_float_value(&mut self, v: f32) {
        self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::float_value(v))
    }

    pub fn get_float_value(&self) -> f32 {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::float_value(v)) => v,
            _ => 0.,
        }
    }

    // optional double double_value = 4;

    pub fn clear_double_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_double_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::double_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_double_value(&mut self, v: f64) {
        self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::double_value(v))
    }

    pub fn get_double_value(&self) -> f64 {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::double_value(v)) => v,
            _ => 0.,
        }
    }

    // optional int32 int32_value = 5;

    pub fn clear_int32_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_int32_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::int32_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int32_value(&mut self, v: i32) {
        self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::int32_value(v))
    }

    pub fn get_int32_value(&self) -> i32 {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::int32_value(v)) => v,
            _ => 0,
        }
    }

    // optional int64 int64_value = 6;

    pub fn clear_int64_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_int64_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::int64_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int64_value(&mut self, v: i64) {
        self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::int64_value(v))
    }

    pub fn get_int64_value(&self) -> i64 {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::int64_value(v)) => v,
            _ => 0,
        }
    }

    // optional uint32 uint32_value = 7;

    pub fn clear_uint32_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_uint32_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::uint32_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_uint32_value(&mut self, v: u32) {
        self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::uint32_value(v))
    }

    pub fn get_uint32_value(&self) -> u32 {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::uint32_value(v)) => v,
            _ => 0,
        }
    }

    // optional uint64 uint64_value = 8;

    pub fn clear_uint64_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_uint64_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::uint64_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_uint64_value(&mut self, v: u64) {
        self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::uint64_value(v))
    }

    pub fn get_uint64_value(&self) -> u64 {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::uint64_value(v)) => v,
            _ => 0,
        }
    }

    // optional sint32 sint32_value = 9;

    pub fn clear_sint32_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_sint32_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::sint32_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_sint32_value(&mut self, v: i32) {
        self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::sint32_value(v))
    }

    pub fn get_sint32_value(&self) -> i32 {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::sint32_value(v)) => v,
            _ => 0,
        }
    }

    // optional sint64 sint64_value = 10;

    pub fn clear_sint64_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_sint64_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::sint64_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_sint64_value(&mut self, v: i64) {
        self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::sint64_value(v))
    }

    pub fn get_sint64_value(&self) -> i64 {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::sint64_value(v)) => v,
            _ => 0,
        }
    }

    // optional fixed32 fixed32_value = 11;

    pub fn clear_fixed32_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_fixed32_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::fixed32_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_fixed32_value(&mut self, v: u32) {
        self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::fixed32_value(v))
    }

    pub fn get_fixed32_value(&self) -> u32 {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::fixed32_value(v)) => v,
            _ => 0,
        }
    }

    // optional fixed64 fixed64_value = 12;

    pub fn clear_fixed64_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_fixed64_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::fixed64_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_fixed64_value(&mut self, v: u64) {
        self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::fixed64_value(v))
    }

    pub fn get_fixed64_value(&self) -> u64 {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::fixed64_value(v)) => v,
            _ => 0,
        }
    }

    // optional sfixed32 sfixed32_value = 13;

    pub fn clear_sfixed32_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_sfixed32_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::sfixed32_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_sfixed32_value(&mut self, v: i32) {
        self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::sfixed32_value(v))
    }

    pub fn get_sfixed32_value(&self) -> i32 {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::sfixed32_value(v)) => v,
            _ => 0,
        }
    }

    // optional sfixed64 sfixed64_value = 14;

    pub fn clear_sfixed64_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_sfixed64_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::sfixed64_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_sfixed64_value(&mut self, v: i64) {
        self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::sfixed64_value(v))
    }

    pub fn get_sfixed64_value(&self) -> i64 {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::sfixed64_value(v)) => v,
            _ => 0,
        }
    }

    // optional bool bool_value = 15;

    pub fn clear_bool_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_bool_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::bool_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bool_value(&mut self, v: bool) {
        self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::bool_value(v))
    }

    pub fn get_bool_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::bool_value(v)) => v,
            _ => false,
        }
    }

    // optional string string_value = 16;

    pub fn clear_string_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_string_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::string_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_string_value(&mut self, v: ::std::string::String) {
        self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::string_value(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_value(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::string_value(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::string_value(::std::string::String::new()));
        }
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::string_value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_string_value(&mut self) -> ::std::string::String {
        if self.has_string_value() {
            match self.value.take() {
                ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::string_value(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_string_value(&self) -> &str {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::string_value(ref v)) => v,
            _ => "",
        }
    }

    // optional bytes bytes_value = 17;

    pub fn clear_bytes_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_bytes_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::bytes_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bytes_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::bytes_value(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bytes_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::bytes_value(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::bytes_value(::std::vec::Vec::new()));
        }
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::bytes_value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_bytes_value(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_bytes_value() {
            match self.value.take() {
                ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::bytes_value(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_bytes_value(&self) -> &[u8] {
        match self.value {
            ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::bytes_value(ref v)) => v,
            _ => &[],
        }
    }
}

impl ::protobuf::Message for Query_QueryFieldOptions {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.field));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.operator = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::float_value(try!(is.read_float())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::double_value(try!(is.read_double())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::int32_value(try!(is.read_int32())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::int64_value(try!(is.read_int64())));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::uint32_value(try!(is.read_uint32())));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::uint64_value(try!(is.read_uint64())));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::sint32_value(try!(is.read_sint32())));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::sint64_value(try!(is.read_sint64())));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::fixed32_value(try!(is.read_fixed32())));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::fixed64_value(try!(is.read_fixed64())));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::sfixed32_value(try!(is.read_sfixed32())));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::sfixed64_value(try!(is.read_sfixed64())));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::bool_value(try!(is.read_bool())));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::string_value(try!(is.read_string())));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(Query_QueryFieldOptions_oneof_value::bytes_value(try!(is.read_bytes())));
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
        for value in &self.field {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.operator {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &Query_QueryFieldOptions_oneof_value::float_value(v) => {
                    my_size += 5;
                },
                &Query_QueryFieldOptions_oneof_value::double_value(v) => {
                    my_size += 9;
                },
                &Query_QueryFieldOptions_oneof_value::int32_value(v) => {
                    my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Query_QueryFieldOptions_oneof_value::int64_value(v) => {
                    my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Query_QueryFieldOptions_oneof_value::uint32_value(v) => {
                    my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Query_QueryFieldOptions_oneof_value::uint64_value(v) => {
                    my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Query_QueryFieldOptions_oneof_value::sint32_value(v) => {
                    my_size += ::protobuf::rt::value_varint_zigzag_size(9, v);
                },
                &Query_QueryFieldOptions_oneof_value::sint64_value(v) => {
                    my_size += ::protobuf::rt::value_varint_zigzag_size(10, v);
                },
                &Query_QueryFieldOptions_oneof_value::fixed32_value(v) => {
                    my_size += 5;
                },
                &Query_QueryFieldOptions_oneof_value::fixed64_value(v) => {
                    my_size += 9;
                },
                &Query_QueryFieldOptions_oneof_value::sfixed32_value(v) => {
                    my_size += 5;
                },
                &Query_QueryFieldOptions_oneof_value::sfixed64_value(v) => {
                    my_size += 9;
                },
                &Query_QueryFieldOptions_oneof_value::bool_value(v) => {
                    my_size += 2;
                },
                &Query_QueryFieldOptions_oneof_value::string_value(ref v) => {
                    my_size += ::protobuf::rt::string_size(16, &v);
                },
                &Query_QueryFieldOptions_oneof_value::bytes_value(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(17, &v);
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.operator {
            try!(os.write_enum(2, v.value()));
        };
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &Query_QueryFieldOptions_oneof_value::float_value(v) => {
                    try!(os.write_float(3, v));
                },
                &Query_QueryFieldOptions_oneof_value::double_value(v) => {
                    try!(os.write_double(4, v));
                },
                &Query_QueryFieldOptions_oneof_value::int32_value(v) => {
                    try!(os.write_int32(5, v));
                },
                &Query_QueryFieldOptions_oneof_value::int64_value(v) => {
                    try!(os.write_int64(6, v));
                },
                &Query_QueryFieldOptions_oneof_value::uint32_value(v) => {
                    try!(os.write_uint32(7, v));
                },
                &Query_QueryFieldOptions_oneof_value::uint64_value(v) => {
                    try!(os.write_uint64(8, v));
                },
                &Query_QueryFieldOptions_oneof_value::sint32_value(v) => {
                    try!(os.write_sint32(9, v));
                },
                &Query_QueryFieldOptions_oneof_value::sint64_value(v) => {
                    try!(os.write_sint64(10, v));
                },
                &Query_QueryFieldOptions_oneof_value::fixed32_value(v) => {
                    try!(os.write_fixed32(11, v));
                },
                &Query_QueryFieldOptions_oneof_value::fixed64_value(v) => {
                    try!(os.write_fixed64(12, v));
                },
                &Query_QueryFieldOptions_oneof_value::sfixed32_value(v) => {
                    try!(os.write_sfixed32(13, v));
                },
                &Query_QueryFieldOptions_oneof_value::sfixed64_value(v) => {
                    try!(os.write_sfixed64(14, v));
                },
                &Query_QueryFieldOptions_oneof_value::bool_value(v) => {
                    try!(os.write_bool(15, v));
                },
                &Query_QueryFieldOptions_oneof_value::string_value(ref v) => {
                    try!(os.write_string(16, v));
                },
                &Query_QueryFieldOptions_oneof_value::bytes_value(ref v) => {
                    try!(os.write_bytes(17, v));
                },
            };
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
        ::std::any::TypeId::of::<Query_QueryFieldOptions>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Query_QueryFieldOptions {
    fn new() -> Query_QueryFieldOptions {
        Query_QueryFieldOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<Query_QueryFieldOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "field",
                    Query_QueryFieldOptions::has_field,
                    Query_QueryFieldOptions::get_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "operator",
                    Query_QueryFieldOptions::has_operator,
                    Query_QueryFieldOptions::get_operator,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "float_value",
                    Query_QueryFieldOptions::has_float_value,
                    Query_QueryFieldOptions::get_float_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "double_value",
                    Query_QueryFieldOptions::has_double_value,
                    Query_QueryFieldOptions::get_double_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "int32_value",
                    Query_QueryFieldOptions::has_int32_value,
                    Query_QueryFieldOptions::get_int32_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "int64_value",
                    Query_QueryFieldOptions::has_int64_value,
                    Query_QueryFieldOptions::get_int64_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "uint32_value",
                    Query_QueryFieldOptions::has_uint32_value,
                    Query_QueryFieldOptions::get_uint32_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "uint64_value",
                    Query_QueryFieldOptions::has_uint64_value,
                    Query_QueryFieldOptions::get_uint64_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sint32_value",
                    Query_QueryFieldOptions::has_sint32_value,
                    Query_QueryFieldOptions::get_sint32_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "sint64_value",
                    Query_QueryFieldOptions::has_sint64_value,
                    Query_QueryFieldOptions::get_sint64_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "fixed32_value",
                    Query_QueryFieldOptions::has_fixed32_value,
                    Query_QueryFieldOptions::get_fixed32_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "fixed64_value",
                    Query_QueryFieldOptions::has_fixed64_value,
                    Query_QueryFieldOptions::get_fixed64_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sfixed32_value",
                    Query_QueryFieldOptions::has_sfixed32_value,
                    Query_QueryFieldOptions::get_sfixed32_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "sfixed64_value",
                    Query_QueryFieldOptions::has_sfixed64_value,
                    Query_QueryFieldOptions::get_sfixed64_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "bool_value",
                    Query_QueryFieldOptions::has_bool_value,
                    Query_QueryFieldOptions::get_bool_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "string_value",
                    Query_QueryFieldOptions::has_string_value,
                    Query_QueryFieldOptions::get_string_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bytes_value",
                    Query_QueryFieldOptions::has_bytes_value,
                    Query_QueryFieldOptions::get_bytes_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Query_QueryFieldOptions>(
                    "Query_QueryFieldOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Query_QueryFieldOptions {
    fn clear(&mut self) {
        self.clear_field();
        self.clear_operator();
        self.clear_float_value();
        self.clear_double_value();
        self.clear_int32_value();
        self.clear_int64_value();
        self.clear_uint32_value();
        self.clear_uint64_value();
        self.clear_sint32_value();
        self.clear_sint64_value();
        self.clear_fixed32_value();
        self.clear_fixed64_value();
        self.clear_sfixed32_value();
        self.clear_sfixed64_value();
        self.clear_bool_value();
        self.clear_string_value();
        self.clear_bytes_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Query_QueryFieldOptions {
    fn eq(&self, other: &Query_QueryFieldOptions) -> bool {
        self.field == other.field &&
        self.operator == other.operator &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Query_QueryFieldOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Query_QueryFieldOptions_Operator {
    EQUAL, // 0
    EQ, // 0
    NOT_EQUAL, // 1
    NE, // 1
    LESS_THAN, // 2
    LT, // 2
    LESS_THAN_OR_EQUAL, // 3
    LTE, // 3
    GREATER_THAN, // 4
    GT, // 4
    GREATER_THAN_OR_EQUAL, // 5
    GTE, // 5
}

impl ::protobuf::ProtobufEnum for Query_QueryFieldOptions_Operator {
    fn value(&self) -> i32 {
        match *self {
            Query_QueryFieldOptions_Operator::EQUAL => 0,
            Query_QueryFieldOptions_Operator::EQ => 0,
            Query_QueryFieldOptions_Operator::NOT_EQUAL => 1,
            Query_QueryFieldOptions_Operator::NE => 1,
            Query_QueryFieldOptions_Operator::LESS_THAN => 2,
            Query_QueryFieldOptions_Operator::LT => 2,
            Query_QueryFieldOptions_Operator::LESS_THAN_OR_EQUAL => 3,
            Query_QueryFieldOptions_Operator::LTE => 3,
            Query_QueryFieldOptions_Operator::GREATER_THAN => 4,
            Query_QueryFieldOptions_Operator::GT => 4,
            Query_QueryFieldOptions_Operator::GREATER_THAN_OR_EQUAL => 5,
            Query_QueryFieldOptions_Operator::GTE => 5,
        }
    }

    fn from_i32(value: i32) -> ::std::option::Option<Query_QueryFieldOptions_Operator> {
        match value {
            0 => ::std::option::Option::Some(Query_QueryFieldOptions_Operator::EQUAL),
            1 => ::std::option::Option::Some(Query_QueryFieldOptions_Operator::NOT_EQUAL),
            2 => ::std::option::Option::Some(Query_QueryFieldOptions_Operator::LESS_THAN),
            3 => ::std::option::Option::Some(Query_QueryFieldOptions_Operator::LESS_THAN_OR_EQUAL),
            4 => ::std::option::Option::Some(Query_QueryFieldOptions_Operator::GREATER_THAN),
            5 => ::std::option::Option::Some(Query_QueryFieldOptions_Operator::GREATER_THAN_OR_EQUAL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Query_QueryFieldOptions_Operator] = &[
            Query_QueryFieldOptions_Operator::EQUAL,
            Query_QueryFieldOptions_Operator::EQ,
            Query_QueryFieldOptions_Operator::NOT_EQUAL,
            Query_QueryFieldOptions_Operator::NE,
            Query_QueryFieldOptions_Operator::LESS_THAN,
            Query_QueryFieldOptions_Operator::LT,
            Query_QueryFieldOptions_Operator::LESS_THAN_OR_EQUAL,
            Query_QueryFieldOptions_Operator::LTE,
            Query_QueryFieldOptions_Operator::GREATER_THAN,
            Query_QueryFieldOptions_Operator::GT,
            Query_QueryFieldOptions_Operator::GREATER_THAN_OR_EQUAL,
            Query_QueryFieldOptions_Operator::GTE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Query_QueryFieldOptions_Operator>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Query_QueryFieldOptions_Operator", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Query_QueryFieldOptions_Operator {
}

#[derive(Clone,Default)]
pub struct Projection {
    // message fields
    fields: ::protobuf::SingularPtrField<FieldMask>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Projection {}

impl Projection {
    pub fn new() -> Projection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Projection {
        static mut instance: ::protobuf::lazy::Lazy<Projection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Projection,
        };
        unsafe {
            instance.get(|| {
                Projection {
                    fields: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .protodb.server.FieldMask fields = 1;

    pub fn clear_fields(&mut self) {
        self.fields.clear();
    }

    pub fn has_fields(&self) -> bool {
        self.fields.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fields(&mut self, v: FieldMask) {
        self.fields = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fields(&mut self) -> &mut FieldMask {
        if self.fields.is_none() {
            self.fields.set_default();
        };
        self.fields.as_mut().unwrap()
    }

    // Take field
    pub fn take_fields(&mut self) -> FieldMask {
        self.fields.take().unwrap_or_else(|| FieldMask::new())
    }

    pub fn get_fields(&self) -> &FieldMask {
        self.fields.as_ref().unwrap_or_else(|| FieldMask::default_instance())
    }
}

impl ::protobuf::Message for Projection {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fields));
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
        for value in &self.fields {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fields.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<Projection>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Projection {
    fn new() -> Projection {
        Projection::new()
    }

    fn descriptor_static(_: ::std::option::Option<Projection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fields",
                    Projection::has_fields,
                    Projection::get_fields,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Projection>(
                    "Projection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Projection {
    fn clear(&mut self) {
        self.clear_fields();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Projection {
    fn eq(&self, other: &Projection) -> bool {
        self.fields == other.fields &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Projection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Cascade {
    // message fields
    cascade_mode: ::std::option::Option<Cascade_CascadeMode>,
    cascade_paths: ::protobuf::SingularPtrField<FieldMask>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Cascade {}

impl Cascade {
    pub fn new() -> Cascade {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Cascade {
        static mut instance: ::protobuf::lazy::Lazy<Cascade> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Cascade,
        };
        unsafe {
            instance.get(|| {
                Cascade {
                    cascade_mode: ::std::option::Option::None,
                    cascade_paths: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .protodb.server.Cascade.CascadeMode cascade_mode = 1;

    pub fn clear_cascade_mode(&mut self) {
        self.cascade_mode = ::std::option::Option::None;
    }

    pub fn has_cascade_mode(&self) -> bool {
        self.cascade_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cascade_mode(&mut self, v: Cascade_CascadeMode) {
        self.cascade_mode = ::std::option::Option::Some(v);
    }

    pub fn get_cascade_mode(&self) -> Cascade_CascadeMode {
        self.cascade_mode.unwrap_or(Cascade_CascadeMode::NONE)
    }

    // optional .protodb.server.FieldMask cascade_paths = 2;

    pub fn clear_cascade_paths(&mut self) {
        self.cascade_paths.clear();
    }

    pub fn has_cascade_paths(&self) -> bool {
        self.cascade_paths.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cascade_paths(&mut self, v: FieldMask) {
        self.cascade_paths = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cascade_paths(&mut self) -> &mut FieldMask {
        if self.cascade_paths.is_none() {
            self.cascade_paths.set_default();
        };
        self.cascade_paths.as_mut().unwrap()
    }

    // Take field
    pub fn take_cascade_paths(&mut self) -> FieldMask {
        self.cascade_paths.take().unwrap_or_else(|| FieldMask::new())
    }

    pub fn get_cascade_paths(&self) -> &FieldMask {
        self.cascade_paths.as_ref().unwrap_or_else(|| FieldMask::default_instance())
    }
}

impl ::protobuf::Message for Cascade {
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
                    let tmp = try!(is.read_enum());
                    self.cascade_mode = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cascade_paths));
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
        for value in &self.cascade_mode {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.cascade_paths {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cascade_mode {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.cascade_paths.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<Cascade>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Cascade {
    fn new() -> Cascade {
        Cascade::new()
    }

    fn descriptor_static(_: ::std::option::Option<Cascade>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "cascade_mode",
                    Cascade::has_cascade_mode,
                    Cascade::get_cascade_mode,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cascade_paths",
                    Cascade::has_cascade_paths,
                    Cascade::get_cascade_paths,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Cascade>(
                    "Cascade",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Cascade {
    fn clear(&mut self) {
        self.clear_cascade_mode();
        self.clear_cascade_paths();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Cascade {
    fn eq(&self, other: &Cascade) -> bool {
        self.cascade_mode == other.cascade_mode &&
        self.cascade_paths == other.cascade_paths &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Cascade {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Cascade_CascadeMode {
    NONE = 0,
    FULL = 1,
    ALL_BUT = 2,
    ONLY = 3,
}

impl ::protobuf::ProtobufEnum for Cascade_CascadeMode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Cascade_CascadeMode> {
        match value {
            0 => ::std::option::Option::Some(Cascade_CascadeMode::NONE),
            1 => ::std::option::Option::Some(Cascade_CascadeMode::FULL),
            2 => ::std::option::Option::Some(Cascade_CascadeMode::ALL_BUT),
            3 => ::std::option::Option::Some(Cascade_CascadeMode::ONLY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Cascade_CascadeMode] = &[
            Cascade_CascadeMode::NONE,
            Cascade_CascadeMode::FULL,
            Cascade_CascadeMode::ALL_BUT,
            Cascade_CascadeMode::ONLY,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Cascade_CascadeMode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Cascade_CascadeMode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Cascade_CascadeMode {
}

#[derive(Clone,Default)]
pub struct FieldMask {
    // message fields
    paths: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FieldMask {}

impl FieldMask {
    pub fn new() -> FieldMask {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FieldMask {
        static mut instance: ::protobuf::lazy::Lazy<FieldMask> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FieldMask,
        };
        unsafe {
            instance.get(|| {
                FieldMask {
                    paths: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string paths = 1;

    pub fn clear_paths(&mut self) {
        self.paths.clear();
    }

    // Param is passed by value, moved
    pub fn set_paths(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.paths = v;
    }

    // Mutable pointer to the field.
    pub fn mut_paths(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.paths
    }

    // Take field
    pub fn take_paths(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.paths, ::protobuf::RepeatedField::new())
    }

    pub fn get_paths(&self) -> &[::std::string::String] {
        &self.paths
    }
}

impl ::protobuf::Message for FieldMask {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.paths));
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
        for value in &self.paths {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.paths {
            try!(os.write_string(1, &v));
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
        ::std::any::TypeId::of::<FieldMask>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FieldMask {
    fn new() -> FieldMask {
        FieldMask::new()
    }

    fn descriptor_static(_: ::std::option::Option<FieldMask>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "paths",
                    FieldMask::get_paths,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FieldMask>(
                    "FieldMask",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FieldMask {
    fn clear(&mut self) {
        self.clear_paths();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FieldMask {
    fn eq(&self, other: &FieldMask) -> bool {
        self.paths == other.paths &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FieldMask {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FindResponse {
    // message fields
    database: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FindResponse {}

impl FindResponse {
    pub fn new() -> FindResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FindResponse {
        static mut instance: ::protobuf::lazy::Lazy<FindResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FindResponse,
        };
        unsafe {
            instance.get(|| {
                FindResponse {
                    database: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string database = 1;

    pub fn clear_database(&mut self) {
        self.database.clear();
    }

    // Param is passed by value, moved
    pub fn set_database(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.database = v;
    }

    // Mutable pointer to the field.
    pub fn mut_database(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.database
    }

    // Take field
    pub fn take_database(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.database, ::protobuf::RepeatedField::new())
    }

    pub fn get_database(&self) -> &[::std::string::String] {
        &self.database
    }
}

impl ::protobuf::Message for FindResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.database));
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.database {
            try!(os.write_string(1, &v));
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
        ::std::any::TypeId::of::<FindResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FindResponse {
    fn new() -> FindResponse {
        FindResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<FindResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "database",
                    FindResponse::get_database,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FindResponse>(
                    "FindResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FindResponse {
    fn clear(&mut self) {
        self.clear_database();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FindResponse {
    fn eq(&self, other: &FindResponse) -> bool {
        self.database == other.database &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FindResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2f, 0x66, 0x69, 0x6e, 0x64, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x0e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x62, 0x2e, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x22, 0xde, 0x01, 0x0a, 0x04, 0x46, 0x69, 0x6e, 0x64, 0x12, 0x1a, 0x0a, 0x08,
    0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08,
    0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x12, 0x1e, 0x0a, 0x0a, 0x63, 0x6f, 0x6c, 0x6c,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x63, 0x6f,
    0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x2b, 0x0a, 0x05, 0x71, 0x75, 0x65, 0x72,
    0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64,
    0x62, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x51, 0x75, 0x65, 0x72, 0x79, 0x52, 0x05,
    0x71, 0x75, 0x65, 0x72, 0x79, 0x12, 0x3a, 0x0a, 0x0a, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x64, 0x62, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x50, 0x72, 0x6f, 0x6a, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0a, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x31, 0x0a, 0x07, 0x63, 0x61, 0x73, 0x63, 0x61, 0x64, 0x65, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x17, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x62, 0x2e, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x2e, 0x43, 0x61, 0x73, 0x63, 0x61, 0x64, 0x65, 0x52, 0x07, 0x63, 0x61, 0x73,
    0x63, 0x61, 0x64, 0x65, 0x22, 0xbf, 0x07, 0x0a, 0x05, 0x51, 0x75, 0x65, 0x72, 0x79, 0x12, 0x57,
    0x0a, 0x13, 0x71, 0x75, 0x65, 0x72, 0x79, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x6f, 0x70,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x12, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x64, 0x62, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x51, 0x75, 0x65,
    0x72, 0x79, 0x2e, 0x51, 0x75, 0x65, 0x72, 0x79, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x4f, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x52, 0x11, 0x71, 0x75, 0x65, 0x72, 0x79, 0x46, 0x69, 0x65, 0x6c, 0x64,
    0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x1a, 0xdc, 0x06, 0x0a, 0x11, 0x51, 0x75, 0x65, 0x72,
    0x79, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x14, 0x0a,
    0x05, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x12, 0x4c, 0x0a, 0x08, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x6f, 0x72, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x30, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x62, 0x2e,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x51, 0x75, 0x65, 0x72, 0x79, 0x2e, 0x51, 0x75, 0x65,
    0x72, 0x79, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x4f,
    0x70, 0x65, 0x72, 0x61, 0x74, 0x6f, 0x72, 0x52, 0x08, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x6f,
    0x72, 0x12, 0x21, 0x0a, 0x0b, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x48, 0x00, 0x52, 0x0a, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x56,
    0x61, 0x6c, 0x75, 0x65, 0x12, 0x23, 0x0a, 0x0c, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x5f, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x48, 0x00, 0x52, 0x0b, 0x64, 0x6f,
    0x75, 0x62, 0x6c, 0x65, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x21, 0x0a, 0x0b, 0x69, 0x6e, 0x74,
    0x33, 0x32, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x48, 0x00,
    0x52, 0x0a, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x21, 0x0a, 0x0b,
    0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x03, 0x48, 0x00, 0x52, 0x0a, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12,
    0x23, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18,
    0x07, 0x20, 0x01, 0x28, 0x0d, 0x48, 0x00, 0x52, 0x0b, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x56,
    0x61, 0x6c, 0x75, 0x65, 0x12, 0x23, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x04, 0x48, 0x00, 0x52, 0x0b, 0x75, 0x69,
    0x6e, 0x74, 0x36, 0x34, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x23, 0x0a, 0x0c, 0x73, 0x69, 0x6e,
    0x74, 0x33, 0x32, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x11, 0x48,
    0x00, 0x52, 0x0b, 0x73, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x23,
    0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x0a,
    0x20, 0x01, 0x28, 0x12, 0x48, 0x00, 0x52, 0x0b, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x56, 0x61,
    0x6c, 0x75, 0x65, 0x12, 0x25, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x07, 0x48, 0x00, 0x52, 0x0c, 0x66, 0x69,
    0x78, 0x65, 0x64, 0x33, 0x32, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x25, 0x0a, 0x0d, 0x66, 0x69,
    0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28,
    0x06, 0x48, 0x00, 0x52, 0x0c, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x56, 0x61, 0x6c, 0x75,
    0x65, 0x12, 0x27, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0f, 0x48, 0x00, 0x52, 0x0d, 0x73, 0x66, 0x69,
    0x78, 0x65, 0x64, 0x33, 0x32, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x27, 0x0a, 0x0e, 0x73, 0x66,
    0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x0e, 0x20, 0x01,
    0x28, 0x10, 0x48, 0x00, 0x52, 0x0d, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x56, 0x61,
    0x6c, 0x75, 0x65, 0x12, 0x1f, 0x0a, 0x0a, 0x62, 0x6f, 0x6f, 0x6c, 0x5f, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x09, 0x62, 0x6f, 0x6f, 0x6c, 0x56,
    0x61, 0x6c, 0x75, 0x65, 0x12, 0x23, 0x0a, 0x0c, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x18, 0x10, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0b, 0x73, 0x74,
    0x72, 0x69, 0x6e, 0x67, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x21, 0x0a, 0x0b, 0x62, 0x79, 0x74,
    0x65, 0x73, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0c, 0x48, 0x00,
    0x52, 0x0a, 0x62, 0x79, 0x74, 0x65, 0x73, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x22, 0xae, 0x01, 0x0a,
    0x08, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x6f, 0x72, 0x12, 0x09, 0x0a, 0x05, 0x45, 0x51, 0x55,
    0x41, 0x4c, 0x10, 0x00, 0x12, 0x06, 0x0a, 0x02, 0x45, 0x51, 0x10, 0x00, 0x12, 0x0d, 0x0a, 0x09,
    0x4e, 0x4f, 0x54, 0x5f, 0x45, 0x51, 0x55, 0x41, 0x4c, 0x10, 0x01, 0x12, 0x06, 0x0a, 0x02, 0x4e,
    0x45, 0x10, 0x01, 0x12, 0x0d, 0x0a, 0x09, 0x4c, 0x45, 0x53, 0x53, 0x5f, 0x54, 0x48, 0x41, 0x4e,
    0x10, 0x02, 0x12, 0x06, 0x0a, 0x02, 0x4c, 0x54, 0x10, 0x02, 0x12, 0x16, 0x0a, 0x12, 0x4c, 0x45,
    0x53, 0x53, 0x5f, 0x54, 0x48, 0x41, 0x4e, 0x5f, 0x4f, 0x52, 0x5f, 0x45, 0x51, 0x55, 0x41, 0x4c,
    0x10, 0x03, 0x12, 0x07, 0x0a, 0x03, 0x4c, 0x54, 0x45, 0x10, 0x03, 0x12, 0x10, 0x0a, 0x0c, 0x47,
    0x52, 0x45, 0x41, 0x54, 0x45, 0x52, 0x5f, 0x54, 0x48, 0x41, 0x4e, 0x10, 0x04, 0x12, 0x06, 0x0a,
    0x02, 0x47, 0x54, 0x10, 0x04, 0x12, 0x19, 0x0a, 0x15, 0x47, 0x52, 0x45, 0x41, 0x54, 0x45, 0x52,
    0x5f, 0x54, 0x48, 0x41, 0x4e, 0x5f, 0x4f, 0x52, 0x5f, 0x45, 0x51, 0x55, 0x41, 0x4c, 0x10, 0x05,
    0x12, 0x07, 0x0a, 0x03, 0x47, 0x54, 0x45, 0x10, 0x05, 0x1a, 0x02, 0x10, 0x01, 0x42, 0x07, 0x0a,
    0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x3f, 0x0a, 0x0a, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x12, 0x31, 0x0a, 0x06, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x62, 0x2e, 0x73,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x4d, 0x61, 0x73, 0x6b, 0x52,
    0x06, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x22, 0xcb, 0x01, 0x0a, 0x07, 0x43, 0x61, 0x73, 0x63,
    0x61, 0x64, 0x65, 0x12, 0x46, 0x0a, 0x0c, 0x63, 0x61, 0x73, 0x63, 0x61, 0x64, 0x65, 0x5f, 0x6d,
    0x6f, 0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x23, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x64, 0x62, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x43, 0x61, 0x73, 0x63, 0x61,
    0x64, 0x65, 0x2e, 0x43, 0x61, 0x73, 0x63, 0x61, 0x64, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x52, 0x0b,
    0x63, 0x61, 0x73, 0x63, 0x61, 0x64, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x3e, 0x0a, 0x0d, 0x63,
    0x61, 0x73, 0x63, 0x61, 0x64, 0x65, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x73, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x19, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x62, 0x2e, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x2e, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x4d, 0x61, 0x73, 0x6b, 0x52, 0x0c, 0x63,
    0x61, 0x73, 0x63, 0x61, 0x64, 0x65, 0x50, 0x61, 0x74, 0x68, 0x73, 0x22, 0x38, 0x0a, 0x0b, 0x43,
    0x61, 0x73, 0x63, 0x61, 0x64, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x08, 0x0a, 0x04, 0x4e, 0x4f,
    0x4e, 0x45, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x46, 0x55, 0x4c, 0x4c, 0x10, 0x01, 0x12, 0x0b,
    0x0a, 0x07, 0x41, 0x4c, 0x4c, 0x5f, 0x42, 0x55, 0x54, 0x10, 0x02, 0x12, 0x08, 0x0a, 0x04, 0x4f,
    0x4e, 0x4c, 0x59, 0x10, 0x03, 0x22, 0x21, 0x0a, 0x09, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x4d, 0x61,
    0x73, 0x6b, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61, 0x74, 0x68, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x09, 0x52, 0x05, 0x70, 0x61, 0x74, 0x68, 0x73, 0x22, 0x2a, 0x0a, 0x0c, 0x46, 0x69, 0x6e, 0x64,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x64, 0x61, 0x74, 0x61,
    0x62, 0x61, 0x73, 0x65, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x52, 0x08, 0x64, 0x61, 0x74, 0x61,
    0x62, 0x61, 0x73, 0x65, 0x4a, 0xe6, 0x19, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x61, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x02, 0x08, 0x16, 0x0a, 0x4c, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0e, 0x01, 0x32,
    0x40, 0x20, 0x55, 0x73, 0x65, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f,
    0x72, 0x74, 0x65, 0x64, 0x0a, 0x20, 0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x22, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x5f, 0x6d, 0x61, 0x73, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x3b,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x0c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x02, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x08, 0x02, 0x07, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x08, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x08, 0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x08, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x02, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x09, 0x02, 0x08, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x09, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x09, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x09, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x0b, 0x02, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x0b, 0x02, 0x09, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x0b, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x10, 0x11, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x0c, 0x02, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x0c, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x0c, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x0c, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0d,
    0x02, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x04, 0x0d, 0x02, 0x0c,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x0d, 0x02, 0x09, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0d, 0x0a, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0d, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x10, 0x00, 0x46, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x10, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x03, 0x00, 0x12, 0x04, 0x11, 0x02, 0x43,
    0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x01, 0x12, 0x03, 0x11, 0x0a, 0x1b, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x12, 0x04, 0x15, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x12, 0x04, 0x11, 0x1d, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x12, 0x04, 0x0a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x0b, 0x10, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x12, 0x13, 0x14, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x12, 0x04, 0x14, 0x04, 0x27, 0x05, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x14, 0x09, 0x11, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x03, 0x12, 0x03, 0x15, 0x06, 0x20, 0x0a,
    0x11, 0x0a, 0x0a, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x03, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x15,
    0x06, 0x20, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x03, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x15, 0x0d, 0x18, 0x0a, 0x13, 0x0a, 0x0c, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00,
    0x03, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x15, 0x0d, 0x18, 0x0a, 0x14, 0x0a, 0x0d, 0x04,
    0x01, 0x03, 0x00, 0x04, 0x00, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x0d,
    0x18, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x03, 0xe7, 0x07, 0x00, 0x03,
    0x12, 0x03, 0x15, 0x1b, 0x1f, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x16, 0x06, 0x10, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x06, 0x0b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00,
    0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x16, 0x0e, 0x0f, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01,
    0x03, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x17, 0x06, 0x0d, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x17, 0x06, 0x08, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x17, 0x0b, 0x0c, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x19, 0x06, 0x14,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x19,
    0x06, 0x0f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x19, 0x12, 0x13, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x1a, 0x06, 0x0d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x1a, 0x06, 0x08, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x04,
    0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x1a, 0x0b, 0x0c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03,
    0x00, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x1c, 0x06, 0x14, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01,
    0x03, 0x00, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1c, 0x06, 0x0f, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x1c, 0x12, 0x13, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x1d, 0x06, 0x0d, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1d, 0x06,
    0x08, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03,
    0x1d, 0x0b, 0x0c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x06, 0x12,
    0x03, 0x1f, 0x06, 0x1d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x06,
    0x01, 0x12, 0x03, 0x1f, 0x06, 0x18, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00,
    0x02, 0x06, 0x02, 0x12, 0x03, 0x1f, 0x1b, 0x1c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00,
    0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x20, 0x06, 0x0e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03,
    0x00, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x20, 0x06, 0x09, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x20, 0x0c, 0x0d, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x22, 0x06, 0x17, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x22, 0x06, 0x12,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x22,
    0x15, 0x16, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03,
    0x23, 0x06, 0x0d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x09, 0x01,
    0x12, 0x03, 0x23, 0x06, 0x08, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02,
    0x09, 0x02, 0x12, 0x03, 0x23, 0x0b, 0x0c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x04,
    0x00, 0x02, 0x0a, 0x12, 0x03, 0x25, 0x06, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00,
    0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x25, 0x06, 0x1b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01,
    0x03, 0x00, 0x04, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x25, 0x1e, 0x1f, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x26, 0x06, 0x0e, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x26, 0x06, 0x09, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x0b, 0x02, 0x12, 0x03, 0x26, 0x0c,
    0x0d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x29, 0x04, 0x1a,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x29, 0x04, 0x27,
    0x05, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x29, 0x04,
    0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x29, 0x0d,
    0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x29, 0x18,
    0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x08, 0x00, 0x12, 0x04, 0x2b, 0x04, 0x42,
    0x05, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x08, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x0a,
    0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03, 0x2c, 0x06, 0x1c,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2c, 0x06, 0x0b,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x0c, 0x17,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2c, 0x1a, 0x1b,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x03, 0x12, 0x03, 0x2d, 0x06, 0x1e, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x2d, 0x06, 0x0c, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2d, 0x0d, 0x19, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2d, 0x1c, 0x1d, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x04, 0x12, 0x03, 0x2f, 0x06, 0x1c, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x2f, 0x06, 0x0b, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2f, 0x0c, 0x17, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2f, 0x1a, 0x1b, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x05, 0x12, 0x03, 0x30, 0x06, 0x1c, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x30, 0x06, 0x0b, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x30, 0x0c, 0x17, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x30, 0x1a, 0x1b, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x06, 0x12, 0x03, 0x32, 0x06, 0x1e, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x32, 0x06, 0x0c, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x32, 0x0d, 0x19, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x32, 0x1c, 0x1d, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x07, 0x12, 0x03, 0x33, 0x06, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x33, 0x06, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x33, 0x0d, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x33, 0x1c, 0x1d, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x08, 0x12, 0x03, 0x35, 0x06, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x35, 0x06, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x35, 0x0d, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x35, 0x1c, 0x1d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x09, 0x12, 0x03, 0x36, 0x06, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x36, 0x06, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x36, 0x0d, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x36, 0x1c, 0x1e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x0a, 0x12, 0x03, 0x38, 0x06, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x0a, 0x05, 0x12, 0x03, 0x38, 0x06, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x0a, 0x01, 0x12, 0x03, 0x38, 0x0e, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x0a, 0x03, 0x12, 0x03, 0x38, 0x1e, 0x20, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x0b, 0x12, 0x03, 0x39, 0x06, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x0b, 0x05, 0x12, 0x03, 0x39, 0x06, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x0b, 0x01, 0x12, 0x03, 0x39, 0x0e, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x0b, 0x03, 0x12, 0x03, 0x39, 0x1e, 0x20, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x0c, 0x12, 0x03, 0x3b, 0x06, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0c,
    0x05, 0x12, 0x03, 0x3b, 0x06, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0c,
    0x01, 0x12, 0x03, 0x3b, 0x0f, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0c,
    0x03, 0x12, 0x03, 0x3b, 0x20, 0x22, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0d,
    0x12, 0x03, 0x3c, 0x06, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0d, 0x05,
    0x12, 0x03, 0x3c, 0x06, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0d, 0x01,
    0x12, 0x03, 0x3c, 0x0f, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0d, 0x03,
    0x12, 0x03, 0x3c, 0x20, 0x22, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0e, 0x12,
    0x03, 0x3e, 0x06, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0e, 0x05, 0x12,
    0x03, 0x3e, 0x06, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0e, 0x01, 0x12,
    0x03, 0x3e, 0x0b, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0e, 0x03, 0x12,
    0x03, 0x3e, 0x18, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0f, 0x12, 0x03,
    0x40, 0x06, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0f, 0x05, 0x12, 0x03,
    0x40, 0x06, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x03,
    0x40, 0x0d, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0f, 0x03, 0x12, 0x03,
    0x40, 0x1c, 0x1e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x10, 0x12, 0x03, 0x41,
    0x06, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x10, 0x05, 0x12, 0x03, 0x41,
    0x06, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x10, 0x01, 0x12, 0x03, 0x41,
    0x0c, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x10, 0x03, 0x12, 0x03, 0x41,
    0x1a, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x45, 0x02, 0x36, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x45, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x45, 0x0b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x45, 0x1d, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x45, 0x33, 0x35, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x48,
    0x00, 0x4c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x48, 0x08, 0x12, 0x0a,
    0x47, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x4b, 0x02, 0x17, 0x1a, 0x3a, 0x20, 0x55,
    0x73, 0x65, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65,
    0x64, 0x0a, 0x20, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
    0x75, 0x66, 0x2e, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x4d, 0x61, 0x73, 0x6b, 0x20, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x20, 0x3d, 0x20, 0x31, 0x3b, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x4b, 0x02, 0x48, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x4b, 0x02, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x4b, 0x0c, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4b, 0x15,
    0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x4e, 0x00, 0x58, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x4e, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x04,
    0x00, 0x12, 0x04, 0x4f, 0x02, 0x54, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x4f, 0x07, 0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x50, 0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x50, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x50, 0x0b, 0x0c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x51, 0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x51, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x51, 0x0b, 0x0c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x52,
    0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x52,
    0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x52,
    0x0e, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x53, 0x04,
    0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x53, 0x04,
    0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x53, 0x0b,
    0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x56, 0x02, 0x1f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x56, 0x02, 0x54, 0x03, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x56, 0x02, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x56, 0x0e, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x56, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12,
    0x03, 0x57, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x04, 0x57,
    0x02, 0x56, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x57, 0x02,
    0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x57, 0x0c, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x57, 0x1c, 0x1d, 0x0a, 0x50, 0x0a,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x5b, 0x00, 0x5d, 0x01, 0x1a, 0x44, 0x20, 0x2f, 0x2f, 0x20, 0x52,
    0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x22, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x65, 0x73,
    0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x20, 0x77,
    0x68, 0x65, 0x6e, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x5b, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x00, 0x12, 0x03, 0x5c, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x5c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x5c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5c,
    0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5c, 0x1a, 0x1b,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x5f, 0x00, 0x61, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x05, 0x01, 0x12, 0x03, 0x5f, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00,
    0x12, 0x03, 0x60, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x60, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x60, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x60, 0x12, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x60, 0x1d, 0x1e, 0x62, 0x06, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x33,
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

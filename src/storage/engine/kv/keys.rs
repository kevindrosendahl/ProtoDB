use prost_types::field_descriptor_proto::Type;
use protodb_schema::encoding::FieldValue;

pub const KEY_DELIMITER: char = '/';
const SYSTEM_KEY_PREFIX: &str = "__system";
// FIXME: make this const or lazy_static
//const DATABASES_PREFIX: &str = SYSTEM_PREFIX + KEY_DELIMITER + DATABASES_DELIMITER + KEY_DELIMITER;

#[inline(always)]
pub fn delimiter_prefix_bound(start: String) -> (Vec<u8>, Vec<u8>) {
    // add 1 to the byte value of the last byte in the prefix
    // this should make the range span over all keys with the prefix
    // and no more
    let end = start.clone();
    let mut end = end.into_bytes();
    let last = end.pop().unwrap();
    end.push(last + 1);

    (start.into_bytes(), end)
}

#[inline(always)]
pub fn key_suffix(prefix: &str, key: &str) -> String {
    let mut key_parts = key.split(&prefix);

    // move past the prefix
    key_parts
        .next()
        .unwrap_or_else(|| panic!("corrupted key (no prefix): {}", key));

    // get the suffix
    let suffix = key_parts
        .next()
        .unwrap_or_else(|| panic!("corrupted key (no suffix): {}", key));

    String::from(suffix)
}

#[inline(always)]
// FIXME: make this const or lazy_static
pub fn databases_key_prefix() -> String {
    format!(
        "{system_prefix}{delimiter}databases{delimiter}",
        system_prefix = SYSTEM_KEY_PREFIX,
        delimiter = KEY_DELIMITER,
    )
}

#[inline(always)]
pub fn database_key(name: &str) -> String {
    format!(
        "{databases_prefix}{name}",
        databases_prefix = databases_key_prefix(),
        name = name,
    )
}

#[inline(always)]
pub fn database_from_key(key: &str) -> String {
    let prefix = databases_key_prefix();
    key_suffix(&prefix, key)
}

#[inline(always)]
pub fn database_key_prefix(name: &str) -> String {
    format!(
        "{system_prefix}{delimiter}database{delimiter}{name}{delimiter}",
        system_prefix = SYSTEM_KEY_PREFIX,
        delimiter = KEY_DELIMITER,
        name = name,
    )
}

#[inline(always)]
pub fn collections_key_prefix(database: &str) -> String {
    format!(
        "{database_prefix}collections{delimiter}",
        database_prefix = database_key_prefix(database),
        delimiter = KEY_DELIMITER,
    )
}

#[inline(always)]
pub fn collection_key(database: &str, name: &str) -> String {
    format!(
        "{collections_prefix}{name}",
        collections_prefix = collections_key_prefix(database),
        name = name,
    )
}

#[inline(always)]
pub fn collection_from_key(database: &str, key: &str) -> String {
    let prefix = collections_key_prefix(database);
    key_suffix(&prefix, key)
}

#[inline(always)]
pub fn collection_key_prefix(database: &str, name: &str) -> String {
    format!(
        "{database_prefix}collection{delimiter}{name}{delimiter}",
        database_prefix = database_key_prefix(database),
        delimiter = KEY_DELIMITER,
        name = name,
    )
}

#[inline(always)]
pub fn descriptor_key(database: &str, collection: &str) -> String {
    format!(
        "{collection_prefix}descriptor",
        collection_prefix = collection_key_prefix(database, collection),
    )
}

#[inline(always)]
pub fn tag_from_key(database: &str, collection: &str, key: &str, id: u64) -> i32 {
    let prefix = field_key_prefix(database, collection, id);
    let tag = key_suffix(&prefix, &key);
    tag.parse().unwrap()
}

#[inline(always)]
pub fn object_id_and_field_from_key(database: &str, collection: &str, key: &str) -> (u64, i32) {
    let prefix = collection_object_key_prefix(database, collection);
    let parts = key_suffix(&prefix, &key);
    let parts: Vec<&str> = parts.split(KEY_DELIMITER).collect();
    if parts.len() != 2 {
        panic!("corrupted key: {}", key);
    }
    (parts[0].parse().unwrap(), parts[1].parse().unwrap())
}

#[inline(always)]
pub fn collection_entity_key_prefix(database: &str, collection: &str) -> String {
    format!(
        "{database}{delimiter}{collection}{delimiter}",
        database = database,
        delimiter = KEY_DELIMITER,
        collection = collection,
    )
}

#[inline(always)]
pub fn collection_object_key_prefix(database: &str, collection: &str) -> String {
    // In order to have the keys sorted in the correct order,
    // pad the left of the id with 0s up to the length of the
    // longest u64.
    format!(
        "{prefix}objects{delimiter}",
        prefix = collection_entity_key_prefix(database, collection),
        delimiter = KEY_DELIMITER,
    )
}

#[inline(always)]
pub fn object_key_prefix(database: &str, collection: &str, id: u64) -> String {
    // In order to have the keys sorted in the correct order,
    // pad the left of the id with 0s up to the length of the
    // longest u64.
    format!(
        "{prefix}{id:0>20}",
        prefix = collection_object_key_prefix(database, collection),
        id = id,
    )
}

#[inline(always)]
pub fn field_key_prefix(database: &str, collection: &str, id: u64) -> String {
    format!(
        "{prefix}{delimiter}",
        prefix = object_key_prefix(database, collection, id),
        delimiter = KEY_DELIMITER,
    )
}

#[inline(always)]
pub fn field_key(database: &str, collection: &str, id: u64, tag: i32) -> String {
    // In order to have the keys sorted in the correct order,
    // pad the left of the id with 0s up to the length of the
    // longest i32.
    format!(
        "{prefix}{tag:0>10}",
        prefix = field_key_prefix(database, collection, id),
        tag = tag,
    )
}

#[inline(always)]
pub fn index_prefix(database: &str, collection: &str) -> String {
    format!(
        "{prefix}indexes{delimiter}",
        prefix = collection_entity_key_prefix(database, collection),
        delimiter = KEY_DELIMITER
    )
}

#[inline(always)]
pub fn index_key_prefix(database: &str, collection: &str, index_id: usize) -> String {
    // In order to have the keys sorted in the correct order,
    // pad the left of the id with 0s up to the length of the
    // longest usize (N.B. this currently assumes usize == u64)
    format!(
        "{prefix}{index_id:0>20}{delimiter}",
        prefix = index_prefix(database, collection),
        index_id = index_id,
        delimiter = KEY_DELIMITER,
    )
}

#[inline(always)]
pub fn index_object_key_prefix(
    database: &str,
    collection: &str,
    index_id: usize,
    value: &FieldValue,
) -> String {
    // FIXME: handle more field types
    let value = match value {
        FieldValue::Int32(val) => format!("{:0>10}", val),
        FieldValue::Uint32(val) => format!("{:0>10}", val),
        FieldValue::Int64(val) => format!("{:0>20}", val),
        FieldValue::Uint64(val) => format!("{:0>20}", val),
        // FIXME: handle this better
        _ => panic!("unindexible FieldValue type"),
    };
    format!(
        "{prefix}{value}",
        prefix = index_key_prefix(database, collection, index_id),
        value = value,
    )
}

#[inline(always)]
pub fn index_object_key(
    database: &str,
    collection: &str,
    index_id: usize,
    value: &FieldValue,
    object_id: u64,
) -> String {
    format!(
        "{prefix}{delimiter}{object_id:0>20}",
        prefix = index_object_key_prefix(database, collection, index_id, value),
        delimiter = KEY_DELIMITER,
        object_id = object_id,
    )
}

#[inline(always)]
pub fn value_and_id_from_index_key(
    database: &str,
    collection: &str,
    index_id: usize,
    field_type: Type,
    key: &str,
) -> (FieldValue, u64) {
    let prefix = index_key_prefix(database, collection, index_id);
    let suffix = key_suffix(&prefix, key);

    let mut suffix_parts = suffix.split(KEY_DELIMITER);
    let value = suffix_parts
        .next()
        .unwrap_or_else(|| panic!("corrupted index key (no value): {}", key));

    let id = suffix_parts
        .next()
        .unwrap_or_else(|| panic!("corrupted index key (no id): {}", key));

    let value = match field_type {
        Type::Int32 => FieldValue::Int32(value.parse().unwrap()),
        Type::Uint32 => FieldValue::Uint32(value.parse().unwrap()),
        Type::Int64 => FieldValue::Int64(value.parse().unwrap()),
        Type::Uint64 => FieldValue::Uint64(value.parse().unwrap()),
        // FIXME: handle this better
        _ => panic!("unindexible FieldValue type"),
    };

    let id = id.parse().unwrap();
    (value, id)
}

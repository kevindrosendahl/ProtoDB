pub mod collection;
pub mod database;
pub mod index;

pub(crate) const KEY_DELIMITER: char = '/';

#[inline(always)]
fn delimiter_prefix_bound(start: String) -> (Vec<u8>, Vec<u8>) {
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
fn key_suffix(prefix: &str, key: &str) -> String {
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

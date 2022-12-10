use std::borrow::Cow;

use super::{
    json::write_as_json,
    parse::{try_parse_value, IParseResult},
    value::Value,
};

pub fn str_to_value(s: &str) -> IParseResult<Value> {
    try_parse_value(s.bytes().into_iter())
}

pub fn str_to_json(s: &str) -> String {
    let value = str_to_value(s).unwrap();
    value_to_json(&value)
}

pub fn value_to_json(value: &Value) -> String {
    let mut bytes: Vec<u8> = Vec::new();

    write_as_json(&value, &mut bytes).unwrap();

    let res = String::from_utf8(bytes).unwrap();

    res
}

pub fn str_keys_lossy(value: &Value) -> impl Iterator<Item = Cow<str>> {
    value.keys().map(|k| k.to_lossy_str().unwrap())
}

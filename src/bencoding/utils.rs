use super::{
    json::write_as_json,
    parse::{try_parse_value, IParseResult},
    value::Value,
};

pub fn str_to_value(s: &str) -> IParseResult<Value> {
    try_parse_value(&mut s.bytes().into_iter().peekable())
}

pub fn str_to_json(s: &str) -> String {
    let value = str_to_value(s).unwrap();
    let mut bytes: Vec<u8> = Vec::new();

    write_as_json(&value, &mut bytes).unwrap();

    let res = String::from_utf8(bytes).unwrap();

    res
}

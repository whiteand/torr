use std::borrow::Cow;
use std::io::Read;

use super::{
    json::write_as_json,
    parse::{try_parse_value, IParseResult},
    value::{Value, ValueType},
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
// Pretty Pring functions
fn shorten(s: &str) -> String {
    if s.len() <= 54 {
        s.to_owned()
    } else {
        let chars = s.chars().collect::<Vec<_>>();

        const START: usize = 20;
        const END: usize = 20;
        let start = &chars[0..START].iter().collect::<String>();
        let end = &chars[(chars.len() - END)..].iter().collect::<String>();

        format!("{}...{} chars...{}", start, chars.len() - START - END, end)
    }
}

fn recursive_print(value: &Value, prefix: &str) {
    match value.get_type() {
        ValueType::String => {
            let s = match value {
                Value::String(bytes) => match String::from_utf8(bytes.clone()) {
                    Ok(s) => s,
                    Err(_) => format!(
                        "0x{}",
                        bytes
                            .iter()
                            .map(|b| format!("{:x}", *b))
                            .collect::<String>()
                    ),
                },
                _ => unreachable!(),
            };
            println!("{} = '{}'", prefix, shorten(&s));
        }
        ValueType::Integer => {
            let int = match value {
                Value::Integer(n) => *n,
                _ => unreachable!(),
            };
            println!("{} = {}", prefix, int);
        }
        ValueType::List => {
            for (ind, v) in value.values().enumerate() {
                let new_prefix = format!("{}[{}]", prefix, ind);
                recursive_print(v, &new_prefix);
            }
        }
        ValueType::Dictionary => {
            for (key, value) in value.entries() {
                let new_prefix = format!("{}['{}']", prefix, key.to_lossy_str().unwrap());
                recursive_print(value, &new_prefix);
            }
        }
    }
}

pub fn print_metainfo(
    r: &mut impl std::io::Read,
) -> Result<(), crate::bencoding::parse::ParseError> {
    let source = r.bytes().take_while(|x| x.is_ok()).map(|x| x.unwrap());
    let value = try_parse_value(source)?;

    recursive_print(&value, "");
    Ok(())
}

use std::io::Read;

use crate::bencoding::{
    parse::try_parse_value,
    value::{Value, ValueType},
};

/*
interface IMetaInfo {
    announce: string
    announce-list: string[][]
    azureus_properties?: {
        dht_backup_enable: 1 | 0
    },
    privage: 1 | 0,
    "creation date": number // e.g 1608033138
    comment: string,
    info: {
        length: number, // 36_947_471_188
        name: string,
        "piece length": number,
        pieces: string
        "file-duration": unknown,
        "profiles": unknown,
    }

}
*/

fn shorten(s: &str) -> String {
    if s.len() <= 80 {
        s.to_owned()
    } else {
        let start = s.chars().take(20).collect::<String>();
        let end = s.chars().skip(s.len() - 20).collect::<String>();

        format!("{}...{}", start, end)
    }
}

fn recursive_print(value: &Value, prefix: &str) {
    match value.get_type() {
        ValueType::String => {
            println!("{} = '{}'", prefix, shorten(&value.to_lossy_str().unwrap()));
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

// TODO: Improve error handling
pub fn read(
    r: &mut impl std::io::Read,
) -> Result<super::MetaInfo, crate::bencoding::parse::ParseError> {
    let source = r.bytes().take_while(|x| x.is_ok()).map(|x| x.unwrap());
    let value = try_parse_value(source)?;

    recursive_print(&value, "");

    todo!("finish reading of metainfo");
}

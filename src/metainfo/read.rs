use std::io::Read;

use crate::bencoding::parse::try_parse_value;

/*
interface IMetaInfo {
    announce: string
    announce-list: string[][]
    azureus_properties: {
        dht_backup_enable: 1 | 0
    },
    privage: 1 | 0,
    "creation date": number // e.g 1608033138
    comment: string,
    info: {
        length: number, // 36_947_471_188
        name: string,
        "piece length": number,
        pieces,
    }

}
*/

// TODO: Improve error handling
pub fn read(
    r: &mut impl std::io::Read,
) -> Result<super::MetaInfo, crate::bencoding::parse::ParseError> {
    let source = r.bytes().take_while(|x| x.is_ok()).map(|x| x.unwrap());
    let value = try_parse_value(source)?;

    println!("announce = {:?}", value["announce"].to_lossy_str().unwrap());
    print!(
        "{}",
        value["announce-list"]
            .values()
            .enumerate()
            .flat_map(|(row, x)| x.values().enumerate().map(move |(col, v)| (row, col, v)))
            .map(|(row, col, v)| format!(
                "announce-list[{}][{}] = {}\n",
                row,
                col,
                v.to_lossy_str().unwrap()
            ))
            .collect::<String>()
    );
    print!(
        "{}",
        value["azureus_properties"]
            .entries()
            .map(|(k, v)| format!(
                "azureus_properties['{}'] = {:?}\n",
                k.to_lossy_str().unwrap(),
                v,
            ))
            .collect::<String>()
    );

    println!("private = {:?}", value["private"]);
    println!("creation date = {:?}", value["creation date"]);
    println!("comment = {:?}", value["comment"].to_lossy_str().unwrap());

    println!(
        "{:?}",
        value["info"]
            .keys()
            .map(|x| x.to_lossy_str().unwrap())
            .collect::<Vec<_>>()
    );

    // info: {
    //     length,
    //     name,
    //     "piece length",
    //     pieces,
    // }
    println!("info.length = {:?}", value["info"]["length"]); // 36_947_471_188
    println!(
        "info.name = {:?}",
        value["info"]["name"].to_lossy_str().unwrap()
    ); // 36_947_471_188
    println!("info['piece length'] = {:?}", value["info"]["piece length"],); // 36_947_471_188
    println!("pieces = {:?}", value["info"]["pieces"].to_lossy_str());

    todo!("finish reading of metainfo");
}

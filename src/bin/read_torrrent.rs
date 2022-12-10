fn main() {
    match std::env::args().skip(1).next() {
        Some(path) => {
            println!("{path:?}");
            let file = std::fs::read(path).unwrap();
            let value =
                torr::bencoding::parse::try_parse_value(&mut file.into_iter().peekable()).unwrap();
            println!("{:?}", value);
        }
        None => {
            println!("You should pass torrent file path as a single parameter");
        }
    };
}

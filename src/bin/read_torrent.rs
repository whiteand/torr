use std::process::exit;

fn main() {
    let path = get_file_path_from_args();
    let metainfo = read_torrent_file(path);
    println!("{:?}", metainfo);
}

fn get_file_path_from_args() -> String {
    match std::env::args().skip(1).next() {
        Some(path) => path,
        None => {
            println!("You should pass torrent file path as a single parameter");
            exit(1);
        }
    }
}

fn read_torrent_file(path: String) -> torr::metainfo::MetaInfo {
    let file = open_file(path);
    read_metainfo_from_file(file)
}

fn read_metainfo_from_file(mut file: std::fs::File) -> torr::metainfo::MetaInfo {
    match torr::metainfo::read::read(&mut file) {
        Ok(metainfo) => metainfo,
        Err(e) => {
            println!("Error: {:?}", e);
            exit(1);
        }
    }
}

fn open_file(path: String) -> std::fs::File {
    match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => exit(e.raw_os_error().unwrap_or(1)),
    }
}

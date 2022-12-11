use std::process::exit;
use torr::bencoding::utils::print_metainfo;

fn main() {
    let path = get_file_path_from_args();
    let mut file = open_file(path);
    print_metainfo(&mut file).unwrap();
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

fn open_file(path: String) -> std::fs::File {
    match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => exit(e.raw_os_error().unwrap_or(1)),
    }
}

use std::fs;
use std::path::PathBuf;


pub fn get_path(filename: &str) -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "testdata", filename]
        .iter()
        .collect()
}

pub fn get_content(filename: &str) -> String {
    let path = get_path(filename);
    fs::read_to_string(path).unwrap()
}

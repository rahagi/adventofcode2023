use std::fs;

pub fn file_to_str(path: &str) -> String {
    fs::read_to_string(path).expect("cant read file")
}

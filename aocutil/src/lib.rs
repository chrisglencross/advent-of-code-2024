use std::fs;

pub mod grid;
pub mod direction;
pub mod coord;

pub fn load_test_input(day: u8) -> String {
    load_file(&format!("day{day}/test_input.txt"))
}

pub fn load_input(day: u8) -> String {
    load_file(&format!("day{day}/input.txt"))
}

fn load_file(filename: &str) -> String {
    let input = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Unable to read file {}", filename));
    if input.is_empty() {
        panic!("File '{filename}' should not be empty");
    }
    input
}


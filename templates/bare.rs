use std::io::prelude::*;
use std::fs::File;
fn main() {
    let mut file = File::open("input/input.txt").expect("Could not open input file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Could not read input file");
    let inputs: Vec<&str> = file_contents.split_whitespace().collect();
}

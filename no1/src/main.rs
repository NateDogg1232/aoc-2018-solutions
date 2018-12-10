use std::fs::File;
use std::io::prelude::*;
fn main() {
    //Open the input file
    let mut file = File::open("input/input.txt").expect("Could not find input file");

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Could not read file");

    let mut cur_freq: i64 = 0;

    for inp in file_contents.split_whitespace() {
        //Go through each one and parse it as an int
        let tmp_freq: i64 = inp.parse().expect("Could not parse input");
        cur_freq += tmp_freq;
    }

    println!("{}", cur_freq);
}

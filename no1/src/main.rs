use std::fs::File;
use std::io::prelude::*;
fn main() {
    //Open the input file
    let mut file = File::open("input/input.txt").expect("Could not find input file");

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Could not read file");

    let mut freq_list = Vec::new();
    let mut cur_freq: i64 = 0;
    loop {
        for inp in file_contents.split_whitespace() {
            let temp_freq: i64 = inp.parse().expect("Could not parse input");
            cur_freq += temp_freq;
            for freq in freq_list.iter() {
                if *freq == cur_freq {
                    println!("{}", freq);
                    return;
                }
            }
            freq_list.push(cur_freq);
        }
    }
}

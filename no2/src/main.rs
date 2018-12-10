use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    let mut file = File::open("input/input.txt").expect("Could not open input file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Could not read from input file");
    let inputs: Vec<&str> = file_contents.split_whitespace().collect();
    let mut char3: usize = 0;
    let mut char2: usize = 0;
    for inp in inputs.iter() {
        let mut chars: HashMap<char, usize> = HashMap::new();
        for c in inp.chars() {
            let num = chars.get(&c).unwrap_or(&0);
            chars.insert(c, num + 1);
        }
        let mut add3 = false;
        let mut add2 = false;
        for num in chars.values() {
            if *num == 3 {
                add3 = true;
            } else if *num == 2 {
                add2 = true;
            }
        }
        if add3 {
            char3 += 1;
        }
        if add2 {
            char2 += 1;
        }
    }
    println!("{}", char2 * char3);
}

use std::fs::File;
use std::io::prelude::*;
fn main() {
    let mut file = File::open("input/input.txt").expect("Could not open input file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Could not read from input file");
    let inputs: Vec<&str> = file_contents.split_whitespace().collect();
    for inp in inputs.iter() {
        println!("inp1: {}", inp);
        for inp2 in inputs.iter() {
            //println!("inp2: {}", inp2);
            let inp1chars: Vec<char> = inp.chars().collect();
            let inp2chars: Vec<char> = inp2.chars().collect();
            let mut diff = 0;
            for ind in 0..inp1chars.len() {
                //println!("{}{}", inp1chars[ind], inp2chars[ind]);
                if inp1chars[ind] != inp2chars[ind] {
                    diff += 1;
                }
            }
            //println!("diff: {}", diff);
            if diff == 1 {
                //Check at what index the character is different and remove it from our string
                let mut new_str: String = String::new();
                for ind in 0..inp1chars.len() {
                    if inp1chars[ind] == inp2chars[ind] {
                        print!("{}", inp1chars[ind]);
                        new_str.push(inp1chars[ind]);
                    }
                }
                println!("");
                println!("{}", new_str);
                return;
            }

        }
    }
    eprintln!("ID not found");
}

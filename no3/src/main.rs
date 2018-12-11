use std::fs::File;
use std::io::prelude::*;
fn main() {
    let mut file = File::open("input/input.txt").expect("Could not open input file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Could not read file");
    let inputs: Vec<&str> = file_contents.split('\n').collect();
    let mut claims: Vec<Claim> = Vec::new();
    for inp in inputs {
        if inp.trim() == "" {
            continue;
        }
        claims.push(parse_claim(inp));
    }
    let mut grid: [[usize; 1000]; 1000] = [[0; 1000]; 1000];
    for claim in claims.iter() {
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                grid[x][y] +=1;
            }
        }
    }
    for claim in claims.iter_mut() {
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                if grid[x][y] > 1 {
                    claim.has_overlapped = true;
                }
            }
        }
    }

    for claim in claims {
        if !claim.has_overlapped {
            println!("{}", claim.id);
        }
    }
    println!("Done");
}

#[derive(Debug)]
struct Claim {
    pub id: usize,
    pub left: usize,
    pub top: usize,
    pub width: usize,
    pub height: usize,
    pub has_overlapped: bool,
}
impl Claim {
    pub fn new() -> Claim {
        Claim {
            id: 0,
            left: 0,
            top: 0,
            width: 0,
            height: 0,
            has_overlapped: false,
        }
    }
}

fn parse_claim(claim: &str) -> Claim {
    let mut tmp_ret = Claim::new();
    //Claim string format: #ID @ TOP,LEFT: WIDTHxHEIGHT
    //Split the ID from the rest
    let tmp_split: Vec<&str> = claim.split('@').collect();
    let id = tmp_split[0].trim();
    let claim = tmp_split[1];
    //Drop our temporary split
    {
        let _ = tmp_split;
    }
    //Finish parsing our ID. Let's remove the # at the front
    let mut id = id.to_owned();
    id.remove(0);
    let id = id.as_str();
    //And now we get our ID from it
    tmp_ret.id = id.parse().expect("Could not parse ID from claim");
    //And then we get our left and top part from it
    let tmp_split: Vec<&str> = claim.split(':').collect();
    let topleft = tmp_split[0];
    let widthheight = tmp_split[1];
    let topleft: Vec<&str> = topleft.split(',').collect();
    let left = topleft[0].trim();
    tmp_ret.left = left.parse().expect("Could not parse left from claim");
    let top = topleft[1].trim();
    tmp_ret.top = top.parse().expect("Could not parse top from claim");
    let widthheight: Vec<&str> = widthheight.split('x').collect();
    let width = widthheight[0].trim();
    tmp_ret.width = width.parse().expect("Could not parse width from claim");
    let height = widthheight[1].trim();
    tmp_ret.height = height.parse().expect("Could not parse height form claim");
    tmp_ret
}

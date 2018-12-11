use chrono::NaiveDateTime;
use std::{fs::File, io::prelude::*};
fn main() {
    let mut file = File::open("input/input.txt").expect("Could not open input file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Could not read input file");
    let inputs: Vec<&str> = file_contents.split('\n').collect();
    let mut log: Vec<LogEntry> = Vec::new();
    //Let's get our times
    for inp in inputs {
        //Parse the input
        if inp == "" {
            continue;
        }
        log.push(LogEntry::new(inp));
    }
    //Sort the log
    let mut log2: Vec<LogEntry> = Vec::new();
    for entry in log {}
}
#[derive(Debug)]
struct LogEntry {
    time: NaiveDateTime,
    action: LogAction,
}
impl LogEntry {
    pub fn new(inp: &str) -> LogEntry {
        //Get our timestamp
        let inp_split: Vec<&str> = inp.split(']').collect();
        //Get rid of the first bracket
        let mut datetimestr = inp_split[0].to_owned();
        datetimestr.remove(0);

        //And now parse our time
        let time: NaiveDateTime =
            NaiveDateTime::parse_from_str(datetimestr.as_str(), "%Y-%m-%d %H:%M")
                .expect("Could not parse time from input");
        //Then parse our action

        LogEntry {
            time,
            action: LogAction::new(inp_split[1].trim()),
        }
    }
}
#[derive(Debug)]
enum LogAction {
    BeginShift(usize),
    WakeUp,
    FallAsleep,
}
impl LogAction {
    pub fn new(inp: &str) -> LogAction {
        match inp {
            "wakes up" => LogAction::WakeUp,
            "falls asleep" => LogAction::FallAsleep,
            _ => {
                //Let's get the guard ID
                let inpsplit: Vec<&str> = inp.split_whitespace().collect();
                let mut id = inpsplit[1].to_owned();
                id.remove(0);
                let id = id.parse().expect("Cannot parse guard ID");
                LogAction::BeginShift(id)
            }
        }
    }
}

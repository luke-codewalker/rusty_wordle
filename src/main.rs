use rusty_wordle::run;
use std::process;

const DICTIONARY: &str = include_str!("./dictionary.txt");

fn main() {
    let dictionary: Vec<&str> = DICTIONARY.lines().collect();
    if let Err(e) = run(dictionary) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

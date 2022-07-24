/// `rusty_wordle` contains an implementation of the Wordle game in Rust. The underlying dictionary and game rules are all based on the [original game
/// from the NY Times](https://www.nytimes.com/games/wordle/index.html)
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

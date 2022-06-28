use colored::Colorize;
use rand::prelude::*;
use regex::Regex;
use rusty_wordle::{Correctness, Game, Guess, State};
use std::io;

const DICTIONARY: &str = include_str!("../dictionary.txt");

fn main() {
    let dictionary: Vec<&str> = DICTIONARY.lines().collect();
    let dict_idx: usize = thread_rng().gen_range(0..dictionary.len());
    let mut game = Game::new(String::from(dictionary[dict_idx]));

    println!("{}", "Welcome to RUSTY_WORDLE!".bold());
    println!("You have six attempts to guess a 5 letter word. Just type it in and press ENTER.");
    println!(
        "Correct letters will be shown as {}, misplaced letters as {} and wrong letters as {}.\n",
        Guess::format("green", &Correctness::Correct),
        Guess::format("yellow", &Correctness::Misplaced),
        Guess::format("crossed out", &Correctness::Wrong)
    );

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim().to_lowercase();

        if !Regex::new("^[a-z]{5}$").unwrap().is_match(&guess) {
            println!("Please make a 5 letter guess with the latin characters a-z only.");
            continue;
        }

        if !dictionary.contains(&guess.as_str()) {
            println!("Word not in dictionary, try again.");
            continue;
        }

        match game.play(guess) {
            Err(err) => {
                eprintln!("{}", err);
                break;
            }
            Ok(history) => {
                let attempts_left = 6 - history.len();
                println!(
                    "{}\t({} attempt{} left)",
                    history.last().unwrap(),
                    attempts_left,
                    if attempts_left == 1 { "" } else { "s" },
                );
                if game.state() == State::Won {
                    println!("Congrats! You've solved this wordle!");
                    break;
                }

                if game.state() == State::Lost {
                    println!("Too bad! You've run out of attempts! The target word was '{}'. Try again soon.", game.target());
                    break;
                }
            }
        };
    }
}

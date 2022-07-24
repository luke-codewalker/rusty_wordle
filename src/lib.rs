use colored::Colorize;
use rand::{thread_rng, Rng};
use std::error::Error;
mod guess;
use std::io;

pub use crate::guess::Guess;
mod correctness;
pub use crate::correctness::Correctness;
mod game;
pub use crate::game::{Game, State};
use crate::utils::validate;

mod utils;

pub fn run(dictionary: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let random_index: usize = thread_rng().gen_range(0..dictionary.len());
    let mut game = Game::new(String::from(dictionary[random_index]))?;

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
        io::stdin().read_line(&mut guess)?;

        let guess = guess.trim().to_lowercase();

        match validate(&guess) {
            Err(err) => {
                println!("{}", err);
                continue;
            }
            _ => (),
        }

        if !dictionary.contains(&guess.as_str()) {
            println!("Word not in dictionary, try again.");
            continue;
        }

        let history = game.play(guess)?;
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
            println!(
                "Too bad! You've run out of attempts! The target word was '{}'. Try again soon.",
                game.target()
            );
            break;
        }
    }

    Ok(())
}

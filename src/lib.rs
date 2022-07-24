use colored::Colorize;
use rand::{thread_rng, Rng};
use std::error::Error;
use std::io;

mod guess;
pub use crate::guess::Guess;
mod correctness;
pub use crate::correctness::Correctness;
mod game;
pub use crate::game::{Game, GameError, State};
mod utils;
use crate::utils::validate;

/// Play wordle via the terminal
///
/// # Arguments
///
/// * `dictionary` - List of 5 letter words from which one will be randomly picked as the target phrase.
///
/// # Examples
///
/// ```no_run
/// rusty_wordle::run(vec!["guess", "wordy", "rusty"]);
/// ```
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
        let mut word = String::new();
        io::stdin().read_line(&mut word)?;

        let word = word.trim().to_lowercase();

        match validate(&word) {
            Err(err) => {
                println!("{}", err);
                continue;
            }
            _ => (),
        }

        if !dictionary.contains(&word.as_str()) {
            println!("Word not in dictionary, try again.");
            continue;
        }

        let guess = game.play(word)?;
        let attempts_left = game.attempts_left();

        println!(
            "{}\t({} attempt{} left)",
            guess,
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

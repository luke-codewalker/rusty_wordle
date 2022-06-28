use std::io;
use wordle;

fn main() {
    let mut game = wordle::Game::new(String::from("hello"));
    println!("Welcome to rusty Wordle. Try to guess the 5 letter word");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim().to_lowercase();

        if guess.len() != 5 {
            println!("Please make a 5 letter guess with the latin characters a-z only");
            continue;
        }

        match game.play(guess) {
            Err(err) => {
                eprintln!("{}", err);
                break;
            }
            Ok(history) => {
                println!("\rAttempt {}: {}", history.len(), history.last().unwrap());
                if game.state() == wordle::State::Won {
                    println!("Congrats! You've solved this wordle!");
                    break;
                }

                if game.state() == wordle::State::Lost {
                    println!("Too bad! You've run out of attempts! Try again soon.");
                    break;
                }
            }
        };
    }
}

use std::fmt::Display;

#[macro_use]
mod macros;

mod guess;
pub use crate::guess::Guess;
mod correctness;
pub use crate::correctness::Correctness;

pub struct Game {
    target: String,
    state: State,
    history: Vec<Guess>,
}

impl Game {
    pub fn new(target: String) -> Self {
        Game {
            target: target,
            state: State::Playing,
            history: vec![],
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn play(&mut self, guess: String) -> Result<&Vec<Guess>, GameError> {
        match self.state {
            State::Lost => Err(GameError::GameOver),
            State::Won => Err(GameError::GameWon),
            State::Playing => {
                // TODO: better error handling instead of unwrap
                let result = correctness::evaluate(&self.target, &guess).unwrap();
                self.history.push(Guess {
                    word: guess,
                    result,
                });

                if Guess::is_winning_guess(&result) {
                    self.state = State::Won;
                } else if self.history.len() == 6 {
                    self.state = State::Lost;
                }

                Ok(&self.history)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameError {
    GameOver,
    GameWon,
}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::GameOver => {
                write!(f, "You've lost this game. Start a new one to keep playing.")
            }
            GameError::GameWon => write!(
                f,
                "You've already won this game. Start a new one to play again."
            ),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum State {
    Playing,
    Won,
    Lost,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_init_works() {
        let game = Game::new(String::from("abcde"));
        assert_eq!(game.state, State::Playing);
        assert_eq!(game.target, String::from("abcde"));
        assert_eq!(game.history.len(), 0);
    }

    #[test]
    fn lost_game_throws_error() {
        let mut lost_game = Game {
            target: String::from(""),
            state: State::Lost,
            history: vec![],
        };
        let result = lost_game.play(String::from("guess"));
        assert_eq!(result.unwrap_err(), GameError::GameOver);
    }

    #[test]
    fn won_game_throws_error() {
        let mut lost_game = Game {
            target: String::from(""),
            state: State::Won,
            history: vec![],
        };
        let result = lost_game.play(String::from("guess"));
        assert_eq!(result.unwrap_err(), GameError::GameWon);
    }

    #[test]
    fn win_in_first_round() {
        let mut game = Game::new(String::from("guess"));
        let guess = game.play(String::from("guess")).unwrap();
        assert_eq!(guess[0].result, result![C C C C C]);
        assert_eq!(game.state, State::Won);
    }

    #[test]
    fn loose() {
        let mut game = Game::new(String::from("guess"));
        for _ in 0..5 {
            let _ = game.play(String::from("xxxxx")).unwrap();
        }
        let guess = game.play(String::from("xuxxg")).unwrap();
        assert_eq!(guess[5].result, result![W C W W M]);
        assert_eq!(game.state, State::Lost);
    }
}

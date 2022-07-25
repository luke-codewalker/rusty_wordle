use crate::{
    correctness,
    utils::{validate, ValidationError},
    Guess,
};
use std::{error::Error, fmt::Display};

/// Struct for holding the logic and state for running a wordle game. Each instance is one round of Wordle for a given target word.
#[derive(Debug)]
pub struct Game {
    target: String,
    state: State,
    history: Vec<Guess>,
    max_attempts: usize,
}

impl Game {
    /// Construct a new game instance for a given `target` word.
    ///
    /// # Arguments
    ///
    /// * `target` - a 5 letter word containing only the letters a-z. Other input will throw a [GameError::InvalidArguments]
    pub fn new(target: String) -> Result<Self, GameError> {
        validate(&target)?;

        Ok(Game {
            target: target,
            state: State::Playing,
            history: vec![],
            max_attempts: 6,
        })
    }

    /// Get the current [State] the game is in.
    pub fn state(&self) -> State {
        self.state
    }

    /// Get a reference to the target word of this game.
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Get the number of attempts the player has left in this game.
    pub fn attempts_left(&self) -> usize {
        self.max_attempts - self.history.len()
    }

    /// Play the game by making a guess at the target word.
    ///
    /// # Arguments
    ///
    /// * `&mut self`
    /// * `word` - a 5 letter word, consisting of letters a-z. Other input will throw a [GameError::InvalidArguments]
    ///
    ///
    /// Try to guess the target word by giving the [Game] a `word`. If the word has a valid format and the player has attempts left
    /// the [Game] will return a [Guess] struct informing the user of the correctness of the guess. If there are no attempts left,
    /// the game was already won or the input was invalid, `play` will return a [GameError].
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let mut game = rusty_wordle::Game::new("guess".to_string())?;
    /// let guess = game.play("gauss".to_string())?;
    /// let attempts_left = game.attempts_left();
    /// println!(
    ///     "{}\t({} attempts left)",
    ///     guess,
    ///     attempts_left,
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn play(&mut self, word: String) -> Result<Guess, GameError> {
        match self.state {
            State::Lost => Err(GameError::GameOver),
            State::Won => Err(GameError::GameWon),
            State::Playing => {
                let result = correctness::evaluate(&self.target, &word)?;
                self.history.push(Guess { word, result });

                if Guess::is_winning_guess(&result) {
                    self.state = State::Won;
                } else if self.history.len() == self.max_attempts {
                    self.state = State::Lost;
                }

                Ok(self.history.last().unwrap().clone())
            }
        }
    }
}

/// Possible error that can happen when constructing or playing a [Game]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameError {
    GameOver,
    GameWon,
    InvalidArguments,
}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::GameOver => {
                write!(f, "You've lost this game. Start a new one to keep playing")
            }
            GameError::GameWon => {
                write!(
                    f,
                    "You've already won this game. Start a new one to play again"
                )
            }
            GameError::InvalidArguments => {
                write!(
                    f,
                    "You've passed invalid arguments to this game. Please only use 5 letters a-z"
                )
            }
        }
    }
}

impl From<ValidationError> for GameError {
    fn from(_: ValidationError) -> Self {
        GameError::InvalidArguments
    }
}

impl Error for GameError {}

/// The state a [Game] can be in
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum State {
    Playing,
    Won,
    Lost,
}

#[cfg(test)]
mod tests {
    use super::*;
    // bring Correctness into scope so macro result! can use it
    use crate::{correctness::Correctness, result};

    #[test]
    fn regular_init_works() {
        let game = Game::new(String::from("abcde")).unwrap();
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
            max_attempts: 6,
        };
        let result = lost_game.play(String::from("guess"));
        assert_eq!(result.unwrap_err(), GameError::GameOver);
    }

    #[test]
    fn won_game_throws_error() {
        let mut won_game = Game {
            target: String::from(""),
            state: State::Won,
            history: vec![],
            max_attempts: 6,
        };
        let result = won_game.play(String::from("guess"));
        assert_eq!(result.unwrap_err(), GameError::GameWon);
    }

    #[test]
    fn incorrect_init_error() {
        assert_eq!(
            Game::new(String::from("")).unwrap_err(),
            GameError::InvalidArguments
        );
    }

    #[test]
    fn incorrect_guess_error() {
        let mut game = Game::new(String::from("abcde")).unwrap();
        let result = game.play(String::from("a"));
        assert_eq!(result.unwrap_err(), GameError::InvalidArguments);
    }

    #[test]
    fn win_in_first_round() {
        let mut game = Game::new(String::from("guess")).unwrap();
        let guess = game.play(String::from("guess")).unwrap();
        assert_eq!(guess.result, result![C C C C C]);
        assert_eq!(game.state, State::Won);
    }

    #[test]
    fn loose() {
        let mut game = Game::new(String::from("guess")).unwrap();
        for _ in 0..5 {
            let _ = game.play(String::from("xxxxx")).unwrap();
        }
        let guess = game.play(String::from("xuxxg")).unwrap();
        assert_eq!(guess.result, result![W C W W M]);
        assert_eq!(game.state, State::Lost);
    }
}

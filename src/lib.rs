use colored::{ColoredString, Colorize};
use std::fmt::Display;
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
                let result = correctness::evaluate(&self.target, &guess);
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

#[derive(Debug)]
pub struct Guess {
    word: String,
    result: [Correctness; 5],
}

impl Guess {
    fn is_winning_guess(result: &[Correctness; 5]) -> bool {
        !result.into_iter().any(|c| *c != Correctness::Correct)
    }

    pub fn format(char: &str, correct: &Correctness) -> ColoredString {
        match correct {
            Correctness::Correct => char.green(),
            Correctness::Misplaced => char.yellow(),
            Correctness::Wrong => char.strikethrough(),
        }
    }
}

impl Display for Guess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            Guess::format(&self.word[..1], &self.result[0]),
            Guess::format(&self.word[1..2], &self.result[1]),
            Guess::format(&self.word[2..3], &self.result[2]),
            Guess::format(&self.word[3..4], &self.result[3]),
            Guess::format(&self.word[4..], &self.result[4])
        )
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

    mod guess {
        use super::*;

        #[test]
        fn is_winning_guess() {
            assert_eq!(Guess::is_winning_guess(&result![C C C C C]), true);
        }

        #[test]
        fn is_not_winning_guess() {
            assert_eq!(Guess::is_winning_guess(&result![C W C C C]), false);
            assert_eq!(Guess::is_winning_guess(&result![C C C M C]), false);
        }
    }

    mod game {
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

    mod correctness {
        use super::*;

        #[test]
        fn all_correct() {
            let result = crate::correctness::evaluate("world", "world");
            assert_eq!(result, [Correctness::Correct; 5]);
        }

        #[test]
        fn all_wrong() {
            let result = crate::correctness::evaluate("abcde", "fghij");
            assert_eq!(result, [Correctness::Wrong; 5]);
        }

        #[test]
        fn all_misplaced() {
            let result = crate::correctness::evaluate("abcde", "eabcd");
            assert_eq!(result, [Correctness::Misplaced; 5]);
        }

        #[test]
        fn some_wrong_others_correct() {
            let result = crate::correctness::evaluate("abcde", "xbxde");
            assert_eq!(result, result![W C W C C]);
        }

        #[test]
        fn first_two_misplaced_others_correct() {
            let result = crate::correctness::evaluate("abcde", "bacde");
            assert_eq!(result, result![M M C C C]);
        }

        #[test]
        fn misplaced_and_correct_once() {
            let result = crate::correctness::evaluate("baabb", "axaxx");
            assert_eq!(result, result![M W C W W]);
        }

        #[test]
        fn same_letter_misplaced_twice() {
            let result = crate::correctness::evaluate("baabb", "axxab");
            assert_eq!(result, result![M W W M C]);
        }

        #[test]
        fn wrong_because_already_used() {
            let result = crate::correctness::evaluate("abcde", "aacde");
            assert_eq!(result, result![C W C C C]);
        }

        #[test]
        fn wrong_because_used_by_other() {
            let result = crate::correctness::evaluate("babbb", "aaccc");
            assert_eq!(result, result![W C W W W]);
        }

        #[test]
        fn only_accepts_length_five() {
            let too_short = std::panic::catch_unwind(|| crate::correctness::evaluate("a", "abcde"));
            assert!(too_short.is_err());
            let too_long =
                std::panic::catch_unwind(|| crate::correctness::evaluate("abcde", "abcdef"));
            assert!(too_long.is_err());
        }
    }
}

#[macro_export]
macro_rules! result {
    (C) => {
        Correctness::Correct
    };
    (M) => {
        Correctness::Misplaced
    };
    (W) => {
        Correctness::Wrong
    };
    ($($item: tt)*) => {
        [
        $(result!($item)),*
        ]
    };
}

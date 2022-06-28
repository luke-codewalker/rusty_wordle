use std::fmt::Display;

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

    pub fn play(&mut self, guess: String) -> Result<&Vec<Guess>, GameError> {
        match self.state {
            State::Lost => Err(GameError::GameOver),
            State::Won => Err(GameError::GameWon),
            State::Playing => {
                let result = evaluate(&self.target, &guess);
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
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum State {
    Playing,
    Won,
    Lost,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Correctness {
    Correct,
    Wrong,
    Misplaced,
}

fn evaluate(target: &str, guess: &str) -> [Correctness; 5] {
    assert_eq!(target.len(), 5);
    assert_eq!(guess.len(), 5);

    let mut result = [Correctness::Wrong; 5];
    // 0 for each character a - z
    let mut missing_chars = [0; (b'z' - b'a' + 1) as usize];

    // find all correct guesses and count missing/possibly misplaced target chars
    for (idx, (g, t)) in guess.bytes().zip(target.bytes()).enumerate() {
        if g == t {
            result[idx] = Correctness::Correct;
        } else {
            missing_chars[(t - b'a') as usize] += 1;
        }
    }

    // check if remaining wrong guesses are actually misplaced
    for (idx, g) in guess.bytes().enumerate() {
        if result[idx] == Correctness::Correct {
            continue;
        }

        if missing_chars[(g - b'a') as usize] > 0 {
            result[idx] = Correctness::Misplaced;
            missing_chars[(g - b'a') as usize] -= 1;
        }
    }

    result
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
            let result = evaluate("world", "world");
            assert_eq!(result, [Correctness::Correct; 5]);
        }

        #[test]
        fn all_wrong() {
            let result = evaluate("abcde", "fghij");
            assert_eq!(result, [Correctness::Wrong; 5]);
        }

        #[test]
        fn all_misplaced() {
            let result = evaluate("abcde", "eabcd");
            assert_eq!(result, [Correctness::Misplaced; 5]);
        }

        #[test]
        fn some_wrong_others_correct() {
            let result = evaluate("abcde", "xbxde");
            assert_eq!(result, result![W C W C C]);
        }

        #[test]
        fn first_two_misplaced_others_correct() {
            let result = evaluate("abcde", "bacde");
            assert_eq!(result, result![M M C C C]);
        }

        #[test]
        fn misplaced_and_correct_once() {
            let result = evaluate("baabb", "axaxx");
            assert_eq!(result, result![M W C W W]);
        }

        #[test]
        fn same_letter_misplaced_twice() {
            let result = evaluate("baabb", "axxab");
            assert_eq!(result, result![M W W M C]);
        }

        #[test]
        fn wrong_because_already_used() {
            let result = evaluate("abcde", "aacde");
            assert_eq!(result, result![C W C C C]);
        }

        #[test]
        fn wrong_because_used_by_other() {
            let result = evaluate("babbb", "aaccc");
            assert_eq!(result, result![W C W W W]);
        }

        #[test]
        fn only_accepts_length_five() {
            let too_short = std::panic::catch_unwind(|| evaluate("a", "abcde"));
            assert!(too_short.is_err());
            let too_long = std::panic::catch_unwind(|| evaluate("abcde", "abcdef"));
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

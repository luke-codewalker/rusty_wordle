use crate::correctness::Correctness;
use colored::{ColoredString, Colorize};
use std::fmt::Display;

#[derive(Debug)]
pub struct Guess {
    pub word: String,
    pub result: [Correctness; 5],
}

impl Guess {
    pub fn is_winning_guess(result: &[Correctness; 5]) -> bool {
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

#[cfg(test)]
mod tests {
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

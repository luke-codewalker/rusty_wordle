struct Game {
    target: String,
    state: State,
}

impl Game {
    fn new(target: String) -> Self {
        Game {
            target: target,
            state: State::Playing,
        }
    }
}

enum State {
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

    for (idx, (t, g)) in target.chars().zip(guess.chars()).enumerate() {
        if t == g {
            result[idx] = Correctness::Correct;
        }
    }

    result
}

#[cfg(test)]
mod tests {
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
    fn wrong_because_already_used() {
        let result = evaluate("abcde", "aacde");
        assert_eq!(result, result![C W C C C]);
    }

    #[test]
    fn misplaced_and_correct_once() {
        let result = evaluate("baabb", "axaxx");
        assert_eq!(result, result![M W C W W]);
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

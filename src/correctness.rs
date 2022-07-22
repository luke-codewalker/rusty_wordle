#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Correctness {
    Correct,
    Wrong,
    Misplaced,
}

pub fn evaluate(target: &str, guess: &str) -> [Correctness; 5] {
    assert_eq!(target.len(), 5);
    assert_eq!(guess.len(), 5);

    let mut result = [Correctness::Wrong; 5];
    // 0 for each character a - z
    let mut unaccounted_target_chars = [0; (b'z' - b'a' + 1) as usize];

    // find all correct guesses and count missing/possibly misplaced target chars
    for (idx, (g, t)) in guess.bytes().zip(target.bytes()).enumerate() {
        if g == t {
            result[idx] = Correctness::Correct;
        } else {
            unaccounted_target_chars[(t - b'a') as usize] += 1;
        }
    }

    // check if remaining wrong guesses are actually misplaced
    for (idx, g) in guess.bytes().enumerate() {
        if result[idx] == Correctness::Correct {
            continue;
        }

        // this char was encountered in the target and not yet correctly guessed
        if unaccounted_target_chars[(g - b'a') as usize] > 0 {
            result[idx] = Correctness::Misplaced;
            unaccounted_target_chars[(g - b'a') as usize] -= 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
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
        let too_long = std::panic::catch_unwind(|| crate::correctness::evaluate("abcde", "abcdef"));
        assert!(too_long.is_err());
    }
}

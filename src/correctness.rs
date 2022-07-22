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

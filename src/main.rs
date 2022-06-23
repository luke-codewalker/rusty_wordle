fn main() {
    let mut game = Game::new(String::from("world"));
    println!("{:?}, {:?}", game.play("wirld"), game.state);
    println!("{:?}, {:?}", game.play("wirld"), game.state);
    println!("{:?}, {:?}", game.play("wirld"), game.state);
    println!("{:?}, {:?}", game.play("world"), game.state);
    println!("{:?}, {:?}", game.play("wirld"), game.state);
    println!("{:?}, {:?}", game.play("wirld"), game.state);
}

#[derive(Clone, Copy, Debug)]
enum Correctness {
    Correct,
    Wrong,
    Misplaced,
}

#[derive(Debug)]
struct Guess {
    word: String,
    correctness: Vec<Correctness>,
}

#[derive(Debug)]
enum GameState {
    Playing,
    Lost,
    Won,
}

#[derive(Debug)]
struct Game {
    target: String,
    history: Vec<Guess>,
    state: GameState,
    round: u8,
}

impl Game {
    fn new(target: String) -> Self {
        Game {
            target: target,
            history: vec![],
            round: 1,
            state: GameState::Playing,
        }
    }

    fn is_winning_guess(guess: &Guess) -> bool {
        0 == guess.correctness.iter().fold(0, |acc, g| {
            acc + match g {
                Correctness::Correct => 0,
                _ => 1,
            }
        })
    }

    fn guess(&mut self, word: &str) -> Guess {
        let correctness: Vec<Correctness> = word
            .chars()
            .zip(self.target.chars())
            .map(|(w, t)| {
                if w == t {
                    return Correctness::Correct;
                } else {
                    if self.target.contains(w) {
                        return Correctness::Misplaced;
                    } else {
                        return Correctness::Wrong;
                    }
                }
            })
            .collect();

        Guess {
            word: word.to_string(),
            correctness,
        }
    }

    pub fn play(&mut self, word: &str) {
        let guess = self.guess(word);

        if Game::is_winning_guess(&guess) {
            self.state = GameState::Won
        } else if self.round == 6 {
            self.state = GameState::Lost
        } else {
            self.state = GameState::Playing
        }

        self.history.push(guess);
        self.round += 1;
    }
}

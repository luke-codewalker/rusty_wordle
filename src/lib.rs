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

use wordle;
fn main() {
    let mut game = wordle::Game::new(String::from("hello"));
    match game.play(String::from("world")) {
        Err(err) => eprintln!("{}", err),
        Ok(history) => println!("{:?}", history),
    };
}

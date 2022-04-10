use wordle_solver::{Guesser, Wordle};

const ANSWERS: &str = include_str!("../answers.txt");

fn main() {
    println!("Hello, World!");
}

fn play<G: Guesser>(maker: impl Fn() -> G) {
    let wordle = Wordle::new();
    for answer in ANSWERS.lines() {
        let guesser = (maker)();
        if let Some(n) = wordle.play(answer, guesser) {
            println!("Solved {} in {} tries!", answer, n)
        } else {
            println!("We couldn't solve {}", answer);
        }
    }
}

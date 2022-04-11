use clap::{ArgEnum, Parser};
use wordle_solver::{Guesser, Wordle};

const ANSWERS: &str = include_str!("../answers.txt");

/// Program to solve wordle games using different strategies.
#[derive(Parser)]
struct Args {
    /// Strategy to solve each game
    #[clap(short, long, arg_enum)]
    strat: Strategy,
    /// The number of games to play.
    ///
    /// If not specified, all possible games are played.
    #[clap(short, long, default_value_t = usize::MAX)]
    games: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Strategy {
    Naive,
}

fn main() {
    let args = Args::parse();
    match args.strat {
        Strategy::Naive => play(wordle_solver::Naive::new, args.games),
    }
}

fn play<G: Guesser>(maker: impl Fn() -> G, n: usize) {
    let wordle = Wordle::new();
    for answer in ANSWERS.lines().take(n) {
        let guesser = (maker)();
        if let Some(n) = wordle.play(answer, guesser) {
            println!("Solved {} in {} tries!", answer, n)
        } else {
            println!("We couldn't solve {}", answer);
        }
    }
}

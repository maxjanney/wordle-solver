use clap::{ArgEnum, Parser};
use wordle_solver::{Guesser, Word, Wordle};

const GAMES: &str = include_str!("../answers.txt");

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
    let mut games = 0;
    let mut score = 0;
    for game in GAMES.lines().take(n) {
        let guesser = (maker)();
        let game: Word = game
            .as_bytes()
            .try_into()
            .expect("every word should be 5 letters");
        if let Some(n) = wordle.play(game, guesser) {
            games += 1;
            score += n;
            println!("Solved {:?} in {} tries!", game, n);
        } else {
            eprintln!("We couldn't solve {:?}", game);
        }
    }
    println!("Average score: {}", score as f64 / games as f64);
}

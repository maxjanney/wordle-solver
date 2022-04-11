pub mod naive;

pub use naive::Naive;

use std::collections::HashSet;

const MAX_TRIES: usize = 32;

pub struct Wordle {
    words: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            words: include_str!("../words.txt")
                .lines()
                .map(|line| line.split_once(' ').expect("word + space + frequency").0)
                .collect(),
        }
    }

    pub fn play<G: Guesser>(&self, answer: &str, mut guesser: G) -> Option<usize> {
        let mut history = Vec::new();
        for i in 0..MAX_TRIES {
            let guess = guesser.guess(&history);
            if guess == answer {
                return Some(i);
            }
            assert!(self.words.contains(&*guess), "Invalid guess: {}", guess);
            let pattern = Cell::calculate_pattern(answer, &guess);
            history.push(Guess {
                word: guess,
                pattern,
            })
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    // Green
    Correct,
    // Yellow
    Misplaced,
    // Gray
    Wrong,
}

impl Cell {
    pub fn calculate_pattern(answer: &str, guess: &str) -> [Self; 5] {
        let answer_bytes = answer.as_bytes();
        let guess_bytes = guess.as_bytes();
        let mut pattern = [Self::Wrong; 5];

        // Check for greens
        for (i, (&a, &g)) in answer_bytes.iter().zip(guess_bytes).enumerate() {
            if a == g {
                pattern[i] = Self::Correct;
            }
        }

        // TODO: Check for yellows

        pattern
    }

    // Return all 243 possible patterns
    pub fn patterns() -> impl Iterator<Item = [Self; 5]> {
        itertools::iproduct!(
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong]
        )
        .map(|(c1, c2, c3, c4, c5)| [c1, c2, c3, c4, c5])
    }
}

pub struct Guess {
    word: String,
    pattern: [Cell; 5],
}

impl Guess {
    pub fn matches(&self, other: &str) -> bool {
        true
    }
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

use std::collections::HashSet;

use crate::{Cell, Guess, Guesser};

// The naive method only considers entropy
// and ignores the relative frequency
pub struct Naive {
    remaining: HashSet<&'static str>,
}

impl Naive {
    pub fn new() -> Self {
        Self {
            remaining: include_str!("../../words.txt")
                .lines()
                .map(|line| line.split_once(' ').expect("word + space + frequency").0)
                .collect(),
        }
    }
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        // retain only the words that match the result of the previous guess
        if let Some(guess) = history.last() {
            self.remaining.retain(|&word| guess.matches(word));
        }

        // "tares" will always be the first guess
        if history.is_empty() {
            return "tares".into();
        }

        // let mut best_guess: Option<&str> = None;
        for &word in self.remaining.iter() {
            let mut entropy = 0.0;
            for pattern in Cell::patterns() {}
        }

        "".into()

        // best_guess.unwrap().into()
    }
}

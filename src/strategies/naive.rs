use std::{borrow::Cow, collections::HashSet};

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

        let mut best_guess: Option<(&str, f64)> = None;
        for &word in self.remaining.iter() {
            let mut sum = 0.0;
            for pattern in Cell::patterns() {
                let g = Guess {
                    word: Cow::Borrowed(word),
                    pattern,
                };
                let p = self
                    .remaining
                    .iter()
                    .filter(|&&word| g.matches(word))
                    .count() as f64
                    / self.remaining.len() as f64;
                sum += p * p.log2();
            }
            let e = -sum;
            if let Some((_, s)) = best_guess {
                if s < e {
                    best_guess = Some((word, e))
                }
            } else {
                best_guess = Some((word, e));
            }
        }
        best_guess.unwrap().0.into()
    }
}

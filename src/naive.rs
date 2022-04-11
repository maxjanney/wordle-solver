use std::collections::HashSet;

use crate::{Guess, Guesser};

// The naive method only considers entropy
// and ignores the relative frequency
pub struct Naive {
    remaining: HashSet<&'static str>,
}

impl Naive {
    pub fn new() -> Self {
        Self {
            remaining: include_str!("../words.txt")
                .lines()
                .map(|line| line.split_once(' ').expect("word + space + frequency").0)
                .collect(),
        }
    }
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        "".into()
    }
}

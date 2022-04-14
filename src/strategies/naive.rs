use std::{borrow::Cow, collections::HashMap};

use crate::{Cell, Guess, Guesser};

pub struct Naive {
    remaining: HashMap<&'static str, usize>,
}

impl Naive {
    pub fn new() -> Self {
        Self {
            remaining: include_str!("../../words.txt")
                .lines()
                .map(|line| {
                    let (word, freq) = line.split_once(' ').expect("word + space + frequency");
                    let freq = freq.parse::<usize>().expect("frequency must be a number");
                    assert!(word.len() == 5);
                    (word, freq)
                })
                .collect(),
        }
    }
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        // retain only the words that match the result of the previous guess
        if let Some(guess) = history.last() {
            self.remaining.retain(|&word, _| guess.matches(word));
        }

        // "tares" will always be the first guess
        if history.is_empty() {
            return "tares".into();
        }

        let remaining_count = self.remaining.iter().map(|(_, &c)| c).sum::<usize>();

        let mut best: Option<(&str, f64)> = None;
        for (&word, _) in &self.remaining {
            let mut sum = 0.0;
            for pattern in Cell::patterns() {
                let mut pattern_total = 0;
                for (&candidate, count) in &self.remaining {
                    let g = Guess {
                        word: Cow::Borrowed(word),
                        pattern,
                    };
                    if g.matches(candidate) {
                        pattern_total += count;
                    }
                }
                if pattern_total == 0 {
                    continue;
                }
                let p = pattern_total as f64 / remaining_count as f64;
                sum += p * p.log2();
            }
            let e = -sum;
            if let Some((_, s)) = best {
                if s < e {
                    best = Some((word, e))
                }
            } else {
                best = Some((word, e));
            }
        }
        best.unwrap().0.into()
    }
}

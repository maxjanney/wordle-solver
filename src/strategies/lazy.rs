use crate::{Cell, Guess, Guesser, Word};

use lazy_static::lazy_static;
use std::borrow::Cow;

lazy_static! {
    static ref INSTANCE: Vec<(&'static Word, usize)> = include_str!("../../words.txt")
        .lines()
        .map(|line| {
            let (word, freq) = line.split_once(' ').expect("word + space + frequency");
            let freq = freq.parse().expect("frequency must be a number");
            let word = word
                .as_bytes()
                .try_into()
                .expect("every word must be 5 characters");
            (word, freq)
        })
        .collect();
}

pub struct Lazy {
    remaining: Cow<'static, Vec<(&'static Word, usize)>>,
}

impl Lazy {
    pub fn new() -> Self {
        Self {
            remaining: Cow::Borrowed(&*INSTANCE),
        }
    }
}

impl Guesser for Lazy {
    fn guess(&mut self, history: &[Guess]) -> Word {
        // retain only the words that match the result of the previous guess
        if let Some(guess) = history.last() {
            if matches!(self.remaining, Cow::Owned(_)) {
                self.remaining
                    .to_mut()
                    .retain(|&(word, _)| guess.matches(*word));
            } else {
                self.remaining = Cow::Owned(
                    self.remaining
                        .iter()
                        .filter(|&&(word, _)| guess.matches(*word))
                        .copied()
                        .collect(),
                );
            }
        }

        // "tares" will always be the first guess
        if history.is_empty() {
            return *b"tares";
        }

        let remaining_count: usize = self.remaining.iter().map(|&(_, c)| c).sum();
        let mut best: Option<(Word, f64)> = None;
        for &(word, _) in &*self.remaining {
            let mut sum = 0.0;
            for pattern in Cell::patterns() {
                let mut pattern_total = 0;
                for &(candidate, count) in &*self.remaining {
                    let g = Guess {
                        word: *word,
                        pattern,
                    };
                    if g.matches(*candidate) {
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
                    best = Some((*word, e))
                }
            } else {
                best = Some((*word, e));
            }
        }
        best.unwrap().0.into()
    }
}

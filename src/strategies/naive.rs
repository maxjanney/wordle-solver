use crate::{Cell, Guess, Guesser, Word};

pub struct Naive {
    remaining: Vec<(Word, usize)>,
}

impl Naive {
    pub fn new() -> Self {
        Self {
            remaining: include_str!("../../words.txt")
                .lines()
                .map(|line| {
                    let (word, freq) = line.split_once(' ').expect("word + space + frequency");
                    let freq = freq.parse::<usize>().expect("frequency must be a number");
                    let word = word
                        .as_bytes()
                        .try_into()
                        .expect("every word must be 5 characters");
                    (word, freq)
                })
                .collect(),
        }
    }
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> Word {
        // retain only the words that match the result of the previous guess
        if let Some(guess) = history.last() {
            self.remaining.retain(|&(word, _)| guess.matches(word));
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
                    let g = Guess { word, pattern };
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

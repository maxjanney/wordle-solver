pub mod strategies;

pub use strategies::Naive;

use std::collections::HashSet;

pub type Word = [u8; 5];

pub struct Wordle {
    words: HashSet<&'static Word>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            words: include_str!("../words.txt")
                .lines()
                .map(|line| {
                    line.split_once(' ')
                        .expect("word + space + frequency")
                        .0
                        .as_bytes()
                        .try_into()
                        .expect("every word must be 5 characters")
                })
                .collect(),
        }
    }

    pub fn play<G: Guesser>(&self, answer: Word, mut guesser: G) -> Option<usize> {
        let mut history = Vec::new();
        for i in 1..=32 {
            let guess = guesser.guess(&history);
            assert!(self.words.contains(&guess),);
            if guess == answer {
                return Some(i);
            }
            let pattern = Cell::calculate_pattern(answer, guess);
            history.push(Guess {
                word: guess,
                pattern,
            })
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    // Green
    Correct,
    // Yellow
    Misplaced,
    // Gray
    Wrong,
}

impl Cell {
    pub fn calculate_pattern(answer: Word, guess: Word) -> [Self; 5] {
        let mut pattern = [Self::Wrong; 5];
        let mut used = [0u8; (b'z' - b'a' + 1) as usize];
        // Add the greens
        for ((&a, g), c) in answer.iter().zip(guess).zip(pattern.iter_mut()) {
            if a == g {
                *c = Self::Correct;
            } else {
                used[(a - b'a') as usize] += 1;
            }
        }
        // Add the yellows
        for (&g, c) in guess.iter().zip(pattern.iter_mut()) {
            if *c == Self::Wrong && used[(g - b'a') as usize] > 0 {
                *c = Self::Misplaced;
                used[(g - b'a') as usize] -= 1;
            }
        }
        pattern
    }

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
    word: Word,
    pattern: [Cell; 5],
}

impl Guess {
    pub fn matches(&self, word: Word) -> bool {
        Cell::calculate_pattern(word, self.word) == self.pattern
    }
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> Word;
}

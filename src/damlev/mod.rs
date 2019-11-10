mod constants;
mod matrix;
mod word;

#[cfg(test)]
mod tests;

use std::fmt;
use std::collections::HashMap;
use matrix::DistanceMatrix;
use word::Word;
use constants::MAX_CHARS;


pub struct DamLev {
    word1: Word,
    word2: Word,
    dists: DistanceMatrix,
    last_i1: HashMap<char, usize>,
}


impl DamLev {
    pub fn new() -> Self {
        let word1 = Word::new();
        let word2 = Word::new();
        let dists = DistanceMatrix::new();
        let last_i1 = HashMap::with_capacity(MAX_CHARS);
        DamLev { word1, word2, dists, last_i1 }
    }

    pub fn set1(&mut self, s: &str) -> &mut Self {
        self.word1.store(s);
        self
    }

    pub fn set2(&mut self, s: &str) -> &mut Self {
        self.word2.store(s);
        self
    }

    pub fn dist(&mut self) -> usize {
        let DamLev { word1, word2, dists, last_i1 } = self;
        last_i1.clear();

        for i1 in 1..word1.len() + 1 {
            let mut l2 = 0;

            for i2 in 1..word2.len() + 1 {
                let l1 = *last_i1.get(&word2[i2 - 1]).unwrap_or(&0);
                let eq = word1[i1 - 1] == word2[i2 - 1];

                let add = dists[(i1 + 1, i2)] + 1;
                let del = dists[(i1, i2 + 1)] + 1;
                let sub = dists[(i1, i2)] + !eq as usize;
                let swp = dists[(l1, l2)] + (i1 - l1 - 1) + (i2 - l2 - 1) + 1;

                dists[(i1 + 1, i2 + 1)] = min4(add, del, sub, swp);
                if eq { l2 = i2; }
            }
            last_i1.insert(word1[i1 - 1], i1);
        }
        dists[(word1.len() + 1, word2.len() + 1)]
    }
}


impl fmt::Display for DamLev {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let DamLev { word1, word2, dists, .. } = self;

        let line = "─".repeat((word1.len() + 1) * 3);

        write!(f, "┌───┬{}┐\n", line)?;
        write!(f, "│   │   ")?;
        for col in 0..word1.len() {
            write!(f, " {} ", word1[col])?;
        }
        write!(f, "│\n")?;
        write!(f, "├───┼{}┤\n", line)?;

        for row in 1..word2.len() + 2 {
            if row <= 1 { write!(f, "│   │")?; }
            if row >= 2 { write!(f, "│ {} │", word2[row - 2])?; }
            for col in 1..word1.len() + 2 {
                write!(f, "{:>2} ", dists[(col, row)].to_string())?;
            }
            write!(f, "│\n")?;
        }

        write!(f, "└───┴{}┘\n", line)?;

        Ok(())
    }
}

fn min4(a: usize, b: usize, c: usize, d: usize) -> usize {
    let mut min = a;
    if b < min { min = b }
    if c < min { min = c }
    if d < min { min = d }
    min
}
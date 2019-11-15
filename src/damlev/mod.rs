#[cfg(test)]
mod tests;

use std::fmt;
use std::collections::HashMap;
use std::cell::RefCell;

const MAX_CHARS: usize = 20;
const MATRIX_SIZE: usize = MAX_CHARS + 2;


pub struct DamLev {
    state: RefCell<(
        Vec<char>,
        Vec<char>,
        Vec<u8>,
        HashMap<char, usize>,
    )>,
}


impl DamLev {
    pub fn new() -> Self {
        let mut dists = vec![0u8; MATRIX_SIZE * MATRIX_SIZE];
        for i in 0..MATRIX_SIZE {
            dists[ix(i, 0)] = MAX_CHARS as u8 * 2;
            dists[ix(0, i)] = MAX_CHARS as u8 * 2;
            if i == 0 { continue; }
            dists[ix(i, 1)] = i as u8 - 1;
            dists[ix(1, i)] = i as u8 - 1;
        }
        let word1 = Vec::with_capacity(MAX_CHARS);
        let word2 = Vec::with_capacity(MAX_CHARS);
        let last_i1 = HashMap::with_capacity(MAX_CHARS);
        let state = RefCell::new((word1, word2, dists, last_i1));
        DamLev { state }
    }

    pub fn dist(&self, s1: &str, s2: &str) -> usize {
        self.build_matrix(s1, s2);
        let (word1, word2, dists, ..) = &*self.state.borrow();
        let len1 = word1.len();
        let len2 = word2.len();
        dists[ix(len1 + 1, len2 + 1)] as usize
    }

    fn build_matrix(&self, s1: &str, s2: &str) -> () {
        let (word1, word2, dists, last_i1) = &mut *self.state.borrow_mut();

        last_i1.clear();
        word1.clear();
        word2.clear();
        for c in s1.chars().take(MAX_CHARS) { word1.push(c); }
        for c in s2.chars().take(MAX_CHARS) { word2.push(c); }

        for i1 in 0..word1.len() {
            let char1 = word1[i1];
            let mut l2 = 0;

            for i2 in 0..word2.len() {
                let char2 = word2[i2];
                let l1 = *last_i1.get(&char2).unwrap_or(&0);

                dists[ix(i1 + 2, i2 + 2)] = min4(
                    dists[ix(i1 + 2, i2 + 1)] + 1,
                    dists[ix(i1 + 1, i2 + 2)] + 1,
                    dists[ix(i1 + 1, i2 + 1)] + (char1 != char2) as u8,
                    dists[ix(l1, l2)] + (i1 - l1) as u8 + (i2 - l2) as u8 + 1,
                );

                if char1 == char2 { l2 = i2 + 1; }
            }
            last_i1.insert(char1, i1 + 1);
        }
    }
}


#[inline]
fn min4(a: u8, b: u8, c: u8, d: u8) -> u8 {
    let mut min = a;
    if b < min { min = b; }
    if c < min { min = c; }
    if d < min { min = d; }
    min
}


#[inline]
fn ix(i: usize, j: usize) -> usize {
    i * MATRIX_SIZE + j
}


impl fmt::Display for DamLev {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (word1, word2, dists, ..) = &*self.state.borrow();
        let line = "─".repeat((word1.len() + 1) * 3);

        // header
        write!(f, "┌───┬{}┐\n", line)?;
        write!(f, "│   │   ")?;
        for char1 in word1 {
            write!(f, " {} ", char1)?;
        }
        write!(f, "│\n")?;
        write!(f, "├───┼{}┤\n", line)?;

        // first row
        write!(f, "│   │")?;
        for col in 1..word1.len() + 2 {
            write!(f, "{:>2} ", dists[ix(col, 1)].to_string())?;
        }
        write!(f, "│\n")?;

        // rest rows
        for row in 0..word2.len() {
            let char2 = word2[row];
            write!(f, "│ {} │", char2)?;
            for col in 1..word1.len() + 2 {
                write!(f, "{:>2} ", dists[ix(col, row + 2)].to_string())?;
            }
            write!(f, "│\n")?;
        }

        // footer
        write!(f, "└───┴{}┘\n", line)?;

        Ok(())
    }
}

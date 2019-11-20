mod matrix;

#[cfg(test)]
mod tests;

use std::cmp::max;
use std::collections::BTreeMap;
use std::cell::RefCell;
use crate::utils::write_str;
use matrix::Matrix;

const DEFAULT_CAPACITY: usize = 20;


struct State {
    word1:   Vec<char>,
    word2:   Vec<char>,
    dists:   Matrix,
    last_i1: BTreeMap<char, usize>,
}


pub struct DamerauLevenshtein {
    state: RefCell<State>,
}


impl DamerauLevenshtein {
    pub fn new() -> Self {
        let dists = Matrix::new(DEFAULT_CAPACITY + 2);
        let word1 = Vec::with_capacity(DEFAULT_CAPACITY);
        let word2 = Vec::with_capacity(DEFAULT_CAPACITY);
        let last_i1 = BTreeMap::new();
        let state = State { word1, word2, dists, last_i1 };
        DamerauLevenshtein { state: RefCell::new(state) }
    }

    pub fn distance(&self, s1: &str, s2: &str) -> usize {
        let State { word1, word2, dists, last_i1 } = &mut *self.state.borrow_mut();

        last_i1.clear();

        write_str(s1, word1);
        write_str(s2, word2);
        let len1 = word1.len();
        let len2 = word2.len();

        dists.grow(max(len1 + 2, len2 + 2));

        for (i1, &char1) in word1.iter().enumerate() {
            let mut l2 = 0;

            for (i2, &char2) in word2.iter().enumerate() {
                let l1 = *last_i1.get(&char2).unwrap_or(&0);

                unsafe {
                    *dists.ix(i1 + 2, i2 + 2) = min!(
                        *dists.ix(i1 + 2, i2 + 1) + 1,
                        *dists.ix(i1 + 1, i2 + 2) + 1,
                        *dists.ix(i1 + 1, i2 + 1) + (char1 != char2) as u8,
                        *dists.ix(l1, l2) + (i1 - l1) as u8 + (i2 - l2) as u8 + 1
                    );
                }

                if char1 == char2 { l2 = i2 + 1; }
            }
            last_i1.insert(char1, i1 + 1);
        }

        let dist = unsafe { *dists.ix(len1 + 1, len2 + 1) };
        dist as usize
    }

    pub fn rel_dist(&self, str1: &str, str2: &str) -> f64 {
        let dist = self.distance(str1, str2);
        let State { word1, word2, .. } = &*self.state.borrow_mut();
        let len = max!(1, word1.len(), word2.len());
        dist as f64 / len as f64
    }

    pub fn similarity(&self, str1: &str, str2: &str) -> f64 {
        1.0 - self.rel_dist(str1, str2)
    }
}

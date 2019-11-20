#[cfg(test)]
mod tests;

use std::cmp::min;
use std::cell::RefCell;
use crate::utils::write_str;


const DEFAULT_CAPATITY: usize = 25;


pub struct State {
    pub word1:    Vec<char>,
    pub word2:    Vec<char>,
    pub matches1: Vec<bool>,
    pub matches2: Vec<bool>,
}


pub struct Jaro {
    pub state: RefCell<State>,
}


impl Jaro {
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPATITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Jaro {
            state: RefCell::new(State {
                word1:    Vec::with_capacity(capacity),
                word2:    Vec::with_capacity(capacity),
                matches1: Vec::with_capacity(capacity),
                matches2: Vec::with_capacity(capacity),
            })
        }
    }

    pub fn similarity(&self, str1: &str, str2: &str) -> f64 {
        match (str1.len(), str2.len()) {
            (0, 0) => { return 1.0; }
            (_, 0) => { return 0.0; }
            (0, _) => { return 0.0; }
            (_, _) => { }
        }

        let State {
            word1,
            word2,
            matches1,
            matches2,
        } = &mut *self.state.borrow_mut();

        write_str(str1, word1);
        write_str(str2, word2);
        matches1.clear();
        matches2.clear();
        matches1.resize(word1.len(), false);
        matches2.resize(word2.len(), false);

        let mut matches = 0;
        let mut trans = 0;
        let len1 = word1.len();
        let len2 = word2.len();
        let i2_range = max!(1, len1 / 2, len2 / 2) - 1;

        for i1 in 0..len1 {
            let i2_lower = i1 - min(i2_range, i1);
            let i2_upper = min(i1 + i2_range + 1, len2);

            for i2 in i2_lower..i2_upper {
                unsafe {
                    let char1 = word1.get_unchecked(i1);
                    let char2 = word2.get_unchecked(i2);
                    let match2 = matches2.get_unchecked_mut(i2);
                    if !*match2 && char1 == char2 {
                        let match1 = matches1.get_unchecked_mut(i1);
                        *match1 = true;
                        *match2 = true;
                        matches += 1;
                        break;
                    }
                }
            }
        }

        if matches == 0 { return 0.0; }

        let mut i2 = 0;
        for i1 in 0..len1 {
            unsafe {
                if !*matches1.get_unchecked(i1) { continue; }
                while !*matches2.get_unchecked(i2) { i2 += 1; }
                let char1 = word1.get_unchecked(i1);
                let char2 = word2.get_unchecked(i2);
                if char1 != char2 { trans += 1; }
                i2 += 1;
            }
        }

        let matches = matches as f64;
        let trans = trans as f64;
        let len1 = len1 as f64;
        let len2 = len2 as f64;

        (matches/len1 + matches/len2 + ((matches - trans/2.) / matches)) / 3.
    }

    pub fn rel_dist(&self, str1: &str, str2: &str) -> f64 {
        1.0 - self.similarity(str1, str2)
    }
}

#[cfg(test)]
mod tests;

use std::cmp::{min, max};
use std::cell::RefCell;

const MAX_CHARS: usize = 20;


pub struct Jaro {
    state: RefCell<State>,
}


struct State {
    m:        usize,
    t:        usize,
    word1:    Vec<char>,
    word2:    Vec<char>,
    matches1: Vec<bool>,
    matches2: Vec<bool>,
}


impl Jaro {
    pub fn new() -> Self {
        let word1 = Vec::with_capacity(MAX_CHARS);
        let word2 = Vec::with_capacity(MAX_CHARS);
        let matches1 = vec![false; MAX_CHARS];
        let matches2 = vec![false; MAX_CHARS];
        let m = 0;
        let t = 0;
        let state = State { m, t, word1, word2, matches1, matches2 };
        Jaro { state: RefCell::new(state) }
    }

    pub fn dist(&self, str1: &str, str2: &str) -> f64 {
        let state = &mut *self.state.borrow_mut();
        let State { m, t, word1, word2, matches1, matches2 } = state;

        *m = 0usize; // matches
        *t = 0usize; // transpositions

        word1.clear();
        word2.clear();
        for c in str1.chars().take(MAX_CHARS) { word1.push(c); }
        for c in str2.chars().take(MAX_CHARS) { word2.push(c); }

        let len1 = word1.len();
        let len2 = word2.len();

        match (len1, len2) {
            (0, 0) => { return 1.0; }
            (_, 0) => { return 0.0; }
            (0, _) => { return 0.0; }
            (_, _) => { }
        }

        for i in 0..len1 { matches1[i] = false; }
        for i in 0..len2 { matches2[i] = false; }

        let range = max3(1, len1 / 2, len2 / 2) - 1;

        for i1 in 0..len1 {
            let i2_lower = if i1 > range { i1 - range } else { 0 };
            let i2_upper = min(i1 + range + 1, len2);

            for i2 in i2_lower..i2_upper {
                if !matches2[i2] && word1[i1] == word2[i2] {
                    matches1[i1] = true;
                    matches2[i2] = true;
                    *m += 1;
                    break;
                }
            }
        }

        if *m == 0 { return 0.0; }

        let mut i2 = 0usize;
        for i1 in 0..len1 {
            if !matches1[i1] { continue; }
            while !matches2[i2] { i2 += 1; }
            if word1[i1] != word2[i2]  { *t += 1; }
            i2 += 1;
        }

        let m = *m as f64;
        let t = *t as f64;
        let len1 = len1 as f64;
        let len2 = len2 as f64;

        (m/len1 + m/len2 + ((m - t/2.) / m)) / 3.
    }
}


#[inline]
fn max3(x1: usize, x2: usize, x3: usize) -> usize {
    max(max(x1, x2), x3)
}

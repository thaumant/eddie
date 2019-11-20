#[cfg(test)]
mod tests;

use std::cell::RefCell;
use crate::utils::write_str;

const DEFAULT_CAPACITY: usize = 20;


pub struct Levenshtein {
    state: RefCell<State>,
}


struct State {
    len1: usize,
    word2: Vec<char>,
    cache: Vec<u8>,
}


impl Levenshtein {
    pub fn new() -> Self {
        let len1 = 0;
        let word2 = Vec::with_capacity(DEFAULT_CAPACITY);
        let cache = Vec::with_capacity(DEFAULT_CAPACITY + 1);
        let state = State { len1, word2, cache };
        Levenshtein { state: RefCell::new(state) }
    }

    pub fn distance(&self, str1: &str, str2: &str) -> usize {
        let State { len1, word2, cache, .. } = &mut *self.state.borrow_mut();

        *len1 = 0;
        write_str(str2, word2);
        cache.clear();
        for i in 0..word2.len() + 1 { cache.push(i as u8 + 1); }

        let mut dist = word2.len() as u8;
        let mut prev;
        for (i1, char1) in str1.chars().enumerate() {
            *len1 = i1 + 1;
            dist = i1 as u8 + 1;
            prev = i1 as u8;
            for (i2, &char2) in word2.iter().enumerate() {
                dist = min!(
                    dist + 1,
                    cache[i2] + 1,
                    prev + (char1 != char2) as u8
                );
                prev = cache[i2];
                cache[i2] = dist;
            }
        }

        dist as usize
    }

    pub fn rel_dist(&self, str1: &str, str2: &str) -> f64 {
        let dist = self.distance(str1, str2);
        let State { len1, word2, .. } = &*self.state.borrow_mut();
        let len = max!(1, *len1, word2.len());
        dist as f64 / len as f64
    }

    pub fn similarity(&self, str1: &str, str2: &str) -> f64 {
        1.0 - self.rel_dist(str1, str2)
    }
}

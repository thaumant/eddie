#[cfg(test)]
mod tests;

use std::cell::RefCell;
use crate::utils::write_str;

const DEFAULT_CAPACITY: usize = 20;


struct State {
    word2: Vec<char>,
    cache: Vec<u8>,
}


pub struct Levenshtein {
    state: RefCell<State>,
}


impl Levenshtein {
    pub fn new() -> Self {
        let word2 = Vec::with_capacity(DEFAULT_CAPACITY);
        let cache = Vec::with_capacity(DEFAULT_CAPACITY + 1);
        let state = State { word2, cache };
        Levenshtein { state: RefCell::new(state) }
    }

    pub fn dist(&self, str1: &str, str2: &str) -> usize {
        let State { word2, cache, .. } = &mut *self.state.borrow_mut();

        write_str(str2, word2);
        cache.clear();
        for i in 0..word2.len() + 1 { cache.push(i as u8 + 1); }

        let mut dist = word2.len() as u8;
        let mut prev;
        for (i1, char1) in str1.chars().enumerate() {
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
}

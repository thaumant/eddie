#[cfg(test)]
mod tests;

use std::cmp;
use std::cell::RefCell;

const MAX_CHARS: usize = 20;
const BUFFER_SIZE: usize = MAX_CHARS + 1;


pub struct Leven {
    state: RefCell<(
        Vec<char>,
        Vec<char>,
        Vec<u8>,
    )>,
}


impl Leven {
    pub fn new() -> Self {
        let cache = vec![0u8; BUFFER_SIZE];
        let word1 = Vec::with_capacity(MAX_CHARS);
        let word2 = Vec::with_capacity(MAX_CHARS);
        let state = RefCell::new((word1, word2, cache));
        Leven { state }
    }

    pub fn dist(&self, s1: &str, s2: &str) -> usize {
        let (word1, word2, cache) = &mut *self.state.borrow_mut();

        word1.clear();
        word2.clear();

        for c in s1.chars().take(MAX_CHARS) {
            word1.push(c);
        }
        for (i, c) in s2.chars().take(MAX_CHARS).enumerate() {
            cache[i] = i as u8 + 1;
            word2.push(c);
        }
        cache[word2.len()] = word2.len() as u8 + 1;

        let mut dist = cmp::max(word1.len(), word2.len()) as u8;
        let mut prev;

        for i1 in 0..word1.len() {
            dist = i1 as u8 + 1;
            prev = i1 as u8;

            for i2 in 0..word2.len() {
                dist = min3(
                    dist + 1,
                    cache[i2] + 1,
                    prev + (word1[i1] != word2[i2]) as u8,
                );
                prev = cache[i2];
                cache[i2] = dist;
            }
        }

        dist as usize
    }
}


#[inline]
fn min3(a: u8, b: u8, c: u8) -> u8 {
    let mut min = a;
    if b < min { min = b; }
    if c < min { min = c; }
    min
}

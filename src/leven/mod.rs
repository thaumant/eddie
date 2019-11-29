#[cfg(test)]
mod tests;

use std::cell::RefCell;
use crate::utils::{Rewrite, common_affix_sizes};

const DEFAULT_CAPACITY: usize = 20;

/// # Levenshtein distance.
///
/// See [the detailed description][1].
///
/// [1]: https://en.wikipedia.org/wiki/Levenshtein_distance
///
/// # Usage
///
/// ```rust
/// use eddie::Levenshtein;
///
/// let lev = Levenshtein::new();
/// let dist = lev.distance("martha", "marhta");
/// assert_eq!(dist, 2);
/// ```
///
/// # Complementary metrics
///
/// Relative distance:
/// ```rust
/// # use std::cmp::max;
/// # let lev = eddie::Levenshtein::new();
/// # let s1 = "martha";
/// # let s2 = "marhta";
/// let dist = lev.distance(s1, s2);
/// let rel = lev.rel_dist(s1, s2);
/// let max_len = max(s1.len(), s2.len());
/// assert_eq!(rel, dist as f64 / max_len as f64);
/// ```
///
/// Similarity:
/// ```rust
/// # use std::cmp::max;
/// # let lev = eddie::Levenshtein::new();
/// # let s1 = "martha";
/// # let s2 = "marhta";
/// let rel = lev.rel_dist(s1, s2);
/// let sim = lev.similarity(s1, s2);
/// assert_eq!(sim, 1.0 - rel);
/// ```
pub struct Levenshtein {
    state: RefCell<State>,
}


struct State {
    word1: Vec<char>,
    word2: Vec<char>,
    dists: Vec<u8>,
}


impl Levenshtein {
    /// Creates a new instance of Levenshtein struct with an internal state
    /// for the metric methods to reuse.
    ///
    /// # Example
    ///
    /// ```rust
    /// use eddie::Levenshtein;
    ///
    /// let lev = Levenshtein::new();
    /// ```
    pub fn new() -> Self {
        let word1 = Vec::with_capacity(DEFAULT_CAPACITY);
        let word2 = Vec::with_capacity(DEFAULT_CAPACITY);
        let dists = Vec::with_capacity(DEFAULT_CAPACITY + 1);
        let state = State { word1, word2, dists };
        Levenshtein { state: RefCell::new(state) }
    }

    /// Distance metric. Returns a number of edits
    /// (character additions, deletions, and substitutions)
    /// required to transform one string into the other.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::Levenshtein;
    /// # let lev = Levenshtein::new();
    /// let dist = lev.distance("martha", "marhta");
    /// assert_eq!(dist, 2);
    /// ```
    pub fn distance(&self, str1: &str, str2: &str) -> usize {
        let state = &mut *self.state.borrow_mut();
        let State { word1, word2, dists } = state;

        word1.rewrite_with(str1.chars());
        word2.rewrite_with(str2.chars());
        dists.rewrite_with(1 .. word2.len() as u8 + 2);

        let (prefix, postfix) = common_affix_sizes(word1, word2);
        let word1 = { let l = word1.len(); &word1[prefix .. l - postfix] };
        let word2 = { let l = word2.len(); &word2[prefix .. l - postfix] };

        let mut dist = word2.len() as u8;
        let mut prev;

        for i1 in 0..word1.len() {
            let char1 = unsafe { *word1.get_unchecked(i1) };
            dist = i1 as u8 + 1;
            prev = i1 as u8;

            for i2 in 0..word2.len() {
                unsafe {
                    let char2 = *word2.get_unchecked(i2);
                    let prev2 = dists.get_unchecked_mut(i2);
                    dist = min!(
                        dist + 1,
                        *prev2 + 1,
                        prev + (char1 != char2) as u8
                    );
                    prev = *prev2;
                    *prev2 = dist;
                }
            }
        }

        dist as usize
    }

    /// Relative distance metric. Returns a number of edits relative to the length of
    /// the longest string, ranging from 0.0 (equality) to 1.0 (nothing in common).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::Levenshtein;
    /// # let lev = Levenshtein::new();
    /// let dist = lev.rel_dist("martha", "marhta");
    /// assert!((dist - 0.333).abs() < 0.001);
    /// ```
    pub fn rel_dist(&self, str1: &str, str2: &str) -> f64 {
        let dist = self.distance(str1, str2);
        let State { word1, word2, .. } = &*self.state.borrow_mut();
        let len = max!(1, word1.len(), word2.len());
        dist as f64 / len as f64
    }

    /// Similarity metric. Inversion of relative distance,
    /// ranging from 1.0 (equality) to 0.0 (nothing in common).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::Levenshtein;
    /// # let lev = Levenshtein::new();
    /// let sim = lev.similarity("martha", "marhta");
    /// assert!((sim - 0.666).abs() < 0.001);
    /// ```
    pub fn similarity(&self, str1: &str, str2: &str) -> f64 {
        1.0 - self.rel_dist(str1, str2)
    }
}

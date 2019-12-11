#[cfg(test)]
mod tests;

use std::cell::RefCell;
use crate::utils::Rewrite;
use crate::slice;


const DEFAULT_CAPACITY: usize = 20;


/// # Damerau-Levenshtein distance.
///
/// See [the detailed description][1].
///
/// [1]: https://en.wikipedia.org/wiki/Damerau–Levenshtein_distance
///
/// # Usage
///
/// ```rust
/// use eddie::DamerauLevenshtein;
///
/// let damlev = DamerauLevenshtein::new();
/// let dist = damlev.distance("martha", "marhta");
/// assert_eq!(dist, 1);
/// ```
///
/// # Complementary metrics
///
/// Relative distance:
/// ```rust
/// # use std::cmp::max;
/// # let damlev = eddie::DamerauLevenshtein::new();
/// # let s1 = "martha";
/// # let s2 = "marhta";
/// let dist = damlev.distance(s1, s2);
/// let rel = damlev.rel_dist(s1, s2);
/// let max_len = max(s1.len(), s2.len());
/// assert_eq!(rel, dist as f64 / max_len as f64);
/// ```
///
/// Similarity:
/// ```rust
/// # use std::cmp::max;
/// # let damlev = eddie::DamerauLevenshtein::new();
/// # let s1 = "martha";
/// # let s2 = "marhta";
/// let rel = damlev.rel_dist(s1, s2);
/// let sim = damlev.similarity(s1, s2);
/// assert_eq!(sim, 1.0 - rel);
/// ```
pub struct DamerauLevenshtein {
    internal: slice::DamerauLevenshtein<char>,
    word1: RefCell<Vec<char>>,
    word2: RefCell<Vec<char>>,
}


impl DamerauLevenshtein {
    /// Creates a new instance of DamerauLevenshtein struct with
    /// an internal state for the metric methods to reuse.
    ///
    /// # Example
    ///
    /// ```rust
    /// use eddie::DamerauLevenshtein;
    ///
    /// let damlev = DamerauLevenshtein::new();
    /// ```
    pub fn new() -> Self {
        let internal = slice::DamerauLevenshtein::new();
        let word1 = RefCell::new(Vec::with_capacity(DEFAULT_CAPACITY));
        let word2 = RefCell::new(Vec::with_capacity(DEFAULT_CAPACITY));
        Self { internal, word1, word2 }
    }

    /// Distance metric. Returns a number of edits
    /// (character additions, deletions, substitutions, and transpositions)
    /// required to transform one string into the other.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::DamerauLevenshtein;
    /// # let damlev = DamerauLevenshtein::new();
    /// let dist = damlev.distance("martha", "marhta");
    /// assert_eq!(dist, 1);
    /// ```
    pub fn distance(&self, str1: &str, str2: &str) -> usize {
        let word1 = &mut *self.word1.borrow_mut();
        let word2 = &mut *self.word2.borrow_mut();
        word1.rewrite_with(str1.chars());
        word2.rewrite_with(str2.chars());
        self.internal.distance(word1, word2)
    }

    /// Relative distance metric. Returns a number of edits relative to the length of
    /// the longest string, ranging from 0.0 (equality) to 1.0 (nothing in common).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::DamerauLevenshtein;
    /// # let damlev = DamerauLevenshtein::new();
    /// let dist = damlev.rel_dist("martha", "marhta");
    /// assert!((dist - 0.167).abs() < 0.001);
    /// ```
    pub fn rel_dist(&self, str1: &str, str2: &str) -> f64 {
        let word1 = &mut *self.word1.borrow_mut();
        let word2 = &mut *self.word2.borrow_mut();
        word1.rewrite_with(str1.chars());
        word2.rewrite_with(str2.chars());
        self.internal.rel_dist(word1, word2)
    }

    /// Similarity metric. Inversion of relative distance,
    /// ranging from 1.0 (equality) to 0.0 (nothing in common).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::DamerauLevenshtein;
    /// # let damlev = DamerauLevenshtein::new();
    /// let sim = damlev.similarity("martha", "marhta");
    /// assert!((sim - 0.833).abs() < 0.001);
    /// ```
    pub fn similarity(&self, str1: &str, str2: &str) -> f64 {
        let word1 = &mut *self.word1.borrow_mut();
        let word2 = &mut *self.word2.borrow_mut();
        word1.rewrite_with(str1.chars());
        word2.rewrite_with(str2.chars());
        self.internal.similarity(word1, word2)
    }
}

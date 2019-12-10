#[cfg(test)]
mod tests;

use std::cell::RefCell;
use crate::utils::Rewrite;
use crate::slice;

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
    internal: slice::Levenshtein,
    buffer1: RefCell<Vec<char>>,
    buffer2: RefCell<Vec<char>>,
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
    /// let lev: Levenshtein = Levenshtein::new();
    /// ```
    pub fn new() -> Self {
        let internal = slice::Levenshtein::new();
        let buffer1 = RefCell::new(Vec::with_capacity(DEFAULT_CAPACITY));
        let buffer2 = RefCell::new(Vec::with_capacity(DEFAULT_CAPACITY));
        Self { internal, buffer1, buffer2 }
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
        let buffer1 = &mut *self.buffer1.borrow_mut();
        let buffer2 = &mut *self.buffer2.borrow_mut();
        buffer1.rewrite_with(str1.chars());
        buffer2.rewrite_with(str2.chars());
        self.internal.distance(buffer1, buffer2)
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
        let buffer1 = &mut *self.buffer1.borrow_mut();
        let buffer2 = &mut *self.buffer2.borrow_mut();
        buffer1.rewrite_with(str1.chars());
        buffer2.rewrite_with(str2.chars());
        self.internal.rel_dist(buffer1, buffer2)
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
        let buffer1 = &mut *self.buffer1.borrow_mut();
        let buffer2 = &mut *self.buffer2.borrow_mut();
        buffer1.rewrite_with(str1.chars());
        buffer2.rewrite_with(str2.chars());
        self.internal.similarity(buffer1, buffer2)
    }
}

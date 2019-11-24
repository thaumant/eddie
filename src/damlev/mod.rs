mod matrix;

#[cfg(test)]
mod tests;

use std::cmp::max;
use std::collections::BTreeMap;
use std::cell::RefCell;
use crate::utils::Rewrite;
use matrix::Matrix;

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
    state: RefCell<State>,
}


struct State {
    word1:   Vec<char>,
    word2:   Vec<char>,
    dists:   Matrix,
    last_i1: BTreeMap<char, usize>,
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
        let dists = Matrix::new(DEFAULT_CAPACITY + 2);
        let word1 = Vec::with_capacity(DEFAULT_CAPACITY);
        let word2 = Vec::with_capacity(DEFAULT_CAPACITY);
        let last_i1 = BTreeMap::new();
        let state = State { word1, word2, dists, last_i1 };
        DamerauLevenshtein { state: RefCell::new(state) }
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
    pub fn distance(&self, s1: &str, s2: &str) -> usize {
        let State { word1, word2, dists, last_i1 } = &mut *self.state.borrow_mut();

        last_i1.clear();

        word1.rewrite_with(s1.chars());
        word2.rewrite_with(s2.chars());
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
    /// # use eddie::DamerauLevenshtein;
    /// # let damlev = DamerauLevenshtein::new();
    /// let sim = damlev.similarity("martha", "marhta");
    /// assert!((sim - 0.833).abs() < 0.001);
    /// ```
    pub fn similarity(&self, str1: &str, str2: &str) -> f64 {
        1.0 - self.rel_dist(str1, str2)
    }
}

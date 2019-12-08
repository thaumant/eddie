#[cfg(test)]
mod tests;

use std::cell::RefCell;
use crate::utils::{Chars, Rewrite, common_affix_sizes};

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
pub struct Levenshtein<T: PartialEq> {
    state: RefCell<State<T>>,
}


struct State<T: PartialEq> {
    buffer1: Vec<T>,
    buffer2: Vec<T>,
    dists: Vec<u8>,
}


impl<T: PartialEq> Levenshtein<T> {
    /// Creates a new instance of Levenshtein struct with an internal state
    /// for the metric methods to reuse.
    ///
    /// # Example
    ///
    /// ```rust
    /// use eddie::Levenshtein;
    ///
    /// let lev: Levenshtein<char> = Levenshtein::new();
    /// ```
    pub fn new() -> Self {
        let buffer1 = Vec::with_capacity(DEFAULT_CAPACITY);
        let buffer2 = Vec::with_capacity(DEFAULT_CAPACITY);
        let dists = Vec::with_capacity(DEFAULT_CAPACITY + 1);
        let state = State { buffer1, buffer2, dists };
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
    pub fn distance<C: Chars<T>>(&self, chars1: C, chars2: C) -> usize {
        let state = &mut *self.state.borrow_mut();
        let State { buffer1, buffer2, dists } = state;

        chars1.copy_to(buffer1);
        chars2.copy_to(buffer2);
        dists.rewrite_with(1 .. buffer2.len() as u8 + 2);

        let (prefix, postfix) = common_affix_sizes(buffer1, buffer2);
        let buffer1 = { let l = buffer1.len(); &buffer1[prefix .. l - postfix] };
        let buffer2 = { let l = buffer2.len(); &buffer2[prefix .. l - postfix] };

        let mut dist = buffer2.len() as u8;
        let mut prev;

        for (i1, char1) in buffer1.into_iter().enumerate() {
            dist = i1 as u8 + 1;
            prev = i1 as u8;

            for (char2, prev2) in buffer2.into_iter().zip(dists.into_iter()) {
                dist = min!(
                    dist + 1,
                    *prev2 + 1,
                    prev + (char1 != char2) as u8
                );
                prev = *prev2;
                *prev2 = dist;
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
    pub fn rel_dist<C: Chars<T>>(&self, chars1: C, chars2: C) -> f64 {
        let dist = self.distance(chars1, chars2);
        let State { buffer1, buffer2, .. } = &*self.state.borrow_mut();
        let len = max!(1, buffer1.len(), buffer2.len());
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
    pub fn similarity<C: Chars<T>>(&self, chars1: C, chars2: C) -> f64 {
        1.0 - self.rel_dist(chars1, chars2)
    }
}

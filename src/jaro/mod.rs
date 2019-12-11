#[cfg(test)]
mod tests;

use std::cell::RefCell;
use crate::slice;
use crate::utils::Rewrite;


const DEFAULT_CAPATITY: usize = 25;

/// # Jaro similarity.
///
/// See [the detailed description][1].
///
/// [1]: https://en.wikipedia.org/wiki/Jaro–Winkler_distance#Jaro_Similarity
///
/// # Usage
///
/// ```rust
/// use eddie::Jaro;
///
/// let jaro = Jaro::new();
/// let sim = jaro.similarity("martha", "marhta");
/// assert!((sim - 0.94).abs() < 0.01);
/// ```
///
/// # Complementary metrics
///
/// Relative distance:
/// ```rust
/// # let jaro = eddie::Jaro::new();
/// # let s1 = "martha";
/// # let s2 = "marhta";
/// let sim = jaro.similarity(s1, s2);
/// let dist = jaro.rel_dist(s1, s2);
/// assert_eq!(dist, 1.0 - sim);
/// ```
pub struct Jaro {
    internal:    slice::Jaro,
    pub buffer1: RefCell<Vec<char>>,
    pub buffer2: RefCell<Vec<char>>,
}


impl Jaro {
    /// Creates a new instance of Jaro struct with an internal state
    /// for the metric methods to reuse.
    ///
    /// # Example
    ///
    /// ```rust
    /// use eddie::Jaro;
    ///
    /// let jaro = Jaro::new();
    /// ```
    pub fn new() -> Self {
        Self {
            internal: slice::Jaro::new(),
            buffer1:  RefCell::new(Vec::with_capacity(DEFAULT_CAPATITY)),
            buffer2:  RefCell::new(Vec::with_capacity(DEFAULT_CAPATITY)),
        }
    }

    /// Similarity metric. Reflects how close two strings are,
    /// ranging from 1.0 (equality) to 0.0 (nothing in common).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::Jaro;
    /// # let mut jaro = Jaro::new();
    /// let sim = jaro.similarity("martha", "marhta");
    /// assert!((sim - 0.94).abs() < 0.01);
    /// ```
    pub fn similarity(&self, chars1: &str, chars2: &str) -> f64 {
        match (chars1.len(), chars2.len()) {
            (0, 0) => { return 1.0; }
            (_, 0) => { return 0.0; }
            (0, _) => { return 0.0; }
            (_, _) => { }
        }
        let buffer1 = &mut *self.buffer1.borrow_mut();
        let buffer2 = &mut *self.buffer2.borrow_mut();
        buffer1.rewrite_with(chars1.chars());
        buffer2.rewrite_with(chars2.chars());
        self.internal.similarity(buffer1, buffer2)
    }

    /// Relative distance metric. Inversion of similarity.
    /// Reflects how far apart two strings are,
    /// ranging from 0.0 (equality) to 1.0 (nothing in common).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::Jaro;
    /// # let mut jaro = Jaro::new();
    /// let dist = jaro.rel_dist("martha", "marhta");
    /// assert!((dist - 0.06).abs() < 0.01);
    /// ```
    pub fn rel_dist(&self, chars1: &str, chars2: &str) -> f64 {
        match (chars1.len(), chars2.len()) {
            (0, 0) => { return 0.0; }
            (_, 0) => { return 1.0; }
            (0, _) => { return 1.0; }
            (_, _) => { }
        }
        let buffer1 = &mut *self.buffer1.borrow_mut();
        let buffer2 = &mut *self.buffer2.borrow_mut();
        buffer1.rewrite_with(chars1.chars());
        buffer2.rewrite_with(chars2.chars());
        self.internal.rel_dist(buffer1, buffer2)
    }
}

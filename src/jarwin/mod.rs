#[cfg(test)]
mod tests;

use std::cell::RefCell;
use crate::utils::Rewrite;
use crate::slice;


const DEFAULT_CAPATITY: usize = 25;


/// # Jaro-Winkler similarity.
///
/// Like Jaro similarity but gives a higher score to the strings
/// that start with the same sequence of characters.
///
/// See [the detailed description][1].
///
/// [1]: https://en.wikipedia.org/wiki/Jaro–Winkler_distance#Jaro–Winkler_Similarity
///
/// # Usage
///
/// ```rust
/// use eddie::JaroWinkler;
///
/// let jarwin = JaroWinkler::new();
/// let sim = jarwin.similarity("martha", "marhta");
/// assert!((sim - 0.96).abs() < 0.01);
/// ```
///
/// # Complementary metrics
///
/// Relative distance:
/// ```rust
/// # let jarwin = eddie::JaroWinkler::new();
/// # let s1 = "martha";
/// # let s2 = "marhta";
/// let sim = jarwin.similarity(s1, s2);
/// let dist = jarwin.rel_dist(s1, s2);
/// assert_eq!(dist, 1.0 - sim);
/// ```
pub struct JaroWinkler {
    internal: slice::JaroWinkler,
    buffer1: RefCell<Vec<char>>,
    buffer2: RefCell<Vec<char>>,
}


impl JaroWinkler {
    /// Creates a new instance of JaroWinkler struct with an internal state
    /// for the metric methods to reuse.
    ///
    /// # Example
    ///
    /// ```rust
    /// use eddie::JaroWinkler;
    ///
    /// let jarwin = JaroWinkler::new();
    /// ```
    pub fn new() -> JaroWinkler {
        Self {
            internal: slice::JaroWinkler::new(),
            buffer1:  RefCell::new(Vec::with_capacity(DEFAULT_CAPATITY)),
            buffer2:  RefCell::new(Vec::with_capacity(DEFAULT_CAPATITY)),
        }
    }

    /// Sets scaling factor for common prefix score boost.
    /// Default value is 0.1.
    /// Panics if it's not in range `[0.0, 0.25]`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::JaroWinkler;
    /// let mut jarwin = JaroWinkler::new();
    ///
    /// let sim1 = jarwin.similarity("martha", "marhta");
    /// jarwin.set_scaling(0.25);
    /// let sim2 = jarwin.similarity("martha", "marhta");
    ///
    /// assert!((sim1 - 0.96).abs() < 0.01);
    /// assert!((sim2 - 0.98).abs() < 0.01);
    /// ```
    pub fn set_scaling(&mut self, scaling: f64) {
        self.internal.set_scaling(scaling);
    }

    /// Similarity metric. Reflects how close two strings are,
    /// ranging from 1.0 (equality) to 0.0 (nothing in common).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::JaroWinkler;
    /// # let mut jarwin = JaroWinkler::new();
    /// let sim = jarwin.similarity("martha", "marhta");
    /// assert!((sim - 0.96).abs() < 0.01);
    /// ```
    pub fn similarity(&self, chars1: &str, chars2: &str) -> f64 {
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
    /// # use eddie::JaroWinkler;
    /// # let mut jarwin = JaroWinkler::new();
    /// let dist = jarwin.rel_dist("martha", "marhta");
    /// assert!((dist - 0.04).abs() < 0.01);
    /// ```
    pub fn rel_dist(&self, chars1: &str, chars2: &str) -> f64 {
        1.0 - self.similarity(chars1, chars2)
    }
}

#[cfg(test)]
mod tests;

use crate::slice;
use crate::utils::buffer::Buffer;


const DEFAULT_CAPACITY: usize = 25;


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
    sliced: slice::JaroWinkler,
    buffer1: Buffer<char>,
    buffer2: Buffer<char>,
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
            sliced: slice::JaroWinkler::new(),
            buffer1: Buffer::with_capacity(DEFAULT_CAPACITY),
            buffer2: Buffer::with_capacity(DEFAULT_CAPACITY),
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
        self.sliced.set_scaling(scaling);
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
    pub fn similarity(&self, str1: &str, str2: &str) -> f64 {
        let buf1 = &*self.buffer1.store(str1.chars()).borrow();
        let buf2 = &*self.buffer2.store(str2.chars()).borrow();
        self.sliced.similarity(buf1, buf2)
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
    pub fn rel_dist(&self, str1: &str, str2: &str) -> f64 {
        1.0 - self.similarity(str1, str2)
    }
}

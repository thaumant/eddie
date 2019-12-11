#[cfg(test)]
mod tests;

use crate::slice;
use crate::utils::buffer::Buffer;


const DEFAULT_CAPACITY: usize = 25;

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
    sliced: slice::Jaro,
    buffer1: Buffer<char>,
    buffer2: Buffer<char>,
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
            sliced: slice::Jaro::new(),
            buffer1: Buffer::with_capacity(DEFAULT_CAPACITY),
            buffer2: Buffer::with_capacity(DEFAULT_CAPACITY),
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
    pub fn similarity(&self, str1: &str, str2: &str) -> f64 {
        match (str1.len(), str2.len()) {
            (0, 0) => { return 1.0; }
            (_, 0) => { return 0.0; }
            (0, _) => { return 0.0; }
            (_, _) => { }
        }
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
    /// # use eddie::Jaro;
    /// # let mut jaro = Jaro::new();
    /// let dist = jaro.rel_dist("martha", "marhta");
    /// assert!((dist - 0.06).abs() < 0.01);
    /// ```
    pub fn rel_dist(&self, str1: &str, str2: &str) -> f64 {
        match (str1.len(), str2.len()) {
            (0, 0) => { return 0.0; }
            (_, 0) => { return 1.0; }
            (0, _) => { return 1.0; }
            (_, _) => { }
        }
        let buf1 = &*self.buffer1.store(str1.chars()).borrow();
        let buf2 = &*self.buffer2.store(str2.chars()).borrow();
        self.sliced.rel_dist(buf1, buf2)
    }
}

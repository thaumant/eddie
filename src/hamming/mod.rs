#[cfg(test)]
mod tests;

use std::cell::RefCell;


/// # Hamming distance.
///
/// See [the detailed description][1].
///
/// [1]: https://en.wikipedia.org/wiki/Hamming_distance
///
/// # Usage
///
/// ```rust
/// use eddie::Hamming;
///
/// let hamming = Hamming::new();
/// let dist = hamming.distance("martha", "marhta");
/// assert_eq!(dist, Some(2));
/// ```
///
/// Returns `None` if string have different lengths:
/// ```rust
/// # let hamming = eddie::Hamming::new();
/// let dist = hamming.distance("martha", "march");
/// assert_eq!(dist, None);
/// ```
///
/// # Complementary metrics
///
/// Relative distance:
/// ```rust
/// # use std::cmp::max;
/// # fn main() {
/// #     if test().is_none() {
/// #         panic!("Expected test to return Some(f), got None");
/// #     };
/// # }
/// # fn test() -> Option<f64> {
/// #     let hamming = eddie::Hamming::new();
/// #     let s1 = "martha";
/// #     let s2 = "marhta";
/// let rel = hamming.rel_dist(s1, s2)?;
/// let dist = hamming.distance(s1, s2).map(|d| d as f64)?;
/// let max_len = max(s1.len(), s2.len()) as f64;
/// assert_eq!(rel, dist / max_len);
/// #     Some(1.0)
/// # }
/// ```
///
/// Similarity:
/// ```rust
/// # use std::cmp::max;
/// # fn main() {
/// #     if test().is_none() {
/// #         panic!("Expected test to return Some(f), got None");
/// #     };
/// # }
/// # fn test() -> Option<f64> {
/// #     let hamming = eddie::Hamming::new();
/// #     let s1 = "martha";
/// #     let s2 = "marhta";
/// let rel = hamming.rel_dist(s1, s2)?;
/// let sim = hamming.similarity(s1, s2)?;
/// assert_eq!(sim, 1.0 - rel);
/// #     Some(1.0)
/// # }
/// ```
pub struct Hamming {
    len: RefCell<usize>,
}


impl Hamming {
    /// Creates a new instance of Hamming struct with an internal state
    /// for the metric methods to reuse.
    ///
    /// # Example
    ///
    /// ```rust
    /// use eddie::Hamming;
    ///
    /// let hamming = Hamming::new();
    /// ```
    pub fn new() -> Self {
        Self { len: RefCell::new(0) }
    }

    /// Distance metric. Returns a number of positions
    /// at wich string characters are different.
    ///
    /// Returns `None` if strings have different lengths.
    ///
    /// # Example
    ///
    /// ```rust
    /// # let hamming = eddie::Hamming::new();
    /// let dist1 = hamming.distance("martha", "marhta");
    /// assert_eq!(dist1, Some(2));
    ///
    /// let dist2 = hamming.distance("martha", "march");
    /// assert_eq!(dist2, None);
    /// ```
    pub fn distance(&self, str1: &str, str2: &str) -> Option<usize> {
        let mut len = 0;
        let mut dist = 0;
        let mut chars1 = str1.chars();
        let mut chars2 = str2.chars();

        loop {
            match (chars1.next(), chars2.next()) {
                (Some(ch1), Some(ch2)) => {
                    if ch1 != ch2 { dist += 1; }
                    len += 1;
                }
                (None, None) => {
                    *self.len.borrow_mut() = len;
                    return Some(dist);
                }
                _ => return None,
            }
        }
    }

    /// Relative distance metric. Returns a distance relative to the string length,
    /// ranging from 0.0 (equality) to 1.0 (nothing in common).
    ///
    /// Returns `None` if strings have different lengths.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::cmp::max;
    /// # fn main() {
    /// #     if test().is_none() {
    /// #         panic!("Expected test to return Some(f), got None");
    /// #     };
    /// # }
    /// # fn test() -> Option<f64> {
    /// #     let hamming = eddie::Hamming::new();
    /// #     let s1 = "martha";
    /// #     let s2 = "marhta";
    /// let dist1 = hamming.rel_dist("martha", "marhta")?;
    /// assert!((dist1 - 0.333).abs() < 0.001);
    ///
    /// let dist2 = hamming.rel_dist("martha", "march");
    /// assert_eq!(dist2, None);
    /// #     Some(1.0)
    /// # }
    /// ```
    pub fn rel_dist(&self, str1: &str, str2: &str) -> Option<f64> {
        match self.distance(str1, str2) {
            None => None,
            Some(0) => Some(0.0),
            Some(dist) => {
                Some(dist as f64 / *self.len.borrow() as f64)
            },
        }
    }

    /// Similarity metric. Inversion of relative distance,
    /// ranging from 1.0 (equality) to 0.0 (nothing in common).
    ///
    /// Returns `None` if strings have different lengths.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::cmp::max;
    /// # fn main() {
    /// #     if test().is_none() {
    /// #         panic!("Expected test to return Some(f), got None");
    /// #     };
    /// # }
    /// # fn test() -> Option<f64> {
    /// #     let hamming = eddie::Hamming::new();
    /// #     let s1 = "martha";
    /// #     let s2 = "marhta";
    /// let sim1 = hamming.similarity("martha", "marhta")?;
    /// assert!((sim1 - 0.666).abs() < 0.001);
    ///
    /// let sim2 = hamming.similarity("martha", "march");
    /// assert_eq!(sim2, None);
    /// #     Some(1.0)
    /// # }
    /// ```
    pub fn similarity(&self, str1: &str, str2: &str) -> Option<f64> {
        self.rel_dist(str1, str2).map(|d| 1.0 - d)
    }
}

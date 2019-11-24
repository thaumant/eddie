#[cfg(test)]
mod tests;

use std::cmp::min;
use std::cell::RefCell;
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
    pub state: RefCell<State>,
}


pub struct State {
    pub word1:    Vec<char>,
    pub word2:    Vec<char>,
    pub matches1: Vec<bool>,
    pub matches2: Vec<bool>,
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
        Jaro {
            state: RefCell::new(State {
                word1:    Vec::with_capacity(DEFAULT_CAPATITY),
                word2:    Vec::with_capacity(DEFAULT_CAPATITY),
                matches1: Vec::with_capacity(DEFAULT_CAPATITY),
                matches2: Vec::with_capacity(DEFAULT_CAPATITY),
            })
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

        let State {
            word1,
            word2,
            matches1,
            matches2,
        } = &mut *self.state.borrow_mut();

        word1.rewrite_with(str1.chars());
        word2.rewrite_with(str2.chars());
        matches1.clear();
        matches2.clear();
        matches1.resize(word1.len(), false);
        matches2.resize(word2.len(), false);

        let mut matches = 0;
        let mut trans = 0;
        let len1 = word1.len();
        let len2 = word2.len();
        let i2_range = max!(1, len1 / 2, len2 / 2) - 1;

        for i1 in 0..len1 {
            let i2_lower = i1 - min(i2_range, i1);
            let i2_upper = min(i1 + i2_range + 1, len2);

            for i2 in i2_lower..i2_upper {
                unsafe {
                    let char1 = word1.get_unchecked(i1);
                    let char2 = word2.get_unchecked(i2);
                    let match2 = matches2.get_unchecked_mut(i2);
                    if !*match2 && char1 == char2 {
                        let match1 = matches1.get_unchecked_mut(i1);
                        *match1 = true;
                        *match2 = true;
                        matches += 1;
                        break;
                    }
                }
            }
        }

        if matches == 0 { return 0.0; }

        let mut i2 = 0;
        for i1 in 0..len1 {
            unsafe {
                if !*matches1.get_unchecked(i1) { continue; }
                while !*matches2.get_unchecked(i2) { i2 += 1; }
                let char1 = word1.get_unchecked(i1);
                let char2 = word2.get_unchecked(i2);
                if char1 != char2 { trans += 1; }
                i2 += 1;
            }
        }

        let matches = matches as f64;
        let trans = trans as f64;
        let len1 = len1 as f64;
        let len2 = len2 as f64;

        (matches/len1 + matches/len2 + ((matches - trans/2.) / matches)) / 3.
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
        1.0 - self.similarity(str1, str2)
    }
}

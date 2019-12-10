#[cfg(test)]
mod tests;

use std::cmp::min;
use std::cell::RefCell;
use crate::utils::{Rewrite, common_prefix_size};


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
    pub buffer1: Vec<BufferItem>,
    pub buffer2: Vec<BufferItem>,
}


#[derive(PartialEq, Clone, Copy)]
pub struct BufferItem {
    pub val: char,
    pub matched: bool,
}


impl BufferItem {
    pub fn new(val: char) -> Self {
        Self { val, matched: false }
    }
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
                buffer1: Vec::with_capacity(DEFAULT_CAPATITY),
                buffer2: Vec::with_capacity(DEFAULT_CAPATITY),
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

        let State { buffer1, buffer2 } = &mut *self.state.borrow_mut();

        buffer1.rewrite_with(str1.chars().map(BufferItem::new));
        buffer2.rewrite_with(str2.chars().map(BufferItem::new));

        let prefix = common_prefix_size(buffer1, buffer2);
        let buffer1 = &mut buffer1[prefix..];
        let buffer2 = &mut buffer2[prefix..];

        let mut matches = 0;

        let len1 = buffer1.len();
        let len2 = buffer2.len();
        let i2_range = max!(1, (len1 + prefix) / 2, (len2 + prefix) / 2) - 1;
        let mut i1 = 0;

        for item1 in buffer1.iter_mut() {
            let i2_lo = i1 - min(i2_range, i1);
            let i2_up = min(i1 + i2_range + 1, len2);
            i1 += 1;

            if i2_lo >= i2_up { continue; }

            for item2 in buffer2[i2_lo..i2_up].iter_mut() {
                if !item2.matched && item1.val == item2.val {
                    item1.matched = true;
                    item2.matched = true;
                    matches += 1;
                    break;
                }
            }
        }

        if prefix + matches == 0 { return 0.0; }

        let mut trans = 0;

        if matches != 0 {
            let mut matches2 = buffer2.iter().filter(|x| x.matched);
            for item1 in buffer1.iter().filter(|x| x.matched) {
                if let Some(item2) = matches2.next() {
                    if item1.val != item2.val { trans += 1; }
                }
            }
        }

        let matches = (prefix + matches) as f64;
        let trans = trans as f64;
        let len1 = (prefix + len1) as f64;
        let len2 = (prefix + len2) as f64;

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

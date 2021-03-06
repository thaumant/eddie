use crate::slice;
use crate::utils::Buffer;


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
    sliced: slice::DamerauLevenshtein<char>,
    buffer1: Buffer<char>,
    buffer2: Buffer<char>,
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
        let sliced = slice::DamerauLevenshtein::new();
        let buffer1 = Buffer::with_capacity(DEFAULT_CAPACITY);
        let buffer2 = Buffer::with_capacity(DEFAULT_CAPACITY);
        Self { sliced, buffer1, buffer2 }
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
    pub fn distance(&self, str1: &str, str2: &str) -> usize {
        let buf1 = &*self.buffer1.store(str1.chars()).borrow();
        let buf2 = &*self.buffer2.store(str2.chars()).borrow();
        self.sliced.distance(buf1, buf2)
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
        let buf1 = &*self.buffer1.store(str1.chars()).borrow();
        let buf2 = &*self.buffer2.store(str2.chars()).borrow();
        self.sliced.rel_dist(buf1, buf2)
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
        let buf1 = &*self.buffer1.store(str1.chars()).borrow();
        let buf2 = &*self.buffer2.store(str2.chars()).borrow();
        self.sliced.similarity(buf1, buf2)
    }
}


#[cfg(test)]
mod tests {
    use super::{DamerauLevenshtein, DEFAULT_CAPACITY};

    fn floor3(num: f64) -> f64 {
        let p = 10usize.pow(3) as f64;
        (num * p).floor() / p
    }

    #[test]
    fn equality() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            "",
            "m",
            "ma",
            "mai",
            "mail",
            "mailb",
            "mailbo",
            "mailbox",
        ];
        for s in sample.iter() {
            assert_eq!(damlev.distance(s, s), 0);
        }
    }

    #[test]
    fn prefix() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1, "mailbox", "mailbo"),
            (2, "mailbox", "mailb"),
            (3, "mailbox", "mail"),
            (4, "mailbox", "mai"),
            (5, "mailbox", "ma"),
            (6, "mailbox", "m"),
            (7, "mailbox", ""),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
            assert_eq!(damlev.distance(s2, s1), *d);
        }
    }

    #[test]
    fn add_del_continuous() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1, "mailbox", "_mailbox"),
            (2, "mailbox", "__mailbox"),
            (3, "mailbox", "___mailbox"),
            (4, "mailbox", "____mailbox"),

            (1, "mailbox", "mail_box"),
            (2, "mailbox", "mail__box"),
            (3, "mailbox", "mail___box"),
            (4, "mailbox", "mail____box"),

            (1, "mailbox", "mailbox_"),
            (2, "mailbox", "mailbox__"),
            (3, "mailbox", "mailbox___"),
            (4, "mailbox", "mailbox____"),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
            assert_eq!(damlev.distance(s2, s1), *d);
        }
    }

    #[test]
    fn sub_continuous() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1, "mailbox", "_ailbox"),
            (2, "mailbox", "__ilbox"),
            (3, "mailbox", "___lbox"),
            (4, "mailbox", "____box"),

            (1, "mailbox", "mai_box"),
            (2, "mailbox", "mai__ox"),
            (3, "mailbox", "ma___ox"),
            (4, "mailbox", "ma____x"),

            (1, "mailbox", "mailbo_"),
            (2, "mailbox", "mailb__"),
            (3, "mailbox", "mail___"),
            (4, "mailbox", "mai____"),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
        }
    }

    #[test]
    fn trans_continuous() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1, "mailbox", "amilbox"),
            (2, "mailbox", "amlibox"),
            (3, "mailbox", "amliobx"),

            (1, "mailbox", "mailbxo"),
            (2, "mailbox", "maiblxo"),
            (3, "mailbox", "miablxo"),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
        }
    }

    #[test]
    fn add_del_intermittent() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1, "mailbox", "_mailbox"),
            (2, "mailbox", "_m_ailbox"),
            (3, "mailbox", "_m_a_ilbox"),
            (4, "mailbox", "_m_a_i_lbox"),

            (1, "mailbox", "mailbox_"),
            (2, "mailbox", "mailbo_x_"),
            (3, "mailbox", "mailb_o_x_"),
            (4, "mailbox", "mail_b_o_x_"),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
            assert_eq!(damlev.distance(s2, s1), *d);
        }
    }

    #[test]
    fn sub_intermittent() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1, "mailbox", "_ailbox"),
            (2, "mailbox", "_a_lbox"),
            (3, "mailbox", "_a_l_ox"),

            (1, "mailbox", "mailbo_"),
            (2, "mailbox", "mail_o_"),
            (3, "mailbox", "ma_l_o_"),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
        }
    }

    #[test]
    fn mixed() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (2, "ca", "abc"),
            (2, "a tc", "a cat"),
            (3, "a cat", "an abct"),
            (2, "crate",     "trace"),
            (2, "captain",   "ptain"),
            (2, "dwayne",    "duane"),
            (1, "martha",    "marhta"),
            (3, "kitten",    "sitting"),
            (3, "mailbox",   "alimbox"),
            (6, "mailbox",   "boxmail"),
            (4, "dixon",     "dicksonx"),
            (2, "jellyfish", "smellyfish"),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
            assert_eq!(damlev.distance(s2, s1), *d);
        }
    }

    #[test]
    fn growth() {
        let damlev = DamerauLevenshtein::new();

        for len in DEFAULT_CAPACITY + 1 .. DEFAULT_CAPACITY * 2 {
            let s1 = &"a".repeat(len);
            let s2 = &"b".repeat(len);
            assert_eq!(damlev.distance(s1, s1), 0);
            assert_eq!(damlev.distance(s1, ""), len);
            assert_eq!(damlev.distance(s1, s2), len);
        }
    }

    #[test]
    fn utf_multibyte() {
        let damlev = DamerauLevenshtein::new();
        let s1 = "もしもし";
        let sample= [
            (1, "もしもしし"),
            (0, "もしもし"),
            (1, "もししも"),
            (1, "もしまし"),
            (1, "もしし"),
            (2, "もし"),
            (3, "し"),
            (4, ""),
        ];
        for (d, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
        }
    }

    #[test]
    fn rel_dist() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (0.000, "",        ""),
            (1.000, "mailbox", ""),
            (0.428, "mailbox", "mail"),
            (0.222, "mailbox", "mail__box"),
            (0.571, "mailbox", "____box"),
            (0.857, "mailbox", "boxmail"),
            (0.428, "mailbox", "amliobx"),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(damlev.rel_dist(s1, s2)), *d);
            assert_eq!(floor3(damlev.rel_dist(s2, s1)), *d);
        }
    }

    #[test]
    fn similarity() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1.000, "",        ""),
            (0.000, "mailbox", ""),
            (0.571, "mailbox", "mail"),
            (0.777, "mailbox", "mail__box"),
            (0.428, "mailbox", "____box"),
            (0.142, "mailbox", "boxmail"),
            (0.571, "mailbox", "amliobx"),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(damlev.similarity(s1, s2)), *d);
            assert_eq!(floor3(damlev.similarity(s2, s1)), *d);
        }
    }
}
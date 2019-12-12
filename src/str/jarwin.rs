use crate::slice;
use crate::utils::Buffer;


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


#[cfg(test)]
mod tests {
    use super::JaroWinkler;


    fn floor3(num: f64) -> f64 {
        let p = 10usize.pow(3) as f64;
        (num * p).floor() / p
    }


    #[test]
    fn equality() {
        let jarwin = JaroWinkler::new();
        let sample = [
            (1., ""),
            (1., "m"),
            (1., "ma"),
            (1., "mai"),
            (1., "mail"),
            (1., "mailb"),
            (1., "mailbo"),
            (1., "mailbox"),
        ];
        for (d, s) in &sample {
            assert_eq!(jarwin.similarity(s, s), *d);
        }
    }


    #[test]
    fn inequality() {
        let jarwin = JaroWinkler::new();
        let sample = [
            (0., "a",     "b"),
            (0., "aa",    "bb"),
            (0., "aaa",   "bbb"),
            (0., "aaaa",  "bbbb"),
            (0., "aaaaa", "bbbbb"),
        ];
        for (d, s1, s2) in &sample {
            assert_eq!(jarwin.similarity(s1, s2), *d);
            assert_eq!(jarwin.similarity(s2, s1), *d);
        }
    }


    #[test]
    fn prefix() {
        let jarwin = JaroWinkler::new();
        let sample = [
            (0.000, "mailbox", ""),
            (0.742, "mailbox", "m"),
            (0.809, "mailbox", "ma"),
            (0.866, "mailbox", "mai"),
            (0.914, "mailbox", "mail"),
            (0.942, "mailbox", "mailb"),
            (0.971, "mailbox", "mailbo"),
        ];
        for (d, s1, s2) in &sample {
            assert_eq!(floor3(jarwin.similarity(s1, s2)), *d);
            assert_eq!(floor3(jarwin.similarity(s2, s1)), *d);
        }
    }


    #[test]
    fn postfix() {
        let jarwin = JaroWinkler::new();
        let sample = [
            (0.952, "mailbox", "ailbox"),
            (0.904, "mailbox", "ilbox"),
            (0.000, "mailbox", "lbox"),
            (0.000, "mailbox", "box"),
            (0.000, "mailbox", "ox"),
            (0.000, "mailbox", "x"),
            (0.000, "mailbox", ""),
        ];
        for (d, s1, s2) in &sample {
            assert_eq!(floor3(jarwin.similarity(s1, s2)), *d);
            assert_eq!(floor3(jarwin.similarity(s2, s1)), *d);
        }
    }


    #[test]
    fn common_prefix_length() {
        let jarwin = JaroWinkler::new();
        let sample = [
            (0.904, "_ailbox", "-ailbox"),
            (0.914, "m_ilbox", "m-ilbox"),
            (0.923, "ma_lbox", "ma-lbox"),
            (0.933, "mai_box", "mai-box"),
            (0.942, "mail_ox", "mail-ox"),
            (0.942, "mailb_x", "mailb-x"),
            (0.942, "mailbo_", "mailbo-"),
        ];
        for (d, s1, s2) in &sample {
            assert_eq!(floor3(jarwin.similarity(s1, s2)), *d);
            assert_eq!(floor3(jarwin.similarity(s2, s1)), *d);
        }
    }


    #[test]
    fn mixed() {
        let jarwin = JaroWinkler::new();
        let sample = [
            (0.000, "ca",        "abc"),
            (0.826, "a tc",      "a cat"),
            (0.811, "a cat",     "an abct"),
            (0.733, "crate",     "trace"),
            (0.804, "captain",   "ptain"),
            (0.840, "dwayne",    "duane"),
            (0.961, "martha",    "marhta"),
            (0.746, "kitten",    "sitting"),
            (0.849, "mailbox",   "alimbox"),
            (0.000, "mailbox",   "boxmail"),
            (0.813, "dixon",     "dicksonx"),
            (0.896, "jellyfish", "smellyfish"),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jarwin.similarity(s1, s2)), *d);
            assert_eq!(floor3(jarwin.similarity(s2, s1)), *d);
        }
    }


    #[test]
    fn rel_dist() {
        let jarwin = JaroWinkler::new();
        let sample = [
            (0.000, "",        ""),
            (1.000, "mailbox", ""),
            (0.085, "mailbox", "mail"),
            (0.095, "mailbox", "ilbox"),
            (0.085, "m_ilbox", "m-ilbox"),
            (0.150, "mailbox", "alimbox"),
            (1.000, "mailbox", "boxmail"),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jarwin.rel_dist(s1, s2)), *d);
            assert_eq!(floor3(jarwin.rel_dist(s2, s1)), *d);
        }
    }
}
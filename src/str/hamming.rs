use std::cell::Cell;


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
/// Returns `None` if strings have different lengths:
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
    len: Cell<usize>,
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
        Self { len: Cell::new(0) }
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
                    self.len.set(len);
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
                Some(dist as f64 / self.len.get() as f64)
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


#[cfg(test)]
mod tests {
    use super::Hamming;

    fn floor3(num: f64) -> f64 {
        let p = 10usize.pow(3) as f64;
        (num * p).floor() / p
    }

    #[test]
    fn equality() {
        let hamming = Hamming::new();
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
            assert_eq!(hamming.distance(s, s), Some(0));
        }
    }

    #[test]
    fn inequality() {
        let hamming = Hamming::new();
        for i in 1..10 {
            let s1 = "a".repeat(i);
            let s2 = "b".repeat(i);
            assert_eq!(hamming.distance(&s1, &s2), Some(i));
        }
    }

    #[test]
    fn length_difference() {
        let hamming = Hamming::new();
        for len1 in 1..10 {
            for len2 in 0 .. len1 - 1 {
                let a1 = "a".repeat(len1);
                let a2 = "a".repeat(len2);
                let b2 = "b".repeat(len2);

                assert_eq!(hamming.distance(&a1, &a2), None);
                assert_eq!(hamming.distance(&a2, &a1), None);

                assert_eq!(hamming.distance(&a1, &b2), None);
                assert_eq!(hamming.distance(&b2, &a1), None);
            }
        }
    }

    #[test]
    fn sub_continuous() {
        let hamming = Hamming::new();
        let sample = [
            (Some(1), "mailbox", "_ailbox"),
            (Some(2), "mailbox", "__ilbox"),
            (Some(3), "mailbox", "___lbox"),
            (Some(4), "mailbox", "____box"),

            (Some(1), "mailbox", "mai_box"),
            (Some(2), "mailbox", "mai__ox"),
            (Some(3), "mailbox", "ma___ox"),
            (Some(4), "mailbox", "ma____x"),

            (Some(1), "mailbox", "mailbo_"),
            (Some(2), "mailbox", "mailb__"),
            (Some(3), "mailbox", "mail___"),
            (Some(4), "mailbox", "mai____"),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(hamming.distance(s1, s2), *d);
        }
    }

    #[test]
    fn sub_intermittent() {
        let hamming = Hamming::new();
        let sample = [
            (Some(1), "mailbox", "_ailbox"),
            (Some(2), "mailbox", "_a_lbox"),
            (Some(3), "mailbox", "_a_l_ox"),

            (Some(1), "mailbox", "mailbo_"),
            (Some(2), "mailbox", "mail_o_"),
            (Some(3), "mailbox", "ma_l_o_"),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(hamming.distance(s1, s2), *d);
        }
    }

    #[test]
    fn utf_multibyte() {
        let hamming = Hamming::new();
        let s1 = "もしもし";
        let sample= [
            (Some(0), "もしもし"),
            (Some(1), "もしまし"),
            (Some(1), "もし_し"),
            (None, "もしも"),
        ];
        for (d, s2) in sample.iter() {
            assert_eq!(hamming.distance(s1, s2), *d);
            assert_eq!(hamming.distance(s2, s1), *d);
        }
    }

    #[test]
    fn rel_dist() {
        let hamming = Hamming::new();
        let sample = [
            (Some(0.000), "",        ""),
            (Some(0.000), "mailbox", "mailbox"),
            (Some(1.000), "mailbox", "boxmail"),
            (Some(0.285), "mailbox", "mai__ox"),
            (Some(0.571), "mailbox", "amilobx"),
            (None,        "mailbox", "mail"),
            (None,        "mailbox", ""),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(hamming.rel_dist(s1, s2).map(floor3), *d);
            assert_eq!(hamming.rel_dist(s2, s1).map(floor3), *d);
        }
    }

    #[test]
    fn similarity() {
        let hamming = Hamming::new();
        let sample = [
            (Some(1.000), "",        ""),
            (Some(1.000), "mailbox", "mailbox"),
            (Some(0.000), "mailbox", "boxmail"),
            (Some(0.428), "mailbox", "amilobx"),
            (Some(0.714), "mailbox", "mai__ox"),
            (None,        "mailbox", "mail"),
            (None,        "mailbox", ""),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(hamming.similarity(s1, s2).map(floor3), *d);
            assert_eq!(hamming.similarity(s2, s1).map(floor3), *d);
        }
    }
}
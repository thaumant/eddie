use crate::utils::{common_affix_sizes, Buffer};


const DEFAULT_CAPACITY: usize = 25;


/// # Levenshtein distance.
///
/// See [the detailed description][1].
///
/// [1]: https://en.wikipedia.org/wiki/Levenshtein_distance
///
/// # Usage
///
/// ```rust
/// use eddie::slice::Levenshtein;
///
/// let lev = Levenshtein::new();
/// let dist = lev.distance(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
/// assert_eq!(dist, 2);
/// ```
///
/// # Complementary metrics
///
/// Relative distance:
/// ```rust
/// # use std::cmp::max;
/// # let lev = eddie::slice::Levenshtein::new();
/// # let s1 = &[1, 2, 3, 4, 5];
/// # let s2 = &[1, 3, 2, 4, 5];
/// let dist = lev.distance(s1, s2);
/// let rel = lev.rel_dist(s1, s2);
/// let max_len = max(s1.len(), s2.len());
/// assert_eq!(rel, dist as f64 / max_len as f64);
/// ```
///
/// Similarity:
/// ```rust
/// # use std::cmp::max;
/// # let lev = eddie::slice::Levenshtein::new();
/// # let s1 = &[1, 2, 3, 4, 5];
/// # let s2 = &[1, 3, 2, 4, 5];
/// let rel = lev.rel_dist(s1, s2);
/// let sim = lev.similarity(s1, s2);
/// assert_eq!(sim, 1.0 - rel);
/// ```
pub struct Levenshtein {
    dists: Buffer<usize>,
}


impl Levenshtein {
    /// Creates a new instance of Levenshtein struct with an internal state
    /// for the metric methods to reuse.
    ///
    /// # Example
    ///
    /// ```rust
    /// use eddie::slice::Levenshtein;
    ///
    /// let lev: Levenshtein = Levenshtein::new();
    /// ```
    pub fn new() -> Self {
        Self { dists: Buffer::with_capacity(DEFAULT_CAPACITY + 1) }
    }

    /// Distance metric. Returns a number of edits
    /// (character additions, deletions, and substitutions)
    /// required to transform one slice into the other.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::slice::Levenshtein;
    /// # let lev = Levenshtein::new();
    /// let dist = lev.distance(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
    /// assert_eq!(dist, 2);
    /// ```
    pub fn distance<T: PartialEq + Copy>(&self, slice1: &[T], slice2: &[T]) -> usize {
        let dists = &mut *self.dists.store(1 .. slice2.len() + 2).borrow_mut();

        let (prefix, postfix) = common_affix_sizes(slice1, slice2);
        let slice1 = { let len = slice1.len(); &slice1[prefix .. len - postfix] };
        let slice2 = { let len = slice2.len(); &slice2[prefix .. len - postfix] };

        let mut dist = slice2.len();
        let mut prev;

        for (i1, x1) in slice1.into_iter().enumerate() {
            dist = i1 + 1;
            prev = i1;

            for (x2, prev2) in slice2.into_iter().zip(dists.into_iter()) {
                dist = min!(
                    dist + 1,
                    *prev2 + 1,
                    prev + (x1 != x2) as usize
                );
                prev = *prev2;
                *prev2 = dist;
            }
        }

        dist
    }

    /// Relative distance metric. Returns a number of edits relative to the length of
    /// the longest slice, ranging from 0.0 (equality) to 1.0 (nothing in common).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::slice::Levenshtein;
    /// # let lev = Levenshtein::new();
    /// let dist = lev.rel_dist(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
    /// assert!((dist - 0.4).abs() < 0.001);
    /// ```
    pub fn rel_dist<T: PartialEq + Copy>(&self, slice1: &[T], slice2: &[T]) -> f64 {
        let dist = self.distance(slice1, slice2);
        let len = max!(1, slice1.len(), slice2.len());
        dist as f64 / len as f64
    }

    /// Similarity metric. Inversion of relative distance,
    /// ranging from 1.0 (equality) to 0.0 (nothing in common).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::slice::Levenshtein;
    /// # let lev = Levenshtein::new();
    /// let sim = lev.similarity(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
    /// assert!((sim - 0.6).abs() < 0.001);
    /// ```
    pub fn similarity<T: PartialEq + Copy>(&self, slice1: &[T], slice2: &[T]) -> f64 {
        1.0 - self.rel_dist(slice1, slice2)
    }
}


#[cfg(test)]
mod tests {
    use super::Levenshtein;

    #[test]
    fn equality() {
        let leven = Levenshtein::new();
        let sample = [
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
        ];
        for s in sample.iter() {
            assert_eq!(leven.distance(s, s), 0);
        }
    }

    #[test]
    fn prefix() {
        let leven = Levenshtein::new();
        let sample = [
            (0, vec![1, 2, 3], vec![1, 2, 3]),
            (1, vec![1, 2, 3], vec![1, 2]),
            (2, vec![1, 2, 3], vec![1]),
            (3, vec![1, 2, 3], vec![]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(leven.distance(s1, s2), *d);
            assert_eq!(leven.distance(s2, s1), *d);
        }
    }

    #[test]
    fn add_del_continuous() {
        let leven = Levenshtein::new();
        let sample = [
            (1, vec![1, 2, 3], vec![0, 1, 2, 3]),
            (2, vec![1, 2, 3], vec![0, 0, 1, 2, 3]),
            (3, vec![1, 2, 3], vec![0, 0, 0, 1, 2, 3]),

            (1, vec![1, 2, 3], vec![1, 0, 2, 3]),
            (2, vec![1, 2, 3], vec![1, 0, 0, 2, 3]),
            (3, vec![1, 2, 3], vec![1, 0, 0, 0, 2, 3]),

            (1, vec![1, 2, 3], vec![1, 2, 3, 0]),
            (2, vec![1, 2, 3], vec![1, 2, 3, 0, 0]),
            (3, vec![1, 2, 3], vec![1, 2, 3, 0, 0, 0]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(leven.distance(s1, s2), *d);
            assert_eq!(leven.distance(s2, s1), *d);
        }
    }

    #[test]
    fn sub_continuous() {
        let leven = Levenshtein::new();
        let sample = [
            (1, vec![1, 2, 3, 4], vec![0, 2, 3, 4]),
            (2, vec![1, 2, 3, 4], vec![0, 0, 3, 4]),
            (3, vec![1, 2, 3, 4], vec![0, 0, 0, 4]),

            (1, vec![1, 2, 3, 4], vec![1, 0, 3, 4]),
            (2, vec![1, 2, 3, 4], vec![1, 0, 0, 4]),

            (1, vec![1, 2, 3, 4], vec![1, 2, 3, 0]),
            (2, vec![1, 2, 3, 4], vec![1, 2, 0, 0]),
            (3, vec![1, 2, 3, 4], vec![1, 0, 0, 0]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(leven.distance(s1, s2), *d);
        }
    }

    #[test]
    fn trans_continuous() {
        let leven = Levenshtein::new();
        let sample = [
            (2, vec![1, 2, 3, 4], vec![2, 1, 3, 4]), // swap 1 and 2
            (3, vec![1, 2, 3, 4], vec![2, 1, 4, 3]), // swap 3 and 4
            (4, vec![1, 2, 3, 4], vec![2, 4, 1, 3]), // swap 1 and 4
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(leven.distance(s1, s2), *d);
        }
    }

    #[test]
    fn add_del_intermittent() {
        let leven = Levenshtein::new();
        let sample = [
            (1, vec![1, 2, 3], vec![0, 1, 2, 3]),
            (2, vec![1, 2, 3], vec![0, 1, 0, 2, 3]),
            (3, vec![1, 2, 3], vec![0, 1, 0, 2, 0, 3]),

            (1, vec![1, 2, 3], vec![1, 2, 3, 0]),
            (2, vec![1, 2, 3], vec![1, 2, 0, 3, 0]),
            (3, vec![1, 2, 3], vec![1, 0, 2, 0, 3, 0]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(leven.distance(s1, s2), *d);
            assert_eq!(leven.distance(s2, s1), *d);
        }
    }

    #[test]
    fn sub_intermittent() {
        let leven = Levenshtein::new();
        let sample = [
            (1, vec![1, 2, 3, 4], vec![0, 2, 3, 4]),
            (2, vec![1, 2, 3, 4], vec![0, 2, 0, 4]),

            (1, vec![1, 2, 3, 4], vec![1, 2, 3, 0]),
            (2, vec![1, 2, 3, 4], vec![1, 0, 3, 0]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(leven.distance(s1, s2), *d);
        }
    }

    #[test]
    fn rel_dist() {
        let leven = Levenshtein::new();
        let sample = [
            (0.00, vec![],           vec![]),
            (1.00, vec![1, 2, 3, 4], vec![]),
            (0.50, vec![1, 2, 3, 4], vec![1, 2]),
            (0.20, vec![1, 2, 3, 4], vec![1, 2, 0, 3, 4]),
            (0.50, vec![1, 2, 3, 4], vec![0, 0, 3, 4]),
            (1.00, vec![1, 2, 3, 4], vec![3, 4, 1, 2]),
            (0.75, vec![1, 2, 3, 4], vec![2, 1, 4, 3]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(leven.rel_dist(s1, s2), *d);
            assert_eq!(leven.rel_dist(s2, s1), *d);
        }
    }

    #[test]
    fn similarity() {
        let leven = Levenshtein::new();
        let sample = [
            (1.00, vec![],           vec![]),
            (0.00, vec![1, 2, 3, 4], vec![]),
            (0.50, vec![1, 2, 3, 4], vec![1, 2]),
            (0.80, vec![1, 2, 3, 4], vec![1, 2, 0, 3, 4]),
            (0.50, vec![1, 2, 3, 4], vec![0, 0, 3, 4]),
            (0.00, vec![1, 2, 3, 4], vec![3, 4, 1, 2]),
            (0.25, vec![1, 2, 3, 4], vec![2, 1, 4, 3]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(leven.similarity(s1, s2), *d);
            assert_eq!(leven.similarity(s2, s1), *d);
        }
    }

    #[test]
    fn growth() {
        let leven = Levenshtein::new();
        for len in (1..1001).step_by(100) {
            let mut v1 = Vec::with_capacity(len);
            let mut v2 = Vec::with_capacity(len);
            v1.resize(len, 1);
            v2.resize(len, 2);
            assert_eq!(leven.distance(&v1, &v1), 0);
            assert_eq!(leven.distance(&v1, &[]), len);
            assert_eq!(leven.distance(&v1, &v2), len);
        }
    }
}

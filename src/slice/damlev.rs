use std::cmp::max;
use std::collections::BTreeMap;
use std::cell::RefCell;
use crate::utils::common_affix_sizes;
use super::matrix::DistMatrix;


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
/// use eddie::slice::DamerauLevenshtein;
///
/// let damlev = DamerauLevenshtein::new();
/// let dist = damlev.distance(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
/// assert_eq!(dist, 1);
/// ```
///
/// # Complementary metrics
///
/// Relative distance:
/// ```rust
/// # use std::cmp::max;
/// # let damlev = eddie::slice::DamerauLevenshtein::new();
/// # let s1 = &[1, 2, 3, 4, 5];
/// # let s2 = &[1, 3, 2, 4, 5];
/// let dist = damlev.distance(s1, s2);
/// let rel = damlev.rel_dist(s1, s2);
/// let max_len = max(s1.len(), s2.len());
/// assert_eq!(rel, dist as f64 / max_len as f64);
/// ```
///
/// Similarity:
/// ```rust
/// # use std::cmp::max;
/// # let damlev = eddie::slice::DamerauLevenshtein::new();
/// # let s1 = &[1, 2, 3, 4, 5];
/// # let s2 = &[1, 3, 2, 4, 5];
/// let rel = damlev.rel_dist(s1, s2);
/// let sim = damlev.similarity(s1, s2);
/// assert_eq!(sim, 1.0 - rel);
/// ```
pub struct DamerauLevenshtein<T: PartialEq + Copy + Ord> {
    dists:   RefCell<DistMatrix>,
    last_i1: RefCell<BTreeMap<T, usize>>,
}


impl<T: PartialEq + Copy + Ord> DamerauLevenshtein<T> {
    /// Creates a new instance of DamerauLevenshtein struct with
    /// an internal state for the metric methods to reuse.
    ///
    /// # Example
    ///
    /// ```rust
    /// use eddie::slice::DamerauLevenshtein;
    ///
    /// let damlev: DamerauLevenshtein<usize> = DamerauLevenshtein::new();
    /// ```
    pub fn new() -> Self {
        let dists   = RefCell::new(DistMatrix::new(DEFAULT_CAPACITY + 2));
        let last_i1 = RefCell::new(BTreeMap::new());
        Self { dists, last_i1 }
    }

    /// Distance metric. Returns a number of edits
    /// (character additions, deletions, substitutions, and transpositions)
    /// required to transform one slice into the other.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::slice::DamerauLevenshtein;
    /// # let damlev = DamerauLevenshtein::new();
    /// let dist = damlev.distance(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
    /// assert_eq!(dist, 1);
    /// ```
    pub fn distance(&self, slice1: &[T], slice2: &[T]) -> usize {
        let (prefix, postfix) = common_affix_sizes(slice1, slice2);
        let mut slice1 = { let len = slice1.len(); &slice1[prefix .. len - postfix] };
        let mut slice2 = { let len = slice2.len(); &slice2[prefix .. len - postfix] };
        if slice2.len() < slice1.len() {
            std::mem::swap(&mut slice1, &mut slice2);
        }

        let dists = &mut *self.dists.borrow_mut();
        dists.grow(max(slice1.len() + 2, slice2.len() + 2));

        let last_i1 = &mut *self.last_i1.borrow_mut();
        last_i1.clear();

        for (i1, &x1) in slice1.iter().enumerate() {
            let mut l2 = 0;

            for (i2, &x2) in slice2.iter().enumerate() {
                let l1 = *last_i1.get(&x2).unwrap_or(&0);

                unsafe {
                    dists.set(i1 + 2, i2 + 2, min!(
                        dists.get(i1 + 2, i2 + 1) + 1,
                        dists.get(i1 + 1, i2 + 2) + 1,
                        dists.get(i1 + 1, i2 + 1) + (x1 != x2) as usize,
                        dists.get(l1, l2) + (i1 - l1) + (i2 - l2) + 1
                    ));
                }

                if x1 == x2 { l2 = i2 + 1; }
            }
            last_i1.insert(x1, i1 + 1);
        }

        unsafe { dists.get(slice1.len() + 1, slice2.len() + 1) }
    }

    /// Relative distance metric. Returns a number of edits relative to the length of
    /// the longest slice, ranging from 0.0 (equality) to 1.0 (nothing in common).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::slice::DamerauLevenshtein;
    /// # let damlev = DamerauLevenshtein::new();
    /// let dist = damlev.rel_dist(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
    /// assert!((dist - 0.2).abs() < 0.001);
    /// ```
    pub fn rel_dist(&self, slice1: &[T], slice2: &[T]) -> f64 {
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
    /// # use eddie::slice::DamerauLevenshtein;
    /// # let damlev = DamerauLevenshtein::new();
    /// let sim = damlev.similarity(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
    /// assert!((sim - 0.8).abs() < 0.001);
    /// ```
    pub fn similarity(&self, slice1: &[T], slice2: &[T]) -> f64 {
        1.0 - self.rel_dist(slice1, slice2)
    }
}


#[cfg(test)]
mod tests {
    use super::DamerauLevenshtein;

    #[test]
    fn equality() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
        ];
        for s in sample.iter() {
            assert_eq!(damlev.distance(s, s), 0);
        }
    }

    #[test]
    fn prefix() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (0, vec![1, 2, 3], vec![1, 2, 3]),
            (1, vec![1, 2, 3], vec![1, 2]),
            (2, vec![1, 2, 3], vec![1]),
            (3, vec![1, 2, 3], vec![]),
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
            assert_eq!(damlev.distance(s1, s2), *d);
            assert_eq!(damlev.distance(s2, s1), *d);
        }
    }

    #[test]
    fn sub_continuous() {
        let damlev = DamerauLevenshtein::new();
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
            assert_eq!(damlev.distance(s1, s2), *d);
        }
    }

    #[test]
    fn trans_continuous() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1, vec![1, 2, 3, 4], vec![2, 1, 3, 4]), // swap 1 and 2
            (2, vec![1, 2, 3, 4], vec![2, 1, 4, 3]), // swap 3 and 4
            (3, vec![1, 2, 3, 4], vec![2, 4, 1, 3]), // swap 1 and 4
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
        }
    }

    #[test]
    fn add_del_intermittent() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1, vec![1, 2, 3], vec![0, 1, 2, 3]),
            (2, vec![1, 2, 3], vec![0, 1, 0, 2, 3]),
            (3, vec![1, 2, 3], vec![0, 1, 0, 2, 0, 3]),

            (1, vec![1, 2, 3], vec![1, 2, 3, 0]),
            (2, vec![1, 2, 3], vec![1, 2, 0, 3, 0]),
            (3, vec![1, 2, 3], vec![1, 0, 2, 0, 3, 0]),
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
            (1, vec![1, 2, 3, 4], vec![0, 2, 3, 4]),
            (2, vec![1, 2, 3, 4], vec![0, 2, 0, 4]),

            (1, vec![1, 2, 3, 4], vec![1, 2, 3, 0]),
            (2, vec![1, 2, 3, 4], vec![1, 0, 3, 0]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
        }
    }

    #[test]
    fn rel_dist() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (0.00, vec![],           vec![]),
            (1.00, vec![1, 2, 3, 4], vec![]),
            (0.50, vec![1, 2, 3, 4], vec![1, 2]),
            (0.20, vec![1, 2, 3, 4], vec![1, 2, 0, 3, 4]),
            (0.50, vec![1, 2, 3, 4], vec![0, 0, 3, 4]),
            (1.00, vec![1, 2, 3, 4], vec![3, 4, 1, 2]),
            (0.50, vec![1, 2, 3, 4], vec![2, 1, 4, 3]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.rel_dist(s1, s2), *d);
            assert_eq!(damlev.rel_dist(s2, s1), *d);
        }
    }

    #[test]
    fn similarity() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1.00, vec![],           vec![]),
            (0.00, vec![1, 2, 3, 4], vec![]),
            (0.50, vec![1, 2, 3, 4], vec![1, 2]),
            (0.80, vec![1, 2, 3, 4], vec![1, 2, 0, 3, 4]),
            (0.50, vec![1, 2, 3, 4], vec![0, 0, 3, 4]),
            (0.00, vec![1, 2, 3, 4], vec![3, 4, 1, 2]),
            (0.50, vec![1, 2, 3, 4], vec![2, 1, 4, 3]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.similarity(s1, s2), *d);
            assert_eq!(damlev.similarity(s2, s1), *d);
        }
    }

    #[test]
    fn growth() {
        let damlev = DamerauLevenshtein::new();
        for len in (1..1001).step_by(100) {
            let mut v1 = Vec::with_capacity(len);
            let mut v2 = Vec::with_capacity(len);
            v1.resize(len, 1);
            v2.resize(len, 2);
            assert_eq!(damlev.distance(&v1, &v1), 0);
            assert_eq!(damlev.distance(&v1, &[]), len);
            assert_eq!(damlev.distance(&v1, &v2), len);
        }
    }
}
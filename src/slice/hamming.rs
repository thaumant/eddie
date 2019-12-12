use std::cmp::max;

/// # Hamming distance.
///
/// See [the detailed description][1].
///
/// [1]: https://en.wikipedia.org/wiki/Hamming_distance
///
/// # Usage
///
/// ```rust
/// use eddie::slice::Hamming;
///
/// let hamming = Hamming::new();
/// let dist = hamming.distance(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
/// assert_eq!(dist, Some(2));
/// ```
///
/// Returns `None` if slices have different lengths:
/// ```rust
/// # let hamming = eddie::slice::Hamming::new();
/// let dist = hamming.distance(&[1, 2, 3, 4, 5], &[1, 3, 2, 4]);
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
/// #     let hamming = eddie::slice::Hamming::new();
/// #     let s1 = &[1, 2, 3, 4, 5];
/// #     let s2 = &[1, 3, 2, 4, 5];
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
/// #     let hamming = eddie::slice::Hamming::new();
/// #     let s1 = &[1, 2, 3, 4, 5];
/// #     let s2 = &[1, 3, 2, 4, 5];
/// let rel = hamming.rel_dist(s1, s2)?;
/// let sim = hamming.similarity(s1, s2)?;
/// assert_eq!(sim, 1.0 - rel);
/// #     Some(1.0)
/// # }
/// ```
pub struct Hamming { }


impl Hamming {
    /// Creates a new instance of Hamming struct with an internal state
    /// for the metric methods to reuse.
    ///
    /// # Example
    ///
    /// ```rust
    /// use eddie::slice::Hamming;
    ///
    /// let hamming = Hamming::new();
    /// ```
    pub fn new() -> Self {
        Self { }
    }

    /// Distance metric. Returns a number of positions
    /// at wich slice items are different.
    ///
    /// Returns `None` if slices have different lengths.
    ///
    /// # Example
    ///
    /// ```rust
    /// # let hamming = eddie::slice::Hamming::new();
    /// let dist1 = hamming.distance(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
    /// assert_eq!(dist1, Some(2));
    ///
    /// let dist2 = hamming.distance(&[1, 2, 3, 4, 5], &[1, 3, 2, 4]);
    /// assert_eq!(dist2, None);
    /// ```
    pub fn distance<T: PartialEq + Copy>(&self, slice1: &[T], slice2: &[T]) -> Option<usize> {
        if slice1.len() != slice2.len() { return None; }
        let dist = slice1.into_iter()
            .zip(slice2.into_iter())
            .filter(|(x1, x2)| x1 != x2)
            .count();
        Some(dist)
    }

    /// Relative distance metric. Returns a distance relative to the slice length,
    /// ranging from 0.0 (equality) to 1.0 (nothing in common).
    ///
    /// Returns `None` if slices have different lengths.
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
    /// #     let hamming = eddie::slice::Hamming::new();
    /// #     let s1 = &[1, 2, 3, 4, 5];
    /// #     let s2 = &[1, 3, 2, 4, 5];
    /// let dist1 = hamming.rel_dist(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5])?;
    /// assert!((dist1 - 0.4).abs() < 0.001);
    ///
    /// let dist2 = hamming.rel_dist(&[1, 2, 3, 4, 5], &[1, 3, 2, 4]);
    /// assert_eq!(dist2, None);
    /// #     Some(1.0)
    /// # }
    /// ```
    pub fn rel_dist<T: PartialEq + Copy>(&self, slice1: &[T], slice2: &[T]) -> Option<f64> {
        self.distance(slice1, slice2)
            .map(|dist| dist as f64 / max(slice1.len(), 1) as f64)
    }

    /// Similarity metric. Inversion of relative distance,
    /// ranging from 1.0 (equality) to 0.0 (nothing in common).
    ///
    /// Returns `None` if slices have different lengths.
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
    /// #     let hamming = eddie::slice::Hamming::new();
    /// #     let s1 = &[1, 2, 3, 4, 5];
    /// #     let s2 = &[1, 3, 2, 4, 5];
    /// let sim1 = hamming.similarity(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5])?;
    /// assert!((sim1 - 0.6).abs() < 0.001);
    ///
    /// let sim2 = hamming.similarity(&[1, 2, 3, 4, 5], &[1, 3, 2, 4]);
    /// assert_eq!(sim2, None);
    /// #     Some(1.0)
    /// # }
    /// ```
    pub fn similarity<T: PartialEq + Copy>(&self, slice1: &[T], slice2: &[T]) -> Option<f64> {
        self.rel_dist(slice1, slice2)
            .map(|dist| 1.0 - dist)
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
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
        ];
        for s in sample.iter() {
            assert_eq!(hamming.distance(s, s), Some(0));
        }
    }

    #[test]
    fn inequality() {
        let hamming = Hamming::new();
        for i in 1..10 {
            let s1: Vec<&usize> = [1].into_iter().cycle().take(i).collect();
            let s2: Vec<&usize> = [2].into_iter().cycle().take(i).collect();
            assert_eq!(hamming.distance(&s1, &s2), Some(i));
        }
    }

    #[test]
    fn length_difference() {
        let hamming = Hamming::new();
        for len1 in 1..10 {
            for len2 in 0 .. len1 - 1 {
                let a1: Vec<&usize> = [1].into_iter().cycle().take(len1).collect();
                let a2: Vec<&usize> = [1].into_iter().cycle().take(len2).collect();
                let b2: Vec<&usize> = [2].into_iter().cycle().take(len2).collect();

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
            (Some(1), vec![1, 2, 3, 4], vec![0, 2, 3, 4]),
            (Some(2), vec![1, 2, 3, 4], vec![0, 0, 3, 4]),
            (Some(3), vec![1, 2, 3, 4], vec![0, 0, 0, 4]),

            (Some(1), vec![1, 2, 3, 4], vec![1, 0, 3, 4]),
            (Some(2), vec![1, 2, 3, 4], vec![1, 0, 0, 4]),

            (Some(1), vec![1, 2, 3, 4], vec![1, 2, 3, 0]),
            (Some(2), vec![1, 2, 3, 4], vec![1, 2, 0, 0]),
            (Some(3), vec![1, 2, 3, 4], vec![1, 0, 0, 0]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(hamming.distance(s1, s2), *d);
        }
    }

    #[test]
    fn sub_intermittent() {
        let hamming = Hamming::new();
        let sample = [
            (1, vec![1, 2, 3, 4], vec![0, 2, 3, 4]),
            (2, vec![1, 2, 3, 4], vec![0, 2, 0, 4]),

            (1, vec![1, 2, 3, 4], vec![1, 2, 3, 0]),
            (2, vec![1, 2, 3, 4], vec![1, 0, 3, 0]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(hamming.distance(s1, s2), Some(*d));
        }
    }

    #[test]
    fn rel_dist() {
        let hamming = Hamming::new();
        let sample = [
            (Some(0.000), vec![],           vec![]),
            (Some(0.000), vec![1, 2, 3, 4], vec![1, 2, 3, 4]),
            (Some(1.000), vec![1, 2, 3, 4], vec![3, 4, 1, 2]),
            (Some(0.500), vec![1, 2, 3, 4], vec![0, 0, 3, 4]),
            (None,        vec![1, 2, 3, 4], vec![1, 2]),
            (None,        vec![1, 2, 3, 4], vec![]),
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
            (Some(1.000), vec![],           vec![]),
            (Some(1.000), vec![1, 2, 3, 4], vec![1, 2, 3, 4]),
            (Some(0.000), vec![1, 2, 3, 4], vec![3, 4, 1, 2]),
            (Some(0.500), vec![1, 2, 3, 4], vec![0, 0, 3, 4]),
            (None,        vec![1, 2, 3, 4], vec![1, 2]),
            (None,        vec![1, 2, 3, 4], vec![]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(hamming.similarity(s1, s2).map(floor3), *d);
            assert_eq!(hamming.similarity(s2, s1).map(floor3), *d);
        }
    }
}

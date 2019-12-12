use std::cmp::min;
use std::ops::Range;
use std::cell::RefCell;
use crate::utils::common_prefix_size;
use crate::utils::Zippable;


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
/// use eddie::slice::Jaro;
///
/// let jaro = Jaro::new();
/// let sim = jaro.similarity(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
/// assert!((sim - 0.93).abs() < 0.01);
/// ```
///
/// # Complementary metrics
///
/// Relative distance:
/// ```rust
/// # let jaro = eddie::slice::Jaro::new();
/// # let s1 = &[1, 2, 3, 4, 5];
/// # let s2 = &[1, 3, 2, 4, 5];
/// let sim = jaro.similarity(s1, s2);
/// let dist = jaro.rel_dist(s1, s2);
/// assert_eq!(dist, 1.0 - sim);
/// ```
pub struct Jaro {
    matches1: RefCell<Vec<bool>>,
    matches2: RefCell<Vec<bool>>,
}


impl Jaro {
    /// Creates a new instance of Jaro struct with an internal state
    /// for the metric methods to reuse.
    ///
    /// # Example
    ///
    /// ```rust
    /// use eddie::slice::Jaro;
    ///
    /// let jaro = Jaro::new();
    /// ```
    pub fn new() -> Self {
        Self {
            matches1: RefCell::new(Vec::with_capacity(DEFAULT_CAPACITY)),
            matches2: RefCell::new(Vec::with_capacity(DEFAULT_CAPACITY)),
        }
    }

    /// Similarity metric. Reflects how close two slices are,
    /// ranging from 1.0 (equality) to 0.0 (nothing in common).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::slice::Jaro;
    /// # let mut jaro = Jaro::new();
    /// let sim = jaro.similarity(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
    /// assert!((sim - 0.93).abs() < 0.01);
    /// ```
    pub fn similarity<T: PartialEq + Copy>(&self, slice1: &[T], slice2: &[T]) -> f64 {
        match (slice1.len(), slice2.len()) {
            (0, 0) => { return 1.0; }
            (_, 0) => { return 0.0; }
            (0, _) => { return 0.0; }
            (_, _) => { }
        }

        let prefix = common_prefix_size(slice1, slice2);
        let slice1 = &slice1[prefix..];
        let slice2 = &slice2[prefix..];

        let matches1 = &mut *self.matches1.borrow_mut();
        let matches2 = &mut *self.matches2.borrow_mut();
        matches1.clear();
        matches2.clear();
        matches1.resize(slice1.len(), false);
        matches2.resize(slice2.len(), false);

        let mut matches = 0;

        let len1 = slice1.len();
        let len2 = slice2.len();
        let i2_range = max!(1, (len1 + prefix) / 2, (len2 + prefix) / 2) - 1;
        let mut i1 = 0;

        for (x1, match1) in (slice1, &mut matches1[..]).zip() {
            let rng = get_range(i1, i2_range, len2);
            if rng.start >= rng.end { continue; }
            for (x2, match2) in (&slice2[rng.clone()], &mut matches2[rng]).zip() {
                if !*match2 && x1 == x2 {
                    *match1 = true;
                    *match2 = true;
                    matches += 1;
                    break;
                }
            }
            i1 += 1;
        }

        if prefix + matches == 0 { return 0.0; }

        let mut trans = 0;
        if matches != 0 {
            let matched1 = (slice1, matches1).zip().filter_map(|(x, m)| some_if(*m, *x));
            let matched2 = (slice2, matches2).zip().filter_map(|(x, m)| some_if(*m, *x));
            trans = (matched1, matched2).zip()
                .filter(|(x1, x2)| x1 != x2)
                .count();
        }

        let matches = (prefix + matches) as f64;
        let trans = trans as f64;
        let len1 = (prefix + len1) as f64;
        let len2 = (prefix + len2) as f64;

        (matches/len1 + matches/len2 + ((matches - trans/2.) / matches)) / 3.
    }

    /// Relative distance metric. Inversion of similarity.
    /// Reflects how far apart two slices are,
    /// ranging from 0.0 (equality) to 1.0 (nothing in common).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::slice::Jaro;
    /// # let mut jaro = Jaro::new();
    /// let dist = jaro.rel_dist(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
    /// assert!((dist - 0.06).abs() < 0.01);
    /// ```
    pub fn rel_dist<T: PartialEq + Copy>(&self, slice1: &[T], slice2: &[T]) -> f64 {
        1.0 - self.similarity(slice1, slice2)
    }
}


#[inline]
fn get_range(i1: usize, search_range: usize, len2: usize) -> Range<usize> {
    let lo = i1 - min(search_range, i1);
    let up = min(i1 + search_range + 1, len2);
    lo .. up
}

#[inline]
fn some_if<T>(cond: bool, val: T) -> Option<T> {
    if cond { Some(val) } else { None }
}


#[cfg(test)]
mod tests {
    use super::{Jaro, DEFAULT_CAPACITY};

    fn floor3(num: f64) -> f64 {
        let p = 10usize.pow(3) as f64;
        (num * p).floor() / p
    }


    #[test]
    fn equality() {
        let jaro = Jaro::new();
        let sample = [
            (1., vec![]),
            (1., vec![1]),
            (1., vec![1, 2]),
            (1., vec![1, 2, 3]),
        ];
        for (d, s) in sample.iter() {
            assert_eq!(jaro.similarity(s, s), *d);
        }
    }


    #[test]
    fn inequality() {
        let jaro = Jaro::new();
        let sample = [
            (0., vec![1],       vec![2]),
            (0., vec![1, 1],    vec![2, 2]),
            (0., vec![1, 1, 1], vec![2, 2, 2]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(jaro.similarity(s1, s2), *d);
        }
    }


    #[test]
    fn prefix() {
        let jaro = Jaro::new();
        let sample = [
            (0.916, vec![1, 2, 3, 4], vec![1, 2, 3]),
            (0.833, vec![1, 2, 3, 4], vec![1, 2]),
            (0.750, vec![1, 2, 3, 4], vec![1]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jaro.similarity(s1, s2)), *d);
            assert_eq!(floor3(jaro.similarity(s2, s1)), *d);
        }
    }


    #[test]
    fn postfix() {
        let jaro = Jaro::new();
        let sample = [
            (0.916, vec![1, 2, 3, 4], vec![2, 3, 4]),
            (0.000, vec![1, 2, 3, 4], vec![3, 4]),
            (0.000, vec![1, 2, 3, 4], vec![4]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jaro.similarity(s1, s2)), *d);
            assert_eq!(floor3(jaro.similarity(s2, s1)), *d);
        }
    }


    #[test]
    fn match_distance() {
        let jaro = Jaro::new();
        let sample = [
            (0.000, vec![1, 2, 3, 4, 5], vec![3, 0, 0, 0, 0]),
            (0.466, vec![1, 2, 3, 4, 5], vec![0, 3, 0, 0, 0]),
            (0.466, vec![1, 2, 3, 4, 5], vec![0, 0, 3, 0, 0]),
            (0.466, vec![1, 2, 3, 4, 5], vec![0, 0, 0, 3, 0]),
            (0.000, vec![1, 2, 3, 4, 5], vec![0, 0, 0, 0, 3]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jaro.similarity(s1, s2)), *d);
            assert_eq!(floor3(jaro.similarity(s2, s1)), *d);
        }
    }

    #[test]
    fn add_del_continuous() {
        let jaro = Jaro::new();
        let sample = [
            (0.933, vec![1, 2, 3, 4], vec![0, 1, 2, 3, 4]),
            (0.888, vec![1, 2, 3, 4], vec![0, 0, 1, 2, 3, 4]),
            (0.000, vec![1, 2, 3, 4], vec![0, 0, 0, 1, 2, 3, 4]),
            (0.000, vec![1, 2, 3, 4], vec![0, 0, 0, 0, 1, 2, 3, 4]),

            (0.933, vec![1, 2, 3, 4], vec![1, 2, 0, 3, 4]),
            (0.888, vec![1, 2, 3, 4], vec![1, 2, 0, 0, 3, 4]),
            (0.595, vec![1, 2, 3, 4], vec![1, 2, 0, 0, 0, 3, 4]),
            (0.583, vec![1, 2, 3, 4], vec![1, 2, 0, 0, 0, 0, 3, 4]),

            (0.933, vec![1, 2, 3, 4], vec![1, 2, 3, 4, 0]),
            (0.888, vec![1, 2, 3, 4], vec![1, 2, 3, 4, 0, 0]),
            (0.857, vec![1, 2, 3, 4], vec![1, 2, 3, 4, 0, 0, 0]),
            (0.833, vec![1, 2, 3, 4], vec![1, 2, 3, 4, 0, 0, 0, 0]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jaro.similarity(s1, s2)), *d);
            assert_eq!(floor3(jaro.similarity(s2, s1)), *d);
        }
    }


    #[test]
    fn sub_continuous() {
        let jaro = Jaro::new();
        let sample = [
            (0.833, vec![1, 2, 3, 4], vec![1, 2, 3, 0]),
            (0.666, vec![1, 2, 3, 4], vec![1, 2, 0, 0]),
            (0.500, vec![1, 2, 3, 4], vec![1, 0, 0, 0]),
            (0.000, vec![1, 2, 3, 4], vec![0, 0, 0, 0]),
            (0.500, vec![1, 2, 3, 4], vec![0, 0, 0, 4]),
            (0.666, vec![1, 2, 3, 4], vec![0, 0, 3, 4]),
            (0.833, vec![1, 2, 3, 4], vec![0, 2, 3, 4]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jaro.similarity(s1, s2)), *d);
            assert_eq!(floor3(jaro.similarity(s2, s1)), *d);
        }
    }


    #[test]
    fn add_del_intermittent() {
        let jaro = Jaro::new();
        let sample = [
            (0.933, vec![1, 2, 3, 4], vec![1, 2, 3, 4, 0]),
            (0.888, vec![1, 2, 3, 4], vec![1, 2, 3, 0, 4, 0]),
            (0.857, vec![1, 2, 3, 4], vec![1, 2, 0, 3, 0, 4, 0]),
            (0.833, vec![1, 2, 3, 4], vec![1, 0, 2, 0, 3, 0, 4, 0]),
            (0.694, vec![1, 2, 3, 4], vec![0, 1, 0, 2, 0, 3, 0, 4, 0]),
            (0.708, vec![1, 2, 3, 4], vec![0, 1, 0, 2, 0, 3, 0, 4]),
            (0.595, vec![1, 2, 3, 4], vec![0, 1, 0, 2, 0, 3, 4]),
            (0.888, vec![1, 2, 3, 4], vec![0, 1, 0, 2, 3, 4]),
            (0.933, vec![1, 2, 3, 4], vec![0, 1, 2, 3, 4]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jaro.similarity(s1, s2)), *d);
            assert_eq!(floor3(jaro.similarity(s2, s1)), *d);
        }
    }

    #[test]
    fn sub_intermittent() {
        let jaro = Jaro::new();
        let sample = [
            (0.833, vec![1, 2, 3, 4], vec![1, 2, 3, 0]),
            (0.666, vec![1, 2, 3, 4], vec![1, 0, 3, 0]),

            (0.833, vec![1, 2, 3, 4], vec![0, 2, 3, 4]),
            (0.666, vec![1, 2, 3, 4], vec![0, 2, 0, 4]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jaro.similarity(s1, s2)), *d);
            assert_eq!(floor3(jaro.similarity(s2, s1)), *d);
        }
    }


    #[test]
    fn transpose() {
        let jaro = Jaro::new();
        let sample = [
            (0.916, vec![1, 2, 3, 4], vec![2, 1, 3, 4]),
            (0.916, vec![1, 2, 3, 4], vec![1, 2, 4, 3]),
            (0.833, vec![1, 2, 3, 4], vec![2, 1, 4, 3]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jaro.similarity(s1, s2)), *d);
            assert_eq!(floor3(jaro.similarity(s2, s1)), *d);
        }
    }


    #[test]
    fn growth() {
        let jaro = Jaro::new();

        for len in 1 .. DEFAULT_CAPACITY * 2 {
            let s1: Vec<&usize> = [1].into_iter().cycle().take(len).collect();
            let s2: Vec<&usize> = [2].into_iter().cycle().take(len).collect();
            assert_eq!(jaro.similarity(&s1, &s1), 1.0);
            assert_eq!(jaro.similarity(&s1, &s2), 0.0);
        }
    }


    #[test]
    fn rel_dist() {
        let jaro = Jaro::new();
        let sample = [
            (0.000, vec![],           vec![]),
            (1.000, vec![1, 2, 3, 4], vec![]),
            (0.166, vec![1, 2, 3, 4], vec![1, 2]),
            (0.083, vec![1, 2, 3, 4], vec![2, 3, 4]),
            (0.500, vec![1, 2, 3, 4], vec![0, 0, 3, 0]),
            (0.111, vec![1, 2, 3, 4], vec![0, 0, 1, 2, 3, 4]),
            (0.333, vec![1, 2, 3, 4], vec![1, 2, 0, 0]),
            (0.166, vec![1, 2, 3, 4], vec![2, 1, 4, 3]),
            (0.500, vec![1, 2, 3, 4], vec![4, 3, 2, 1]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jaro.rel_dist(s1, s2)), *d);
            assert_eq!(floor3(jaro.rel_dist(s2, s1)), *d);
        }
    }
}

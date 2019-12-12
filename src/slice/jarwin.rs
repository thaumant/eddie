use crate::slice::Jaro;


const MAX_PREFIX: usize = 4;
const DEFAULT_SCALING: f64 = 0.1;


/// # Jaro-Winkler similarity.
///
/// Like Jaro similarity but gives a higher score to the slices
/// that start with the same sequence of items.
///
/// See [the detailed description][1].
///
/// [1]: https://en.wikipedia.org/wiki/Jaro–Winkler_distance#Jaro–Winkler_Similarity
///
/// # Usage
///
/// ```rust
/// use eddie::slice::JaroWinkler;
///
/// let jarwin = JaroWinkler::new();
/// let sim = jarwin.similarity(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
/// assert!((sim - 0.93).abs() < 0.01);
/// ```
///
/// # Complementary metrics
///
/// Relative distance:
/// ```rust
/// # let jarwin = eddie::slice::JaroWinkler::new();
/// # let s1 = &[1, 2, 3, 4, 5];
/// # let s2 = &[1, 3, 2, 4, 5];
/// let sim = jarwin.similarity(s1, s2);
/// let dist = jarwin.rel_dist(s1, s2);
/// assert_eq!(dist, 1.0 - sim);
/// ```
pub struct JaroWinkler {
    scaling: f64,
    jaro: Jaro,
}


impl JaroWinkler {
    /// Creates a new instance of JaroWinkler struct with an internal state
    /// for the metric methods to reuse.
    ///
    /// # Example
    ///
    /// ```rust
    /// use eddie::slice::JaroWinkler;
    ///
    /// let jarwin = JaroWinkler::new();
    /// ```
    pub fn new() -> JaroWinkler {
        let scaling = DEFAULT_SCALING;
        let jaro = Jaro::new();
        JaroWinkler { scaling, jaro }
    }

    /// Sets scaling factor for common prefix score boost.
    /// Default value is 0.1.
    /// Panics if it's not in range `[0.0, 0.25]`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::slice::JaroWinkler;
    /// let mut jarwin = JaroWinkler::new();
    ///
    /// let sim1 = jarwin.similarity(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
    /// jarwin.set_scaling(0.25);
    /// let sim2 = jarwin.similarity(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
    ///
    /// assert!((sim1 - 0.93).abs() < 0.01);
    /// assert!((sim2 - 0.95).abs() < 0.01);
    /// ```
    pub fn set_scaling(&mut self, scaling: f64) {
        if scaling > 0.25 {
            panic!("Scaling factor should not be greater than 0.25");
        }
        if scaling < 0.0 {
            panic!("Scaling factor should not be less than 0.0");
        }
        self.scaling = scaling;
    }

    /// Similarity metric. Reflects how close two slices are,
    /// ranging from 1.0 (equality) to 0.0 (nothing in common).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::slice::JaroWinkler;
    /// # let mut jarwin = JaroWinkler::new();
    /// let sim = jarwin.similarity(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
    /// assert!((sim - 0.93).abs() < 0.01);
    /// ```
    pub fn similarity<T: Copy + PartialEq>(&self, slice1: &[T], slice2: &[T]) -> f64 {
        let jaro_dist = self.jaro.similarity(slice1, slice2);
        if jaro_dist == 0. { return 0.; }

        let prefix_size = slice1.into_iter()
            .zip(slice2.into_iter())
            .take(MAX_PREFIX)
            .take_while(|(x1, x2)| x1 == x2)
            .count() as f64;

        jaro_dist + prefix_size * self.scaling * (1. - jaro_dist)
    }

    /// Relative distance metric. Inversion of similarity.
    /// Reflects how far apart two slices are,
    /// ranging from 0.0 (equality) to 1.0 (nothing in common).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use eddie::slice::JaroWinkler;
    /// # let mut jarwin = JaroWinkler::new();
    /// let dist = jarwin.rel_dist(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
    /// assert!((dist - 0.07).abs() < 0.01);
    /// ```
    pub fn rel_dist<T: Copy + PartialEq>(&self, slice1: &[T], slice2: &[T]) -> f64 {
        1.0 - self.similarity(slice1, slice2)
    }
}


#[cfg(test)]
mod tests {
    use super::{JaroWinkler};

    fn floor3(num: f64) -> f64 {
        let p = 10usize.pow(3) as f64;
        (num * p).floor() / p
    }


    #[test]
    fn equality() {
        let jarwin = JaroWinkler::new();
        let sample = [
            (1., vec![]),
            (1., vec![1]),
            (1., vec![1, 2]),
            (1., vec![1, 2, 3]),
        ];
        for (d, s) in sample.iter() {
            assert_eq!(jarwin.similarity(s, s), *d);
        }
    }


    #[test]
    fn inequality() {
        let jarwin = JaroWinkler::new();
        let sample = [
            (0., vec![1],       vec![2]),
            (0., vec![1, 1],    vec![2, 2]),
            (0., vec![1, 1, 1], vec![2, 2, 2]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(jarwin.similarity(s1, s2), *d);
        }
    }


    #[test]
    fn prefix() {
        let jarwin = JaroWinkler::new();
        let sample = [
            (0.941, vec![1, 2, 3, 4], vec![1, 2, 3]),
            (0.866, vec![1, 2, 3, 4], vec![1, 2]),
            (0.775, vec![1, 2, 3, 4], vec![1]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jarwin.similarity(s1, s2)), *d);
            assert_eq!(floor3(jarwin.similarity(s2, s1)), *d);
        }
    }


    #[test]
    fn postfix() {
        let jarwin = JaroWinkler::new();
        let sample = [
            (0.916, vec![1, 2, 3, 4], vec![2, 3, 4]),
            (0.000, vec![1, 2, 3, 4], vec![3, 4]),
            (0.000, vec![1, 2, 3, 4], vec![4]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jarwin.similarity(s1, s2)), *d);
            assert_eq!(floor3(jarwin.similarity(s2, s1)), *d);
        }
    }


    #[test]
    fn match_distance() {
        let jarwin = JaroWinkler::new();
        let sample = [
            (0.000, vec![1, 2, 3, 4, 5], vec![3, 0, 0, 0, 0]),
            (0.466, vec![1, 2, 3, 4, 5], vec![0, 3, 0, 0, 0]),
            (0.466, vec![1, 2, 3, 4, 5], vec![0, 0, 3, 0, 0]),
            (0.466, vec![1, 2, 3, 4, 5], vec![0, 0, 0, 3, 0]),
            (0.000, vec![1, 2, 3, 4, 5], vec![0, 0, 0, 0, 3]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jarwin.similarity(s1, s2)), *d);
            assert_eq!(floor3(jarwin.similarity(s2, s1)), *d);
        }
    }

    #[test]
    fn add_del_continuous() {
        let jarwin = JaroWinkler::new();
        let sample = [
            (0.933, vec![1, 2, 3, 4], vec![0, 1, 2, 3, 4]),
            (0.888, vec![1, 2, 3, 4], vec![0, 0, 1, 2, 3, 4]),
            (0.000, vec![1, 2, 3, 4], vec![0, 0, 0, 1, 2, 3, 4]),
            (0.000, vec![1, 2, 3, 4], vec![0, 0, 0, 0, 1, 2, 3, 4]),

            (0.946, vec![1, 2, 3, 4], vec![1, 2, 0, 3, 4]),
            (0.911, vec![1, 2, 3, 4], vec![1, 2, 0, 0, 3, 4]),
            (0.676, vec![1, 2, 3, 4], vec![1, 2, 0, 0, 0, 3, 4]),
            (0.666, vec![1, 2, 3, 4], vec![1, 2, 0, 0, 0, 0, 3, 4]),

            (0.960, vec![1, 2, 3, 4], vec![1, 2, 3, 4, 0]),
            (0.933, vec![1, 2, 3, 4], vec![1, 2, 3, 4, 0, 0]),
            (0.914, vec![1, 2, 3, 4], vec![1, 2, 3, 4, 0, 0, 0]),
            (0.900, vec![1, 2, 3, 4], vec![1, 2, 3, 4, 0, 0, 0, 0]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jarwin.similarity(s1, s2)), *d);
            assert_eq!(floor3(jarwin.similarity(s2, s1)), *d);
        }
    }


    #[test]
    fn sub_continuous() {
        let jarwin = JaroWinkler::new();
        let sample = [
            (0.883, vec![1, 2, 3, 4], vec![1, 2, 3, 0]),
            (0.733, vec![1, 2, 3, 4], vec![1, 2, 0, 0]),
            (0.550, vec![1, 2, 3, 4], vec![1, 0, 0, 0]),
            (0.000, vec![1, 2, 3, 4], vec![0, 0, 0, 0]),
            (0.500, vec![1, 2, 3, 4], vec![0, 0, 0, 4]),
            (0.666, vec![1, 2, 3, 4], vec![0, 0, 3, 4]),
            (0.833, vec![1, 2, 3, 4], vec![0, 2, 3, 4]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jarwin.similarity(s1, s2)), *d);
            assert_eq!(floor3(jarwin.similarity(s2, s1)), *d);
        }
    }


    #[test]
    fn add_del_intermittent() {
        let jarwin = JaroWinkler::new();
        let sample = [
            (0.960, vec![1, 2, 3, 4], vec![1, 2, 3, 4, 0]),
            (0.922, vec![1, 2, 3, 4], vec![1, 2, 3, 0, 4, 0]),
            (0.885, vec![1, 2, 3, 4], vec![1, 2, 0, 3, 0, 4, 0]),
            (0.850, vec![1, 2, 3, 4], vec![1, 0, 2, 0, 3, 0, 4, 0]),
            (0.694, vec![1, 2, 3, 4], vec![0, 1, 0, 2, 0, 3, 0, 4, 0]),
            (0.708, vec![1, 2, 3, 4], vec![0, 1, 0, 2, 0, 3, 0, 4]),
            (0.595, vec![1, 2, 3, 4], vec![0, 1, 0, 2, 0, 3, 4]),
            (0.888, vec![1, 2, 3, 4], vec![0, 1, 0, 2, 3, 4]),
            (0.933, vec![1, 2, 3, 4], vec![0, 1, 2, 3, 4]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jarwin.similarity(s1, s2)), *d);
            assert_eq!(floor3(jarwin.similarity(s2, s1)), *d);
        }
    }

    #[test]
    fn sub_intermittent() {
        let jarwin = JaroWinkler::new();
        let sample = [
            (0.883, vec![1, 2, 3, 4], vec![1, 2, 3, 0]),
            (0.700, vec![1, 2, 3, 4], vec![1, 0, 3, 0]),

            (0.833, vec![1, 2, 3, 4], vec![0, 2, 3, 4]),
            (0.666, vec![1, 2, 3, 4], vec![0, 2, 0, 4]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jarwin.similarity(s1, s2)), *d);
            assert_eq!(floor3(jarwin.similarity(s2, s1)), *d);
        }
    }


    #[test]
    fn transpose() {
        let jarwin = JaroWinkler::new();
        let sample = [
            (0.916, vec![1, 2, 3, 4], vec![2, 1, 3, 4]),
            (0.933, vec![1, 2, 3, 4], vec![1, 2, 4, 3]),
            (0.833, vec![1, 2, 3, 4], vec![2, 1, 4, 3]),
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
            (0.000, vec![],           vec![]),
            (1.000, vec![1, 2, 3, 4], vec![]),
            (0.133, vec![1, 2, 3, 4], vec![1, 2]),
            (0.083, vec![1, 2, 3, 4], vec![2, 3, 4]),
            (0.500, vec![1, 2, 3, 4], vec![0, 0, 3, 0]),
            (0.111, vec![1, 2, 3, 4], vec![0, 0, 1, 2, 3, 4]),
            (0.266, vec![1, 2, 3, 4], vec![1, 2, 0, 0]),
            (0.166, vec![1, 2, 3, 4], vec![2, 1, 4, 3]),
            (0.500, vec![1, 2, 3, 4], vec![4, 3, 2, 1]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(floor3(jarwin.rel_dist(s1, s2)), *d);
            assert_eq!(floor3(jarwin.rel_dist(s2, s1)), *d);
        }
    }
}

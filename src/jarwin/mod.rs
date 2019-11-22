use crate::jaro::{Jaro, State};

#[cfg(test)]
mod tests;

const MAX_PREFIX: usize = 4;
const DEFAULT_SCALING: f64 = 0.1;


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
    /// use eddie::JaroWinkler;
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
        if scaling > 0.25 {
            panic!("Scaling factor should not be greater than 0.25");
        }
        if scaling < 0.0 {
            panic!("Scaling factor should not be less than 0.0");
        }
        self.scaling = scaling;
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
        let jaro_dist = self.jaro.similarity(str1, str2);
        if jaro_dist == 0. { return 0.; }

        let State { word1, word2, .. } = &*self.jaro.state.borrow();

        let scaling = self.scaling;
        let mut prefix_size = 0.;
        for i in 0 .. min!(word1.len(), word2.len(), MAX_PREFIX) {
            if word1[i] != word2[i] { break; }
            prefix_size += 1.;
        }

        jaro_dist + prefix_size * scaling * (1. - jaro_dist)
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

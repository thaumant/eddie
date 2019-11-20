use crate::jaro::{Jaro, State};

#[cfg(test)]
mod tests;

const MAX_PREFIX: usize = 4;


pub struct JaroWinkler {
    scaling: f64,
    jaro: Jaro,
}


impl JaroWinkler {
    pub fn new() -> JaroWinkler {
        let scaling = 0.1;
        let jaro = Jaro::new();
        JaroWinkler { scaling, jaro }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let scaling = 0.1;
        let jaro = Jaro::with_capacity(capacity);
        JaroWinkler { scaling, jaro }
    }

    pub fn set_scaling(&mut self, s: f64) -> &mut Self {
        self.scaling = s;
        self
    }

    pub fn dist(&self, str1: &str, str2: &str) -> f64 {
        let jaro_dist = self.jaro.dist(str1, str2);
        if jaro_dist == 0. { return 0.; }

        let State { word1, word2, .. } = &*self.jaro.state.borrow();

        let scaling = self.scaling;
        let mut prefix_size = 0.;
        for i in 0 .. min!(word1.len(), word2.len(), MAX_PREFIX) {
            if word1[i] != word2[i] { break; }
            prefix_size += 1.;
        }

        jaro_dist - prefix_size * scaling * (1. - jaro_dist)
    }
}

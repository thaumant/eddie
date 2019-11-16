use std::cmp::min;
use crate::jaro::{Jaro, JaroState};

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

    pub fn set_scaling(&mut self, s: f64) -> &mut Self {
        self.scaling = s;
        self
    }

    pub fn dist(&self, str1: &str, str2: &str) -> f64 {
        let scaling = self.scaling;
        let jaro_dist = self.jaro.dist(str1, str2);
        let JaroState { word1, word2, .. } = &*self.jaro.state.borrow();

        let mut prefix_size = 0.;
        for i in 0..min3(word1.len(), word2.len(), MAX_PREFIX) {
            if word1[i] != word2[i] { break; }
            prefix_size += 1.;
        }

        jaro_dist - prefix_size * scaling * (1. - jaro_dist)
    }
}


#[inline]
fn min3(x1: usize, x2: usize, x3: usize) -> usize {
    min(min(x1, x2), x3)
}

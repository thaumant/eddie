use crate::jaro::{Jaro, State};

#[cfg(test)]
mod tests;

const MAX_PREFIX: usize = 4;
const DEFAULT_SCALING: f64 = 0.1;


pub struct JaroWinkler {
    scaling: f64,
    jaro: Jaro,
}


impl JaroWinkler {
    pub fn new() -> JaroWinkler {
        let scaling = DEFAULT_SCALING;
        let jaro = Jaro::new();
        JaroWinkler { scaling, jaro }
    }

    pub fn set_scaling(&mut self, scaling: f64) {
        if scaling > 0.25 {
            panic!("Scaling factor should not be greater than 0.25");
        }
        if scaling < 0.0 {
            panic!("Scaling factor should not be less than 0.0");
        }
        self.scaling = scaling;
    }

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

    pub fn rel_dist(&self, str1: &str, str2: &str) -> f64 {
        1.0 - self.similarity(str1, str2)
    }
}

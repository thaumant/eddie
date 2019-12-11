use crate::slice::Jaro;


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

    pub fn similarity<T: Copy + PartialEq>(&self, chars1: &[T], chars2: &[T]) -> f64 {
        let jaro_dist = self.jaro.similarity(chars1, chars2);
        if jaro_dist == 0. { return 0.; }

        let prefix_size = chars1.into_iter()
            .zip(chars2.into_iter())
            .take(MAX_PREFIX)
            .take_while(|(ch1, ch2)| ch1 == ch2)
            .count() as f64;

        jaro_dist + prefix_size * self.scaling * (1. - jaro_dist)
    }

    pub fn rel_dist<T: Copy + PartialEq>(&self, str1: &[T], str2: &[T]) -> f64 {
        1.0 - self.similarity(str1, str2)
    }
}

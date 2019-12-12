use std::cmp::max;


pub struct Hamming { }


impl Hamming {
    pub fn new() -> Self {
        Self { }
    }

    pub fn distance<T: PartialEq + Copy>(&self, chars1: &[T], chars2: &[T]) -> Option<usize> {
        if chars1.len() != chars2.len() { return None; }
        let dist = chars1.into_iter()
            .zip(chars2.into_iter())
            .filter(|(ch1, ch2)| ch1 != ch2)
            .count();
        Some(dist)
    }

    pub fn rel_dist<T: PartialEq + Copy>(&self, chars1: &[T], chars2: &[T]) -> Option<f64> {
        self.distance(chars1, chars2)
            .map(|dist| dist as f64 / max(chars1.len(), 1) as f64)
    }

    pub fn similarity<T: PartialEq + Copy>(&self, chars1: &[T], chars2: &[T]) -> Option<f64> {
        self.rel_dist(chars1, chars2)
            .map(|dist| 1.0 - dist)
    }
}

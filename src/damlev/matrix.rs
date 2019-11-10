use std::ops::{Index, IndexMut};
use super::constants::{MATRIX_SIZE, MAX_CHARS};


pub struct DistanceMatrix([usize; MATRIX_SIZE * MATRIX_SIZE]);


impl DistanceMatrix {
    pub fn new() -> DistanceMatrix {
        let mut matrix = DistanceMatrix([0; MATRIX_SIZE * MATRIX_SIZE]);
        for i in 0..MATRIX_SIZE {
            matrix[(i, 0)] = MAX_CHARS * 2;
            matrix[(0, i)] = MAX_CHARS * 2;
            if i == 0 { continue; }
            matrix[(i, 1)] = i - 1;
            matrix[(1, i)] = i - 1;
        }
        matrix
    }
}


impl Index<(usize, usize)> for DistanceMatrix {
    type Output = usize;
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.0[i * MATRIX_SIZE + j]
    }
}


impl IndexMut<(usize, usize)> for DistanceMatrix {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.0[i * MATRIX_SIZE + j]
    }
}

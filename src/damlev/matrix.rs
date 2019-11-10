use std::ops::{Index, IndexMut};
use super::constants::MATRIX_SIZE;


pub struct DistanceMatrix([u8; MATRIX_SIZE * MATRIX_SIZE]);


impl DistanceMatrix {
    pub fn new() -> DistanceMatrix {
        let mut matrix = DistanceMatrix([0; MATRIX_SIZE * MATRIX_SIZE]);
        for i in 0..MATRIX_SIZE {
            matrix[(i, 0)] = i as u8;
            matrix[(0, i)] = i as u8;
        }
        matrix
    }
}


impl Index<(usize, usize)> for DistanceMatrix {
    type Output = u8;
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.0[i * MATRIX_SIZE + j]
    }
}


impl IndexMut<(usize, usize)> for DistanceMatrix {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.0[i * MATRIX_SIZE + j]
    }
}

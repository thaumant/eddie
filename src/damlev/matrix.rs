
pub struct Matrix {
    size: usize,
    raw: Vec<u8>,
}

impl Matrix {
    pub fn new(size: usize) -> Matrix {
        let raw = vec![0; size * size];
        let mut matrix = Matrix { size, raw };
        matrix.init();
        matrix
    }

    pub fn grow(&mut self, size: usize) {
        if size <= self.size {
            return;
        }
        let size = size + size / 2;
        self.raw.resize(size * size, 0);
        self.size = size;
        self.init();
    }

    pub fn init(&mut self) {
        if self.size == 0 { return; }
        unsafe {
            for i in 0..self.size {
                *self.ix(i, 0) = self.size as u8;
                *self.ix(0, i) = self.size as u8;
            }
            for i in 1..self.size {
                *self.ix(i, 1) = i as u8 - 1;
                *self.ix(1, i) = i as u8 - 1;
            }
        }
    }

    #[inline]
    pub unsafe fn ix(&mut self, i: usize, j: usize) -> &mut u8 {
        self.raw.get_unchecked_mut(i * self.size + j)
    }
}

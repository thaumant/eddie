#![macro_use]

#[cfg(test)]
mod tests;

use std::ptr;
use std::cmp::max;


macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::min($x, min!($($z),*)));
}


macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::max($x, max!($($z),*)));
}


pub trait Rewrite<T> {
    fn rewrite_with<It: Iterator<Item=T>>(&mut self, it: It);
}


impl<T> Rewrite<T> for Vec<T> {
    fn rewrite_with<It: Iterator<Item=T>>(&mut self, it: It) {
        self.clear();
        let mut i = 0;
        let mut p = self.as_mut_ptr();
        let mut cap = self.capacity() as isize;

        for c in it.into_iter() {
            if i >= cap {
                self.reserve(max(cap * 2, 1) as usize);
                cap = self.capacity() as isize;
                p = self.as_mut_ptr();
            }
            unsafe {
                ptr::write(p.offset(i), c);
                i += 1;
            }
        }
        unsafe {
            self.set_len(i as usize);
        }
    }
}

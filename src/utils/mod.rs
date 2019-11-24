#![macro_use]

#[cfg(test)]
mod tests;

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
    fn rewrite_with<Items: Iterator<Item=T>>(&mut self, items: Items);
}


impl<T> Rewrite<T> for Vec<T> {
    fn rewrite_with<Items: Iterator<Item=T>>(&mut self, items: Items) {
        self.clear();
        let mut cap = self.capacity();
        let mut i = 0;
        for item in items {
            if i >= cap {
                self.reserve(max(cap * 2, 1));
                cap = self.capacity();
            }
            unsafe { *self.get_unchecked_mut(i) = item; }
            i += 1;
        }
        unsafe { self.set_len(i); }
    }
}

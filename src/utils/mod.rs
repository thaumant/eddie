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


pub fn write_str(s: &str, vec: &mut Vec<char>) -> () {
    unsafe {
        let len = vec.len();
        let mut i = 0isize;
        let mut p = vec.as_mut_ptr();
        let mut cap = vec.capacity() as isize;
        for c in s.chars() {
            if i >= cap {
                vec.reserve(max(cap * 2, 1) as usize - len);
                cap = vec.capacity() as isize;
                p = vec.as_mut_ptr();
            }
            ptr::write(p.offset(i), c);
            i += 1;
        }
        vec.set_len(i as usize);
    }
}

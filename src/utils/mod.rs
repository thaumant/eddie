#![macro_use]

#[cfg(test)]
mod tests;

pub mod zip;
pub mod buffer;

use std::cmp::min;


macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::min($x, min!($($z),*)));
}


macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::max($x, max!($($z),*)));
}


pub fn common_prefix_size<T: Copy + PartialEq>(slice1: &[T], slice2: &[T]) -> usize {
    slice1.into_iter().zip(slice2.into_iter())
        .take_while(|(ch1, ch2)| ch1 == ch2)
        .count()
}


pub fn common_affix_sizes<T: Copy + PartialEq>(slice1: &[T], slice2: &[T]) -> (usize, usize) {
    let min_len = min(slice1.len(), slice2.len());
    let prefix = common_prefix_size(slice1, slice2);
    let suffix = slice1.into_iter().rev().zip(slice2.into_iter().rev())
        .take(min_len - prefix)
        .take_while(|(item1, item2)| item1 == item2)
        .count();
    (prefix, suffix)
}

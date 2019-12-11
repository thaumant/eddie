mod matrix;

use std::cmp::max;
use std::collections::BTreeMap;
use std::cell::RefCell;
use crate::utils::common_affix_sizes;
use matrix::DistMatrix;


const DEFAULT_CAPACITY: usize = 20;


pub struct DamerauLevenshtein<T: PartialEq + Copy + Ord> {
  dists:   RefCell<DistMatrix>,
  last_i1: RefCell<BTreeMap<T, usize>>,
}


impl<T: PartialEq + Copy + Ord> DamerauLevenshtein<T> {
  pub fn new() -> Self {
      let dists   = RefCell::new(DistMatrix::new(DEFAULT_CAPACITY + 2));
      let last_i1 = RefCell::new(BTreeMap::new());
      Self { dists, last_i1 }
  }

  pub fn distance(&self, chars1: &[T], chars2: &[T]) -> usize {
      let dists   = &mut *self.dists.borrow_mut();
      let last_i1 = &mut *self.last_i1.borrow_mut();

      last_i1.clear();

      let (prefix, postfix) = common_affix_sizes(chars1, chars2);
      let chars1 = { let len = chars1.len(); &chars1[prefix .. len - postfix] };
      let chars2 = { let len = chars2.len(); &chars2[prefix .. len - postfix] };
      let len1 = chars1.len();
      let len2 = chars2.len();

      dists.grow(max(len1 + 2, len2 + 2));

      for (i1, &char1) in chars1.iter().enumerate() {
          let mut l2 = 0;

          for (i2, &char2) in chars2.iter().enumerate() {
              let l1 = *last_i1.get(&char2).unwrap_or(&0);

              unsafe {
                  dists.set(i1 + 2, i2 + 2, min!(
                      dists.get(i1 + 2, i2 + 1) + 1,
                      dists.get(i1 + 1, i2 + 2) + 1,
                      dists.get(i1 + 1, i2 + 1) + (char1 != char2) as u8,
                      dists.get(l1, l2) + (i1 - l1) as u8 + (i2 - l2) as u8 + 1
                  ));
              }

              if char1 == char2 { l2 = i2 + 1; }
          }
          last_i1.insert(char1, i1 + 1);
      }

      let dist = unsafe { dists.get(len1 + 1, len2 + 1) };
      dist as usize
  }

  pub fn rel_dist(&self, chars1: &[T], chars2: &[T]) -> f64 {
      let dist = self.distance(chars1, chars2);
      let len = max!(1, chars1.len(), chars2.len());
      dist as f64 / len as f64
  }

  pub fn similarity(&self, chars1: &[T], chars2: &[T]) -> f64 {
      1.0 - self.rel_dist(chars1, chars2)
  }
}

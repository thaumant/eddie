use crate::utils::{common_affix_sizes, Buffer};


const DEFAULT_CAPACITY: usize = 20;


pub struct Levenshtein {
    dists: Buffer<u8>,
}


impl Levenshtein {
    pub fn new() -> Self {
        Self { dists: Buffer::with_capacity(DEFAULT_CAPACITY + 1) }
    }

    pub fn distance<T: PartialEq + Copy>(&self, chars1: &[T], chars2: &[T]) -> usize {
        let dists = &mut *self.dists.store(1 .. chars2.len() as u8 + 2).borrow_mut();

        let (prefix, postfix) = common_affix_sizes(chars1, chars2);
        let chars1 = { let l = chars1.len(); &chars1[prefix .. l - postfix] };
        let chars2 = { let l = chars2.len(); &chars2[prefix .. l - postfix] };

        let mut dist = chars2.len() as u8;
        let mut prev;

        for (i1, char1) in chars1.into_iter().enumerate() {
            dist = i1 as u8 + 1;
            prev = i1 as u8;

            for (char2, prev2) in chars2.into_iter().zip(dists.into_iter()) {
                dist = min!(
                    dist + 1,
                    *prev2 + 1,
                    prev + (char1 != char2) as u8
                );
                prev = *prev2;
                *prev2 = dist;
            }
        }

        dist as usize
    }

    pub fn rel_dist<T: PartialEq + Copy>(&self, chars1: &[T], chars2: &[T]) -> f64 {
        let dist = self.distance(chars1, chars2);
        let len = max!(1, chars1.len(), chars2.len());
        dist as f64 / len as f64
    }

    pub fn similarity<T: PartialEq + Copy>(&self, chars1: &[T], chars2: &[T]) -> f64 {
        1.0 - self.rel_dist(chars1, chars2)
    }
}

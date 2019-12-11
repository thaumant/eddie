use std::cmp::min;
use std::ops::Range;
use std::cell::RefCell;
use crate::utils::common_prefix_size;
use crate::utils::zip::Zippable;


const DEFAULT_CAPACITY: usize = 25;


pub struct Jaro {
    matches1: RefCell<Vec<bool>>,
    matches2: RefCell<Vec<bool>>,
}


impl Jaro {
    pub fn new() -> Self {
        Self {
            matches1: RefCell::new(Vec::with_capacity(DEFAULT_CAPACITY)),
            matches2: RefCell::new(Vec::with_capacity(DEFAULT_CAPACITY)),
        }
    }

    pub fn similarity<T: PartialEq + Copy>(&self, chars1: &[T], chars2: &[T]) -> f64 {
        match (chars1.len(), chars2.len()) {
            (0, 0) => { return 1.0; }
            (_, 0) => { return 0.0; }
            (0, _) => { return 0.0; }
            (_, _) => { }
        }

        let prefix = common_prefix_size(chars1, chars2);
        let chars1 = &chars1[prefix..];
        let chars2 = &chars2[prefix..];

        let matches1 = &mut *self.matches1.borrow_mut();
        let matches2 = &mut *self.matches2.borrow_mut();
        matches1.clear();
        matches2.clear();
        matches1.resize(chars1.len(), false);
        matches2.resize(chars2.len(), false);

        let mut matches = 0;

        let len1 = chars1.len();
        let len2 = chars2.len();
        let i2_range = max!(1, (len1 + prefix) / 2, (len2 + prefix) / 2) - 1;
        let mut i1 = 0;

        for (ch1, match1) in (chars1, &mut matches1[..]).zip() {
            let rng = get_range(i1, i2_range, len2);
            if rng.start >= rng.end { continue; }
            for (ch2, match2) in (&chars2[rng.clone()], &mut matches2[rng]).zip() {
                if !*match2 && ch1 == ch2 {
                    *match1 = true;
                    *match2 = true;
                    matches += 1;
                    break;
                }
            }
            i1 += 1;
        }

        if prefix + matches == 0 { return 0.0; }

        let mut trans = 0;
        if matches != 0 {
            let matched1 = (chars1, matches1).zip().filter_map(|(c, m)| some_if(*m, *c));
            let matched2 = (chars2, matches2).zip().filter_map(|(c, m)| some_if(*m, *c));
            trans = (matched1, matched2).zip()
                .filter(|(c1, c2)| c1 != c2)
                .count();
        }

        let matches = (prefix + matches) as f64;
        let trans = trans as f64;
        let len1 = (prefix + len1) as f64;
        let len2 = (prefix + len2) as f64;

        (matches/len1 + matches/len2 + ((matches - trans/2.) / matches)) / 3.
    }

    pub fn rel_dist<T: PartialEq + Copy>(&self, chars1: &[T], chars2: &[T]) -> f64 {
        1.0 - self.similarity(chars1, chars2)
    }
}


#[inline]
fn get_range(i1: usize, search_range: usize, len2: usize) -> Range<usize> {
    let lo = i1 - min(search_range, i1);
    let up = min(i1 + search_range + 1, len2);
    lo .. up
}

#[inline]
fn some_if<T>(cond: bool, val: T) -> Option<T> {
    if cond { Some(val) } else { None }
}

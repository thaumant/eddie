#![macro_use]

#[cfg(test)]
mod tests;


macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::min($x, min!($($z),*)));
}


macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::max($x, max!($($z),*)));
}


pub fn write_str(s: &str, v: &mut Vec<char>) -> () {
    if s.len() == 0 {
        v.clear();
        return;
    }
    let len = v.len();
    let mut i = 0;
    for c in s.chars() {
        if i < len {
            unsafe { *v.get_unchecked_mut(i) = c;  }
        } else {
            v.push(c);
        }
        i += 1;
    }
    v.resize(i, '\0');
}

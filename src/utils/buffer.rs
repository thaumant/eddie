use std::cmp::max;
use std::cell::RefCell;


pub struct Buffer<T: Copy> {
    cell: RefCell<Vec<T>>,
}


impl<T: Copy> Buffer<T> {
    pub fn with_capacity(cap: usize) -> Self {
        Self { cell: RefCell::new(Vec::with_capacity(cap)) }
    }

    pub fn store<Iter: Iterator<Item=T>>(&self, iter: Iter) -> &RefCell<Vec<T>> {
        {
            let buf = &mut *self.cell.borrow_mut();
            buf.clear();
            let mut cap = buf.capacity();
            let mut i = 0;
            for item in iter {
                if i >= cap {
                    buf.reserve(max(cap * 2, 1));
                    cap = buf.capacity();
                }
                unsafe { *buf.get_unchecked_mut(i) = item; }
                i += 1;
            }
            unsafe { buf.set_len(i); }
        }
        &self.cell
    }
}


#[cfg(test)]
mod tests {
    use super::Buffer;

    #[test]
    fn store_empty() {
        let mut sample = [
            ("", 0, vec![], 0, 0),
            ("", 1, vec![], 0, 1),
            ("", 2, vec![], 0, 2),
        ];
        for (input, cap, expected, len, cap_expected) in &mut sample {
            let buf = Buffer::with_capacity(*cap);
            let vec = &*buf.store(input.chars()).borrow();
            assert_eq!(vec, expected);
            assert_eq!(vec.len(), *len);
            assert_eq!(vec.capacity(), *cap_expected);
        }
    }

    #[test]
    fn store_nonempty() {
        let mut sample = [
            ("foo", 0, vec!['f', 'o', 'o'], 3, 4),
            ("foo", 1, vec!['f', 'o', 'o'], 3, 4),
            ("foo", 2, vec!['f', 'o', 'o'], 3, 4),
            ("foo", 3, vec!['f', 'o', 'o'], 3, 3),
            ("foo", 4, vec!['f', 'o', 'o'], 3, 4),
        ];
        for (input, cap, expected, len, cap_expected) in &mut sample {
            let buf = Buffer::with_capacity(*cap);
            let vec = &*buf.store(input.chars()).borrow();
            assert_eq!(vec, expected);
            assert_eq!(vec.len(), *len);
            assert_eq!(vec.capacity(), *cap_expected);
        }
    }

    #[test]
    fn store_range() {
        let buf = Buffer::with_capacity(0);
        let vec = &*buf.store(0..5).borrow();
        assert_eq!(vec, &vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn capacity_regress() {
        for len in 0..33 {
            for cap in 0..33 {
                let str1: String = "abcdefg".chars().cycle().take(len).collect();
                let str2: String = "abcdefg".chars().cycle().take(len).collect();

                let vec1: &mut Vec<char> = &mut Vec::with_capacity(cap);
                for chr1 in str1.chars() { vec1.push(chr1); }

                let buf2 = Buffer::with_capacity(cap);
                let vec2 = &*buf2.store(str2.chars()).borrow();

                assert_eq!(vec1, vec2);
                assert_eq!(str1, str2);
                assert_eq!(vec1.capacity(), vec2.capacity());
            }
        }
    }
}
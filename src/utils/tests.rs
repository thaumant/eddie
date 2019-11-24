use super::Rewrite;

#[test]
fn rewrite_chars_empty() {
    let mut sample = [
        ("", vec!['0'; 0], vec![], 0, 0),
        ("", vec!['0'; 1], vec![], 0, 1),
        ("", vec!['0'; 2], vec![], 0, 2),
    ];
    for (input, vec, expected, len, cap) in &mut sample {
        vec.rewrite_with(input.chars());
        assert_eq!(vec, expected);
        assert_eq!(vec.len(), *len);
        assert_eq!(vec.capacity(), *cap);
    }
}

#[test]
fn rewrite_chars_nonempty() {
    let mut sample = [
        ("foo", vec!['0'; 0], vec!['f', 'o', 'o'], 3, 4),
        ("foo", vec!['0'; 1], vec!['f', 'o', 'o'], 3, 4),
        ("foo", vec!['0'; 2], vec!['f', 'o', 'o'], 3, 4),
        ("foo", vec!['0'; 3], vec!['f', 'o', 'o'], 3, 3),
        ("foo", vec!['0'; 4], vec!['f', 'o', 'o'], 3, 4),
    ];
    for (input, vec, expected, len, cap) in &mut sample {
        vec.rewrite_with(input.chars());
        assert_eq!(vec, expected);
        assert_eq!(vec.len(), *len);
        assert_eq!(vec.capacity(), *cap);
    }
}

#[test]
fn rewrite_chars_regress() {
    for len in 0..33 {
        for cap in 0..33 {
            let mut v1: Vec<char> = Vec::with_capacity(cap);
            let mut v2: Vec<char> = Vec::with_capacity(cap);

            let s1: String = "abcdefg".chars().cycle().take(len).collect();
            let s2: String = "abcdefg".chars().cycle().take(len).collect();

            for c in s1.chars() { v1.push(c); }
            v2.rewrite_with(s2.chars());

            assert_eq!(v1, v2);
            assert_eq!(s1, s2);
            assert_eq!(v1.capacity(), v2.capacity());
        }
    }
}

#[test]
fn rewrite_range() {
    let mut vec = Vec::new();
    vec.rewrite_with(0..5);
    assert_eq!(vec, vec![0, 1, 2, 3, 4]);
}

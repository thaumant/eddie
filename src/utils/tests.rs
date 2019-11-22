use super::write_str;

#[test]
fn write_str_empty() {
    let mut sample = [
        ("", vec!['0'; 0], vec![], 0, 0),
        ("", vec!['0'; 1], vec![], 0, 1),
        ("", vec!['0'; 2], vec![], 0, 2),
    ];
    for (input, store, expected, len, cap) in &mut sample {
        write_str(input, store);
        assert_eq!(store, expected);
        assert_eq!(store.len(), *len);
        assert_eq!(store.capacity(), *cap);
    }
}

#[test]
fn write_str_nonempty() {
    let mut sample = [
        ("foo", vec!['0'; 0], vec!['f', 'o', 'o'], 3, 4),
        ("foo", vec!['0'; 1], vec!['f', 'o', 'o'], 3, 4),
        ("foo", vec!['0'; 2], vec!['f', 'o', 'o'], 3, 4),
        ("foo", vec!['0'; 3], vec!['f', 'o', 'o'], 3, 3),
        ("foo", vec!['0'; 4], vec!['f', 'o', 'o'], 3, 4),
    ];
    for (input, store, expected, len, cap) in &mut sample {
        write_str(input, store);
        assert_eq!(store, expected);
        assert_eq!(store.len(), *len);
        assert_eq!(store.capacity(), *cap);
    }
}

#[test]
fn write_str_regress() {
    for len in 0..33 {
        for cap in 0..33 {
            let mut v1: Vec<char> = Vec::with_capacity(cap);
            let mut v2: Vec<char> = Vec::with_capacity(cap);

            let s1: String = "abcdefg".chars().cycle().take(len).collect();
            let s2: String = "abcdefg".chars().cycle().take(len).collect();

            for c in s1.chars() { v1.push(c); }
            write_str(&s2, &mut v2);

            assert_eq!(v1, v2);
            assert_eq!(s1, s2);
            assert_eq!(v1.capacity(), v2.capacity());
        }
    }
}

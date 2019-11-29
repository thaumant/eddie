use super::{Rewrite, common_affix_sizes};


// Rewrite tests
// ------------------------------------------------

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


// common_affix_sizes tests
// ------------------------------------------------

fn vec(s: &str) -> Vec<char> {
    s.chars().collect()
}


#[test]
fn affix_sizes_empty() {
    let v1 = vec("");
    let sample = [
        ((0, 0), vec("")),
        ((0, 0), vec("m")),
        ((0, 0), vec("ma")),
        ((0, 0), vec("mai")),
        ((0, 0), vec("mail")),
        ((0, 0), vec("mailb")),
        ((0, 0), vec("mailbo")),
        ((0, 0), vec("mailbox")),
    ];
    for (expected, v2) in &sample {
        assert_eq!(common_affix_sizes(&v1, &v2), *expected);
        assert_eq!(common_affix_sizes(&v2, &v1), *expected);
    }
}

#[test]
fn affix_sizes_equal() {
    let sample = [
        ((1, 0), vec("m")),
        ((2, 0), vec("ma")),
        ((3, 0), vec("mai")),
        ((4, 0), vec("mail")),
        ((5, 0), vec("mailb")),
        ((6, 0), vec("mailbo")),
        ((7, 0), vec("mailbox")),
    ];
    for (expected, v1) in &sample {
        assert_eq!(common_affix_sizes(&v1, &v1), *expected);
    }
}

#[test]
fn affix_sizes_nonequal() {
    let sample = [
        ((0, 0), vec("m"),    vec("b")),
        ((0, 0), vec("ma"),   vec("bo")),
        ((0, 0), vec("mai"),  vec("bol")),
        ((0, 0), vec("mail"), vec("bolt")),
    ];
    for (expected, v1, v2) in &sample {
        assert_eq!(common_affix_sizes(&v1, &v2), *expected);
    }
}

#[test]
fn affix_sizes_prefix() {
    let sample = [
        ((1, 0), vec("mailbox"), vec("m")),
        ((2, 0), vec("mailbox"), vec("ma")),
        ((3, 0), vec("mailbox"), vec("mai")),
        ((4, 0), vec("mailbox"), vec("mail")),
        ((5, 0), vec("mailbox"), vec("mailb")),
        ((6, 0), vec("mailbox"), vec("mailbo")),
    ];
    for (expected, v1, v2) in &sample {
        assert_eq!(common_affix_sizes(&v1, &v2), *expected);
        assert_eq!(common_affix_sizes(&v2, &v1), *expected);
    }
}

#[test]
fn affix_sizes_suffix() {
    let sample = [
        ((0, 1), vec("mailbox"), vec("x")),
        ((0, 2), vec("mailbox"), vec("ox")),
        ((0, 3), vec("mailbox"), vec("box")),
        ((0, 4), vec("mailbox"), vec("lbox")),
        ((0, 5), vec("mailbox"), vec("ilbox")),
        ((0, 6), vec("mailbox"), vec("ailbox")),
    ];
    for (expected, v1, v2) in &sample {
        assert_eq!(common_affix_sizes(&v1, &v2), *expected);
        assert_eq!(common_affix_sizes(&v2, &v1), *expected);
    }
}

#[test]
fn affix_sizes_sub() {
    let sample = [
        ((3, 3), vec("mailbox"), vec("mai_box")),
        ((2, 3), vec("mailbox"), vec("ma__box")),
        ((2, 2), vec("mailbox"), vec("ma___ox")),
        ((1, 2), vec("mailbox"), vec("m____ox")),
        ((1, 1), vec("mailbox"), vec("m_____x")),
    ];
    for (expected, v1, v2) in &sample {
        assert_eq!(common_affix_sizes(&v1, &v2), *expected);
        assert_eq!(common_affix_sizes(&v2, &v1), *expected);
    }
}

#[test]
fn affix_sizes_add_del() {
    let sample = [
        ((3, 4), vec("mailbox"), vec("mai_lbox")),
        ((3, 3), vec("mailbox"), vec("mai_l_box")),
        ((2, 3), vec("mailbox"), vec("ma_i_l_box")),
        ((2, 2), vec("mailbox"), vec("ma_i_l_b_ox")),
        ((1, 2), vec("mailbox"), vec("m_a_i_l_b_ox")),
        ((1, 1), vec("mailbox"), vec("m_a_i_l_b_o_x")),
    ];
    for (expected, v1, v2) in &sample {
        assert_eq!(common_affix_sizes(&v1, &v2), *expected);
        assert_eq!(common_affix_sizes(&v2, &v1), *expected);
    }
}

#[test]
fn affix_size_utf_multibyte() {
    let sample = [
        ((4, 0), vec("もしもし"), vec("もしもしし")),
        ((4, 0), vec("もしもし"), vec("もしもし")),
        ((2, 1), vec("もしもし"), vec("もしまし")),
        ((2, 1), vec("もしもし"), vec("もしし")),
    ];
    for (expected, v1, v2) in &sample {
        assert_eq!(common_affix_sizes(&v1, &v2), *expected);
        assert_eq!(common_affix_sizes(&v2, &v1), *expected);
    }
}

#[test]
fn affix_size_mixed() {
    let sample = [
        ((0, 0), vec("ca"),        vec("abc")),
        ((2, 0), vec("a tc"),      vec("a cat")),
        ((1, 1), vec("a cat"),     vec("an abct")),
        ((0, 1), vec("crate"),     vec("trace")),
        ((0, 5), vec("captain"),   vec("ptain")),
        ((1, 2), vec("dwayne"),    vec("duane")),
        ((3, 1), vec("martha"),    vec("marhta")),
        ((0, 0), vec("kitten"),    vec("sitting")),
        ((0, 0), vec("mailbox"),   vec("boxmail")),
        ((0, 3), vec("mailbox"),   vec("alimbox")),
        ((2, 0), vec("dixon"),     vec("dicksonx")),
        ((0, 8), vec("jellyfish"), vec("smellyfish")),
    ];
    for (expected, v1, v2) in &sample {
        assert_eq!(common_affix_sizes(&v1, &v2), *expected);
        assert_eq!(common_affix_sizes(&v2, &v1), *expected);
    }
}

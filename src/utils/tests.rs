use super::{common_affix_sizes};


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

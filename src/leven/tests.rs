use super::{Levenshtein, DEFAULT_CAPACITY};

fn floor3(num: f64) -> f64 {
    let p = 10usize.pow(3) as f64;
    (num * p).floor() / p
}

#[test]
fn equality() {
    let lev = Levenshtein::new();
    let sample = [
        "",
        "m",
        "ma",
        "mai",
        "mail",
        "mailb",
        "mailbo",
        "mailbox",
    ];
    for s in sample.iter() {
        assert_eq!(lev.distance(s, s), 0);
    }
}

#[test]
fn prefix() {
    let lev = Levenshtein::new();
    let sample = [
        (1, "mailbox", "mailbo"),
        (2, "mailbox", "mailb"),
        (3, "mailbox", "mail"),
        (4, "mailbox", "mai"),
        (5, "mailbox", "ma"),
        (6, "mailbox", "m"),
        (7, "mailbox", ""),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(lev.distance(s1, s2), *d);
        assert_eq!(lev.distance(s2, s1), *d);
    }
}

#[test]
fn add_del_continuous() {
    let lev = Levenshtein::new();
    let sample = [
        (1, "mailbox", "_mailbox"),
        (2, "mailbox", "__mailbox"),
        (3, "mailbox", "___mailbox"),
        (4, "mailbox", "____mailbox"),

        (1, "mailbox", "mail_box"),
        (2, "mailbox", "mail__box"),
        (3, "mailbox", "mail___box"),
        (4, "mailbox", "mail____box"),

        (1, "mailbox", "mailbox_"),
        (2, "mailbox", "mailbox__"),
        (3, "mailbox", "mailbox___"),
        (4, "mailbox", "mailbox____"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(lev.distance(s1, s2), *d);
        assert_eq!(lev.distance(s2, s1), *d);
    }
}

#[test]
fn sub_continuous() {
    let lev = Levenshtein::new();
    let sample = [
        (1, "mailbox", "_ailbox"),
        (2, "mailbox", "__ilbox"),
        (3, "mailbox", "___lbox"),
        (4, "mailbox", "____box"),

        (1, "mailbox", "mai_box"),
        (2, "mailbox", "mai__ox"),
        (3, "mailbox", "ma___ox"),
        (4, "mailbox", "ma____x"),

        (1, "mailbox", "mailbo_"),
        (2, "mailbox", "mailb__"),
        (3, "mailbox", "mail___"),
        (4, "mailbox", "mai____"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(lev.distance(s1, s2), *d);
    }
}

#[test]
fn add_del_intermittent() {
    let lev = Levenshtein::new();
    let sample = [
        (1, "mailbox", "_mailbox"),
        (2, "mailbox", "_m_ailbox"),
        (3, "mailbox", "_m_a_ilbox"),
        (4, "mailbox", "_m_a_i_lbox"),

        (1, "mailbox", "mailbox_"),
        (2, "mailbox", "mailbo_x_"),
        (3, "mailbox", "mailb_o_x_"),
        (4, "mailbox", "mail_b_o_x_"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(lev.distance(s1, s2), *d);
        assert_eq!(lev.distance(s2, s1), *d);
    }
}

#[test]
fn sub_intermittent() {
    let lev = Levenshtein::new();
    let sample = [
        (1, "mailbox", "_ailbox"),
        (2, "mailbox", "_a_lbox"),
        (3, "mailbox", "_a_l_ox"),

        (1, "mailbox", "mailbo_"),
        (2, "mailbox", "mail_o_"),
        (3, "mailbox", "ma_l_o_"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(lev.distance(s1, s2), *d);
    }
}

#[test]
fn mixed() {
    let lev = Levenshtein::new();
    let sample = [
        (3, "ca",        "abc"),
        (3, "a tc",      "a cat"),
        (4, "a cat",     "an abct"),
        (2, "crate",     "trace"),
        (2, "captain",   "ptain"),
        (2, "dwayne",    "duane"),
        (2, "martha",    "marhta"),
        (3, "kitten",    "sitting"),
        (6, "mailbox",   "boxmail"),
        (3, "mailbox",   "alimbox"),
        (4, "dixon",     "dicksonx"),
        (2, "jellyfish", "smellyfish"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(lev.distance(s1, s2), *d);
        assert_eq!(lev.distance(s2, s1), *d);
    }
}

#[test]
fn growth() {
    let lev = Levenshtein::new();

    for len in 0 .. DEFAULT_CAPACITY * 2 {
        let s1 = &"a".repeat(len);
        let s2 = &"b".repeat(len);
        assert_eq!(lev.distance(s1, s1), 0);
        assert_eq!(lev.distance(s1, s2), len);
    }
}

#[test]
fn utf_multibyte() {
    let lev = Levenshtein::new();
    let s1 = "もしもし";
    let sample= [
        (1, "もしもしし"),
        (0, "もしもし"),
        (1, "もしまし"),
        (1, "もしし"),
        (2, "もし"),
        (3, "し"),
        (4, ""),
    ];
    for (d, s2) in sample.iter() {
        assert_eq!(lev.distance(s1, s2), *d);
    }
}

#[test]
fn rel_dist() {
    let lev = Levenshtein::new();
    let sample = [
        (0.000, "",        ""),
        (1.000, "mailbox", ""),
        (0.428, "mailbox", "mail"),
        (0.222, "mailbox", "mail__box"),
        (0.571, "mailbox", "____box"),
        (0.857, "mailbox", "boxmail"),
        (0.571, "mailbox", "amliobx"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(floor3(lev.rel_dist(s1, s2)), *d);
        assert_eq!(floor3(lev.rel_dist(s2, s1)), *d);
    }
}

#[test]
fn similarity() {
    let lev = Levenshtein::new();
    let sample = [
        (1.000, "",        ""),
        (0.000, "mailbox", ""),
        (0.571, "mailbox", "mail"),
        (0.777, "mailbox", "mail__box"),
        (0.428, "mailbox", "____box"),
        (0.142, "mailbox", "boxmail"),
        (0.428, "mailbox", "amliobx"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(floor3(lev.similarity(s1, s2)), *d);
        assert_eq!(floor3(lev.similarity(s2, s1)), *d);
    }
}

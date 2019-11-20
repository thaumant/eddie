use super::JaroWinkler;


fn floor3(num: f64) -> f64 {
    let p = 10usize.pow(3) as f64;
    (num * p).floor() / p
}


#[test]
fn equality() {
    let jarwin = JaroWinkler::new();
    let sample = [
        (1., ""),
        (1., "m"),
        (1., "ma"),
        (1., "mai"),
        (1., "mail"),
        (1., "mailb"),
        (1., "mailbo"),
        (1., "mailbox"),
    ];
    for (d, s) in &sample {
        assert_eq!(jarwin.similarity(s, s), *d);
    }
}


#[test]
fn inequality() {
    let jarwin = JaroWinkler::new();
    let sample = [
        (0., "a",     "b"),
        (0., "aa",    "bb"),
        (0., "aaa",   "bbb"),
        (0., "aaaa",  "bbbb"),
        (0., "aaaaa", "bbbbb"),
    ];
    for (d, s1, s2) in &sample {
        assert_eq!(jarwin.similarity(s1, s2), *d);
        assert_eq!(jarwin.similarity(s2, s1), *d);
    }
}


#[test]
fn prefix() {
    let jarwin = JaroWinkler::new();
    let sample = [
        (0.000, "mailbox", ""),
        (0.685, "mailbox", "m"),
        (0.714, "mailbox", "ma"),
        (0.752, "mailbox", "mai"),
        (0.799, "mailbox", "mail"),
        (0.866, "mailbox", "mailb"),
        (0.933, "mailbox", "mailbo"),
    ];
    for (d, s1, s2) in &sample {
        assert_eq!(floor3(jarwin.similarity(s1, s2)), *d);
        assert_eq!(floor3(jarwin.similarity(s2, s1)), *d);
    }
}


#[test]
fn postfix() {
    let jarwin = JaroWinkler::new();
    let sample = [
        (0.952, "mailbox", "ailbox"),
        (0.904, "mailbox", "ilbox"),
        (0.000, "mailbox", "lbox"),
        (0.000, "mailbox", "box"),
        (0.000, "mailbox", "ox"),
        (0.000, "mailbox", "x"),
    ];
    for (d, s1, s2) in &sample {
        assert_eq!(floor3(jarwin.similarity(s1, s2)), *d);
        assert_eq!(floor3(jarwin.similarity(s2, s1)), *d);
    }
}


#[test]
fn scaling() {
    let jarwin = JaroWinkler::new();
    let sample = [
        (0.904, "_ailbox", "-ailbox"),
        (0.895, "m_ilbox", "m-ilbox"),
        (0.885, "ma_lbox", "ma-lbox"),
        (0.876, "mai_box", "mai-box"),
        (0.866, "mail_ox", "mail-ox"),
        (0.866, "mailb_x", "mailb-x"),
        (0.866, "mailbo_", "mailbo-"),
    ];
    for (d, s1, s2) in &sample {
        assert_eq!(floor3(jarwin.similarity(s1, s2)), *d);
        assert_eq!(floor3(jarwin.similarity(s2, s1)), *d);
    }
}


#[test]
fn mixed() {
    let jarwin = JaroWinkler::new();
    let sample = [
        (0.804, "dwayne",    "duane"),
        (0.927, "martha",    "marhta"),
        (0.720, "dixon",     "dicksonx"),
        (0.896, "jellyfish", "smellyfish"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(floor3(jarwin.similarity(s1, s2)), *d);
        assert_eq!(floor3(jarwin.similarity(s2, s1)), *d);
    }
}


#[test]
fn rel_dist() {
    let jarwin = JaroWinkler::new();
    let sample = [
        (0.000, "",        ""),
        (1.000, "mailbox", ""),
        (0.200, "mailbox", "mail"),
        (0.095, "mailbox", "ilbox"),
        (0.104, "m_ilbox", "m-ilbox"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(floor3(jarwin.rel_dist(s1, s2)), *d);
        assert_eq!(floor3(jarwin.rel_dist(s2, s1)), *d);
    }
}
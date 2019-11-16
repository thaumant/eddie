use super::{Jaro, MAX_CHARS};


fn pfloor(num: f64, presision: u32) -> f64 {
    let p = 10usize.pow(presision) as f64;
    (num * p).floor() / p
}


#[test]
fn jaro_equality() {
    let jaro = Jaro::new();
    let sample = [
        (0, 0, 1., ""),
        (1, 0, 1., "m"),
        (2, 0, 1., "ma"),
        (3, 0, 1., "mai"),
        (4, 0, 1., "mail"),
        (5, 0, 1., "mailb"),
        (6, 0, 1., "mailbo"),
        (7, 0, 1., "mailbox"),
    ];
    for (m, t, d, s) in sample.iter() {
        assert_eq!(jaro.dist(s, s), *d);
        assert_eq!(jaro.state.borrow().m, *m);
        assert_eq!(jaro.state.borrow().t, *t);
    }
}


#[test]
fn jaro_inequality() {
    let jaro = Jaro::new();
    let sample = [
        (0, 0, 0., "a",     "b"),
        (0, 0, 0., "aa",    "bb"),
        (0, 0, 0., "aaa",   "bbb"),
        (0, 0, 0., "aaaa",  "bbbb"),
        (0, 0, 0., "aaaaa", "bbbbb"),
    ];
    for (m, t, d, s1, s2) in sample.iter() {
        assert_eq!(jaro.dist(s1, s2), *d);
        assert_eq!(jaro.state.borrow().m, *m);
        assert_eq!(jaro.state.borrow().t, *t);
    }
}


#[test]
fn jaro_prefix() {
    let jaro = Jaro::new();
    let sample = [
        (6, 0, 0.952, "mailbox", "mailbo"),
        (5, 0, 0.904, "mailbox", "mailb"),
        (4, 0, 0.857, "mailbox", "mail"),
        (3, 0, 0.809, "mailbox", "mai"),
        (2, 0, 0.761, "mailbox", "ma"),
        (1, 0, 0.714, "mailbox", "m"),
    ];
    for (m, t, d, s1, s2) in sample.iter() {
        assert_eq!(pfloor(jaro.dist(s1, s2), 3), *d);
        assert_eq!(pfloor(jaro.dist(s2, s1), 3), *d);
        assert_eq!(jaro.state.borrow().m, *m);
        assert_eq!(jaro.state.borrow().t, *t);
    }
}


#[test]
fn jaro_postfix() {
    let jaro = Jaro::new();
    let sample = [
        (6, 0, 0.952, "mailbox", "ailbox"),
        (5, 0, 0.904, "mailbox", "ilbox"),
        (0, 0, 0.000, "mailbox", "lbox"),
        (0, 0, 0.000, "mailbox", "box"),
        (0, 0, 0.000, "mailbox", "ox"),
        (0, 0, 0.000, "mailbox", "x"),
    ];
    for (m, t, d, s1, s2) in sample.iter() {
        assert_eq!(pfloor(jaro.dist(s1, s2), 3), *d);
        assert_eq!(pfloor(jaro.dist(s2, s1), 3), *d);
        assert_eq!(jaro.state.borrow().m, *m);
        assert_eq!(jaro.state.borrow().t, *t);
    }
}


#[test]
fn jaro_match_distance() {
    let jaro = Jaro::new();
    let sample = [
        (0, 0, 0.000, "mailbox", "l......"),
        (1, 0, 0.428, "mailbox", ".l....."),
        (1, 0, 0.428, "mailbox", "..l...."),
        (1, 0, 0.428, "mailbox", "...l..."),
        (1, 0, 0.428, "mailbox", "....l.."),
        (1, 0, 0.428, "mailbox", ".....l."),
        (0, 0, 0.000, "mailbox", "......l"),
    ];
    for (m, t, d, s1, s2) in sample.iter() {
        assert_eq!(pfloor(jaro.dist(s1, s2), 3), *d);
        assert_eq!(pfloor(jaro.dist(s2, s1), 3), *d);
        assert_eq!(jaro.state.borrow().m, *m);
        assert_eq!(jaro.state.borrow().t, *t);
    }
}

#[test]
fn jaro_add_del_continuous() {
    let jaro = Jaro::new();
    let sample = [
        (7, 0, 0.958, "mailbox", ".mailbox"),
        (7, 0, 0.925, "mailbox", "..mailbox"),
        (7, 0, 0.900, "mailbox", "...mailbox"),
        (7, 0, 0.878, "mailbox", "....mailbox"),
        (7, 0, 0.861, "mailbox", ".....mailbox"),
        (0, 0, 0.000, "mailbox", "......mailbox"),

        (7, 0, 0.958, "mailbox", "mail.box"),
        (7, 0, 0.925, "mailbox", "mail..box"),
        (7, 0, 0.900, "mailbox", "mail...box"),
        (7, 0, 0.878, "mailbox", "mail....box"),
        (7, 0, 0.861, "mailbox", "mail.....box"),
        (4, 0, 0.626, "mailbox", "mail......box"),

        (7, 0, 0.958, "mailbox", "mailbox."),
        (7, 0, 0.925, "mailbox", "mailbox.."),
        (7, 0, 0.900, "mailbox", "mailbox..."),
        (7, 0, 0.878, "mailbox", "mailbox...."),
        (7, 0, 0.861, "mailbox", "mailbox....."),
        (7, 0, 0.846, "mailbox", "mailbox......"),
    ];
    for (m, t, d, s1, s2) in sample.iter() {
        assert_eq!(pfloor(jaro.dist(s1, s2), 3), *d);
        assert_eq!(pfloor(jaro.dist(s2, s1), 3), *d);
        assert_eq!(jaro.state.borrow().m, *m);
        assert_eq!(jaro.state.borrow().t, *t);
    }
}


#[test]
fn jaro_sub_continuous() {
    let jaro = Jaro::new();
    let sample = [
        (6, 0, 0.904, "mailbox", "mailbo."),
        (5, 0, 0.809, "mailbox", "mailb.."),
        (4, 0, 0.714, "mailbox", "mail..."),
        (3, 0, 0.619, "mailbox", "mai...."),
        (2, 0, 0.523, "mailbox", "ma....."),
        (1, 0, 0.428, "mailbox", "m......"),
        (1, 0, 0.428, "mailbox", "......x"),
        (2, 0, 0.523, "mailbox", ".....ox"),
        (3, 0, 0.619, "mailbox", "....box"),
        (4, 0, 0.714, "mailbox", "...lbox"),
        (5, 0, 0.809, "mailbox", "..ilbox"),
        (6, 0, 0.904, "mailbox", ".ailbox"),
    ];
    for (m, t, d, s1, s2) in sample.iter() {
        assert_eq!(pfloor(jaro.dist(s1, s2), 3), *d);
        assert_eq!(pfloor(jaro.dist(s2, s1), 3), *d);
        assert_eq!(jaro.state.borrow().m, *m);
        assert_eq!(jaro.state.borrow().t, *t);
    }
}


#[test]
fn jaro_add_del_intermittent() {
    let jaro = Jaro::new();
    let sample = [
        (7, 0, 0.958, "mailbox", "mailbox."),
        (7, 0, 0.925, "mailbox", "mailbo.x."),
        (7, 0, 0.900, "mailbox", "mailb.o.x."),
        (7, 0, 0.878, "mailbox", "mail.b.o.x."),
        (7, 0, 0.861, "mailbox", "mai.l.b.o.x."),
        (7, 0, 0.846, "mailbox", "ma.i.l.b.o.x."),
        (7, 0, 0.833, "mailbox", "m.a.i.l.b.o.x."),
        (6, 0, 0.752, "mailbox", ".m.a.i.l.b.o.x."),
        (6, 0, 0.761, "mailbox", ".m.a.i.l.b.o.x"),
        (5, 0, 0.699, "mailbox", ".m.a.i.l.b.ox"),
        (7, 0, 0.861, "mailbox", ".m.a.i.l.box"),
        (7, 0, 0.878, "mailbox", ".m.a.i.lbox"),
        (7, 0, 0.900, "mailbox", ".m.a.ilbox"),
        (7, 0, 0.925, "mailbox", ".m.ailbox"),
        (7, 0, 0.958, "mailbox", ".mailbox"),
    ];
    for (m, t, d, s1, s2) in sample.iter() {
        assert_eq!(pfloor(jaro.dist(s1, s2), 3), *d);
        assert_eq!(pfloor(jaro.dist(s2, s1), 3), *d);
        assert_eq!(jaro.state.borrow().m, *m);
        assert_eq!(jaro.state.borrow().t, *t);
    }
}


#[test]
fn jaro_sub_intermittent() {
    let jaro = Jaro::new();
    let sample = [
        (6, 0, 0.904, "mailbox", "mailbo."),
        (5, 0, 0.809, "mailbox", "mail.o."),
        (4, 0, 0.714, "mailbox", "ma.l.o."),
        (3, 0, 0.619, "mailbox", ".a.l.o."),
        (4, 0, 0.714, "mailbox", ".a.l.ox"),
        (5, 0, 0.809, "mailbox", ".a.lbox"),
        (6, 0, 0.904, "mailbox", ".ailbox"),
    ];
    for (m, t, d, s1, s2) in sample.iter() {
        assert_eq!(pfloor(jaro.dist(s1, s2), 3), *d);
        assert_eq!(pfloor(jaro.dist(s2, s1), 3), *d);
        assert_eq!(jaro.state.borrow().m, *m);
        assert_eq!(jaro.state.borrow().t, *t);
    }
}


#[test]
fn jaro_transpose() {
    let jaro = Jaro::new();
    let sample = [
        (7, 2, 0.952, "mailbox", "amilbox"),
        (7, 3, 0.928, "mailbox", "imalbox"),
        (7, 4, 0.904, "mailbox", "amlibox"),
        (7, 5, 0.880, "mailbox", "ambilox"),
        (7, 6, 0.857, "mailbox", "amliobx"),
        (7, 7, 0.833, "mailbox", "amlioxb"),

        (7, 2, 0.952, "mailbox", "mailbxo"),
        (7, 3, 0.928, "mailbox", "mailxbo"),
        (7, 4, 0.904, "mailbox", "maiblxo"),
        (7, 5, 0.880, "mailbox", "mabixlo"),
        (7, 6, 0.857, "mailbox", "miabxlo"),
        (7, 7, 0.833, "mailbox", "imabxlo"),
    ];
    for (m, t, d, s1, s2) in sample.iter() {
        assert_eq!(pfloor(jaro.dist(s1, s2), 3), *d);
        assert_eq!(pfloor(jaro.dist(s2, s1), 3), *d);
        assert_eq!(jaro.state.borrow().m, *m);
        assert_eq!(jaro.state.borrow().t, *t);
    }
}


#[test]
fn jaro_utf_multibyte() {
    let jaro = Jaro::new();
    let sample = [
        (4, 0, 0.933, "もしもし", "もしもしし"),
        (4, 0, 1.000, "もしもし", "もしもし"),
        (4, 2, 0.916, "もしもし", "もししも"),
        (3, 0, 0.833, "もしもし", "もしまし"),
        (3, 0, 0.916, "もしもし", "もしし"),
        (2, 0, 0.833, "もしもし", "もし"),
        (1, 0, 0.750, "もしもし", "し"),
        (0, 0, 0.000, "もしもし", ""),
    ];
    for (m, t, d, s1, s2) in sample.iter() {
        assert_eq!(pfloor(jaro.dist(s1, s2), 3), *d);
        assert_eq!(pfloor(jaro.dist(s2, s1), 3), *d);
        assert_eq!(jaro.state.borrow().m, *m);
        assert_eq!(jaro.state.borrow().t, *t);
    }
}


#[test]
fn jaro_mixed() {
    let jaro = Jaro::new();
    let sample = [
        (5, 3, 0.804, "captain",   "ptain"), // TODO should be t=0, d=0.904
        (4, 0, 0.822, "dwayne",    "duane"),
        (6, 2, 0.944, "martha",    "marhta"),
        (4, 0, 0.766, "dixon",     "dicksonx"),
        (8, 0, 0.896, "jellyfish", "smellyfish"),
    ];
    for (m, t, d, s1, s2) in sample.iter() {
        assert_eq!(pfloor(jaro.dist(s1, s2), 3), *d);
        assert_eq!(pfloor(jaro.dist(s2, s1), 3), *d);
        assert_eq!(jaro.state.borrow().m, *m);
        assert_eq!(jaro.state.borrow().t, *t);
    }
}


#[test]
fn jaro_max_chars() {
    let jaro = Jaro::new();

    for len in 1 .. MAX_CHARS + 1 {
        let s = &"a".repeat(len);
        assert_eq!(jaro.dist(s, s), 1.0);
        assert_eq!(jaro.dist(s, s), 1.0);
        assert_eq!(jaro.state.borrow().m, len);
        assert_eq!(jaro.state.borrow().t, 0);
    }

    for len in MAX_CHARS + 1 .. MAX_CHARS + 2 {
        let s = &"a".repeat(len);
        assert_eq!(jaro.dist(s, s), 1.0);
        assert_eq!(jaro.dist(s, s), 1.0);
        assert_eq!(jaro.state.borrow().m, MAX_CHARS);
        assert_eq!(jaro.state.borrow().t, 0);
    }
}

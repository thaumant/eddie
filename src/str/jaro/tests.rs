use super::{Jaro, DEFAULT_CAPACITY};


fn floor3(num: f64) -> f64 {
    let p = 10usize.pow(3) as f64;
    (num * p).floor() / p
}


#[test]
fn equality() {
    let jaro = Jaro::new();
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
    for (d, s) in sample.iter() {
        assert_eq!(jaro.similarity(s, s), *d);
    }
}


#[test]
fn inequality() {
    let jaro = Jaro::new();
    let sample = [
        (0., "a",     "b"),
        (0., "aa",    "bb"),
        (0., "aaa",   "bbb"),
        (0., "aaaa",  "bbbb"),
        (0., "aaaaa", "bbbbb"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(jaro.similarity(s1, s2), *d);
    }
}


#[test]
fn prefix() {
    let jaro = Jaro::new();
    let sample = [
        (0.952, "mailbox", "mailbo"),
        (0.904, "mailbox", "mailb"),
        (0.857, "mailbox", "mail"),
        (0.809, "mailbox", "mai"),
        (0.761, "mailbox", "ma"),
        (0.714, "mailbox", "m"),
        (0.000, "mailbox", ""),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(floor3(jaro.similarity(s1, s2)), *d);
        assert_eq!(floor3(jaro.similarity(s2, s1)), *d);
    }
}


#[test]
fn postfix() {
    let jaro = Jaro::new();
    let sample = [
        (0.952, "mailbox", "ailbox"),
        (0.904, "mailbox", "ilbox"),
        (0.000, "mailbox", "lbox"),
        (0.000, "mailbox", "box"),
        (0.000, "mailbox", "ox"),
        (0.000, "mailbox", "x"),
        (0.000, "mailbox", ""),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(floor3(jaro.similarity(s1, s2)), *d);
        assert_eq!(floor3(jaro.similarity(s2, s1)), *d);
    }
}


#[test]
fn match_distance() {
    let jaro = Jaro::new();
    let sample = [
        (0.000, "mailbox", "l......"),
        (0.428, "mailbox", ".l....."),
        (0.428, "mailbox", "..l...."),
        (0.428, "mailbox", "...l..."),
        (0.428, "mailbox", "....l.."),
        (0.428, "mailbox", ".....l."),
        (0.000, "mailbox", "......l"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(floor3(jaro.similarity(s1, s2)), *d);
        assert_eq!(floor3(jaro.similarity(s2, s1)), *d);
    }
}

#[test]
fn add_del_continuous() {
    let jaro = Jaro::new();
    let sample = [
        (0.958, "mailbox", ".mailbox"),
        (0.925, "mailbox", "..mailbox"),
        (0.900, "mailbox", "...mailbox"),
        (0.878, "mailbox", "....mailbox"),
        (0.861, "mailbox", ".....mailbox"),
        (0.000, "mailbox", "......mailbox"),

        (0.958, "mailbox", "mail.box"),
        (0.925, "mailbox", "mail..box"),
        (0.900, "mailbox", "mail...box"),
        (0.878, "mailbox", "mail....box"),
        (0.861, "mailbox", "mail.....box"),
        (0.626, "mailbox", "mail......box"),

        (0.958, "mailbox", "mailbox."),
        (0.925, "mailbox", "mailbox.."),
        (0.900, "mailbox", "mailbox..."),
        (0.878, "mailbox", "mailbox...."),
        (0.861, "mailbox", "mailbox....."),
        (0.846, "mailbox", "mailbox......"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(floor3(jaro.similarity(s1, s2)), *d);
        assert_eq!(floor3(jaro.similarity(s2, s1)), *d);
    }
}


#[test]
fn sub_continuous() {
    let jaro = Jaro::new();
    let sample = [
        (0.904, "mailbox", "mailbo."),
        (0.809, "mailbox", "mailb.."),
        (0.714, "mailbox", "mail..."),
        (0.619, "mailbox", "mai...."),
        (0.523, "mailbox", "ma....."),
        (0.428, "mailbox", "m......"),
        (0.428, "mailbox", "......x"),
        (0.523, "mailbox", ".....ox"),
        (0.619, "mailbox", "....box"),
        (0.714, "mailbox", "...lbox"),
        (0.809, "mailbox", "..ilbox"),
        (0.904, "mailbox", ".ailbox"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(floor3(jaro.similarity(s1, s2)), *d);
        assert_eq!(floor3(jaro.similarity(s2, s1)), *d);
    }
}


#[test]
fn add_del_intermittent() {
    let jaro = Jaro::new();
    let sample = [
        (0.958, "mailbox", "mailbox."),
        (0.925, "mailbox", "mailbo.x."),
        (0.900, "mailbox", "mailb.o.x."),
        (0.878, "mailbox", "mail.b.o.x."),
        (0.861, "mailbox", "mai.l.b.o.x."),
        (0.846, "mailbox", "ma.i.l.b.o.x."),
        (0.833, "mailbox", "m.a.i.l.b.o.x."),
        (0.752, "mailbox", ".m.a.i.l.b.o.x."),
        (0.761, "mailbox", ".m.a.i.l.b.o.x"),
        (0.699, "mailbox", ".m.a.i.l.b.ox"),
        (0.861, "mailbox", ".m.a.i.l.box"),
        (0.878, "mailbox", ".m.a.i.lbox"),
        (0.900, "mailbox", ".m.a.ilbox"),
        (0.925, "mailbox", ".m.ailbox"),
        (0.958, "mailbox", ".mailbox"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(floor3(jaro.similarity(s1, s2)), *d);
        assert_eq!(floor3(jaro.similarity(s2, s1)), *d);
    }
}


#[test]
fn sub_intermittent() {
    let jaro = Jaro::new();
    let sample = [
        (0.904, "mailbox", "mailbo."),
        (0.809, "mailbox", "mail.o."),
        (0.714, "mailbox", "ma.l.o."),
        (0.619, "mailbox", ".a.l.o."),
        (0.714, "mailbox", ".a.l.ox"),
        (0.809, "mailbox", ".a.lbox"),
        (0.904, "mailbox", ".ailbox"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(floor3(jaro.similarity(s1, s2)), *d);
        assert_eq!(floor3(jaro.similarity(s2, s1)), *d);
    }
}


#[test]
fn transpose() {
    let jaro = Jaro::new();
    let sample = [
        (0.952, "mailbox", "amilbox"),
        (0.928, "mailbox", "imalbox"),
        (0.904, "mailbox", "amlibox"),
        (0.880, "mailbox", "ambilox"),
        (0.857, "mailbox", "amliobx"),
        (0.833, "mailbox", "amlioxb"),

        (0.952, "mailbox", "mailbxo"),
        (0.928, "mailbox", "mailxbo"),
        (0.904, "mailbox", "maiblxo"),
        (0.880, "mailbox", "mabixlo"),
        (0.857, "mailbox", "miabxlo"),
        (0.833, "mailbox", "imabxlo"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(floor3(jaro.similarity(s1, s2)), *d);
        assert_eq!(floor3(jaro.similarity(s2, s1)), *d);
    }
}


#[test]
fn utf_multibyte() {
    let jaro = Jaro::new();
    let sample = [
        (0.933, "もしもし", "もしもしし"),
        (1.000, "もしもし", "もしもし"),
        (0.916, "もしもし", "もししも"),
        (0.833, "もしもし", "もしまし"),
        (0.916, "もしもし", "もしし"),
        (0.833, "もしもし", "もし"),
        (0.750, "もしもし", "し"),
        (0.000, "もしもし", ""),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(floor3(jaro.similarity(s1, s2)), *d);
        assert_eq!(floor3(jaro.similarity(s2, s1)), *d);
    }
}


#[test]
fn mixed() {
    let jaro = Jaro::new();
    let sample = [
        (0.000, "ca",        "abc"),
        (0.783, "a tc",      "a cat"),
        (0.790, "a cat",     "an abct"),
        (0.733, "crate",     "trace"),
        (0.804, "captain",   "ptain"),
        (0.822, "dwayne",    "duane"),
        (0.944, "martha",    "marhta"),
        (0.746, "kitten",    "sitting"),
        (0.849, "mailbox",   "alimbox"),
        (0.000, "mailbox",   "boxmail"),
        (0.766, "dixon",     "dicksonx"),
        (0.896, "jellyfish", "smellyfish"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(floor3(jaro.similarity(s1, s2)), *d);
        assert_eq!(floor3(jaro.similarity(s2, s1)), *d);
    }
}


#[test]
fn growth() {
    let jaro = Jaro::new();

    for len in 1 .. DEFAULT_CAPACITY * 2 {
        let s1 = &"a".repeat(len);
        let s2 = &"b".repeat(len);
        assert_eq!(jaro.similarity(s1, s1), 1.0);
        assert_eq!(jaro.similarity(s1, s2), 0.0);
    }
}


#[test]
fn rel_dist() {
    let jaro = Jaro::new();
    let sample = [
        (0.000, "",        ""),
        (1.000, "mailbox", ""),
        (0.142, "mailbox", "mail"),
        (0.095, "mailbox", "ilbox"),
        (0.571, "mailbox", "..l...."),
        (0.121, "mailbox", "....mailbox"),
        (0.285, "mailbox", "mail..."),
        (0.095, "mailbox", "amlibox"),
        (1.000, "mailbox", "boxmail"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(floor3(jaro.rel_dist(s1, s2)), *d);
        assert_eq!(floor3(jaro.rel_dist(s2, s1)), *d);
    }
}

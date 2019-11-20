use super::{DamerauLevenshtein, DEFAULT_CAPACITY};

#[test]
fn equality() {
    let damlev = DamerauLevenshtein::new();
    let sample = [
        "",
        "c",
        "ca",
        "cap",
        "capt",
        "capta",
        "captai",
        "captain",
    ];
    for s in sample.iter() {
        assert_eq!(damlev.dist(s, s), 0);
    }
}

#[test]
fn prefix() {
    let damlev = DamerauLevenshtein::new();
    let s1 = "captain";
    let sample = [
        (1, "captai"),
        (2, "capta"),
        (3, "capt"),
        (4, "cap"),
        (5, "ca"),
        (6, "c"),
        (7, ""),
    ];
    for (d, s2) in sample.iter() {
        assert_eq!(damlev.dist(s1, s2), *d);
        assert_eq!(damlev.dist(s2, s1), *d);
    }
}

#[test]
fn del_continuous() {
    let damlev = DamerauLevenshtein::new();
    let s2 = "captain";
    let sample = [
        (1, "_captain"),
        (2, "__captain"),
        (3, "___captain"),
        (4, "____captain"),

        (1, "cap_tain"),
        (2, "cap__tain"),
        (3, "cap___tain"),
        (4, "cap____tain"),

        (1, "captain_"),
        (2, "captain__"),
        (3, "captain___"),
        (4, "captain____"),
    ];
    for (d, s1) in sample.iter() {
        assert_eq!(damlev.dist(s1, s2), *d);
    }
}

#[test]
fn add_continuous() {
    let damlev = DamerauLevenshtein::new();
    let s1 = "captain";
    let sample = [
        (1, "_captain"),
        (2, "__captain"),
        (3, "___captain"),
        (4, "____captain"),

        (1, "cap_tain"),
        (2, "cap__tain"),
        (3, "cap___tain"),
        (4, "cap____tain"),

        (1, "captain_"),
        (2, "captain__"),
        (3, "captain___"),
        (4, "captain____"),
    ];
    for (d, s2) in sample.iter() {
        assert_eq!(damlev.dist(s1, s2), *d);
    }
}

#[test]
fn sub_continuous() {
    let damlev = DamerauLevenshtein::new();
    let s1 = "captain";
    let sample = [
        (1, "_aptain"),
        (2, "__ptain"),
        (3, "___tain"),
        (4, "____ain"),

        (1, "cap_ain"),
        (2, "cap__in"),
        (3, "ca___in"),
        (4, "ca____n"),

        (1, "captai_"),
        (2, "capta__"),
        (3, "capt___"),
        (4, "cap____"),
    ];
    for (d, s2) in sample.iter() {
        assert_eq!(damlev.dist(s1, s2), *d);
    }
}

#[test]
fn trans_continuous() {
    let damlev = DamerauLevenshtein::new();
    let s1 = "captain";
    let sample = [
        (1, "acptain"),
        (2, "actpain"),
        (3, "actpian"),

        (1, "captani"),
        (2, "capatni"),
        (3, "cpaatni"),
    ];
    for (d, s2) in sample.iter() {
        assert_eq!(damlev.dist(s1, s2), *d);
    }
}

#[test]
fn del_intermittent() {
    let damlev = DamerauLevenshtein::new();
    let s2 = "captain";
    let sample = [
        (1, "_captain"),
        (2, "_c_aptain"),
        (3, "_c_a_ptain"),
        (4, "_c_a_p_tain"),

        (1, "captain_"),
        (2, "captai_n_"),
        (3, "capta_i_n_"),
        (4, "capt_a_i_n_"),
    ];
    for (d, s1) in sample.iter() {
        assert_eq!(damlev.dist(s1, s2), *d);
    }
}

#[test]
fn add_intermittent() {
    let damlev = DamerauLevenshtein::new();
    let s1 = "captain";
    let sample = [
        (1, "_captain"),
        (2, "_c_aptain"),
        (3, "_c_a_ptain"),
        (4, "_c_a_p_tain"),

        (1, "captain_"),
        (2, "captai_n_"),
        (3, "capta_i_n_"),
        (4, "capt_a_i_n_"),
    ];
    for (d, s2) in sample.iter() {
        assert_eq!(damlev.dist(s1, s2), *d);
    }
}

#[test]
fn sub_intermittent() {
    let damlev = DamerauLevenshtein::new();
    let s1 = "captain";
    let sample = [
        (1, "_aptain"),
        (2, "_a_tain"),
        (3, "_a_t_in"),

        (1, "captai_"),
        (2, "capt_i_"),
        (3, "ca_t_i_"),
    ];
    for (d, s2) in sample.iter() {
        assert_eq!(damlev.dist(s1, s2), *d);
    }
}

#[test]
fn trans_intermittent() {
    let damlev = DamerauLevenshtein::new();
    let s1 = "captain";
    let sample = [
        (1, "acptain"),
        (2, "acpatin"),

        (1, "captani"),
        (2, "catpani"),
    ];
    for (d, s2) in sample.iter() {
        assert_eq!(damlev.dist(s1, s2), *d);
    }
}

#[test]
fn mixed() {
    let damlev = DamerauLevenshtein::new();
    let sample = [
        (2, "ca", "abc"),
        (2, "a tc", "a cat"),
        (3, "a cat", "an abct"),
        (3, "captain", "atpcain"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(damlev.dist(s1, s2), *d);
        assert_eq!(damlev.dist(s2, s1), *d);
    }
}

#[test]
fn growth() {
    let damlev = DamerauLevenshtein::new();

    for len in DEFAULT_CAPACITY + 1 .. DEFAULT_CAPACITY * 2 {
        let s1 = &"a".repeat(len);
        let s2 = &"b".repeat(len);
        assert_eq!(damlev.dist(s1, s1), 0);
        assert_eq!(damlev.dist(s1, ""), len);
        assert_eq!(damlev.dist(s1, s2), len);
    }
}

#[test]
fn utf_multibyte() {
    let damlev = DamerauLevenshtein::new();
    let s1 = "もしもし";
    let sample= [
        (1, "もしもしし"),
        (0, "もしもし"),
        (1, "もししも"),
        (1, "もしまし"),
        (1, "もしし"),
        (2, "もし"),
        (3, "し"),
        (4, ""),
    ];
    for (d, s2) in sample.iter() {
        assert_eq!(damlev.dist(s1, s2), *d);
    }
}
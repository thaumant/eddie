use super::{DamLev, MAX_CHARS};

#[test]
fn damlev_dist_equality() {
    let dl = DamLev::new();
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
        assert_eq!(dl.dist(s, s), 0);
    }
}

#[test]
fn damlev_dist_prefix_left() {
    let dl = DamLev::new();
    let s2 = "captain";
    let sample = [
        (0, "captain"),
        (1, "captai"),
        (2, "capta"),
        (3, "capt"),
        (4, "cap"),
        (5, "ca"),
        (6, "c"),
        (7, ""),
    ];
    for (d, s1) in sample.iter() {
        assert_eq!(dl.dist(s1, s2), *d);
    }
}

#[test]
fn damlev_dist_prefix_right() {
    let dl = DamLev::new();
    let s1 = "captain";
    let sample = [
        (0, "captain"),
        (1, "captai"),
        (2, "capta"),
        (3, "capt"),
        (4, "cap"),
        (5, "ca"),
        (6, "c"),
        (7, ""),
    ];
    for (d, s2) in sample.iter() {
        assert_eq!(dl.dist(s1, s2), *d);
    }
}

#[test]
fn damlev_dist_del_continuous() {
    let dl = DamLev::new();
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
        assert_eq!(dl.dist(s1, s2), *d);
    }
}

#[test]
fn damlev_dist_add_continuous() {
    let dl = DamLev::new();
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
        assert_eq!(dl.dist(s1, s2), *d);
    }
}

#[test]
fn damlev_dist_sub_continuous() {
    let dl = DamLev::new();
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
        assert_eq!(dl.dist(s1, s2), *d);
    }
}

#[test]
fn damlev_dist_swp_continuous() {
    let dl = DamLev::new();
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
        assert_eq!(dl.dist(s1, s2), *d);
    }
}

#[test]
fn damlev_dist_del_intermittent() {
    let dl = DamLev::new();
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
        assert_eq!(dl.dist(s1, s2), *d);
    }
}

#[test]
fn damlev_dist_add_intermittent() {
    let dl = DamLev::new();
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
        assert_eq!(dl.dist(s1, s2), *d);
    }
}

#[test]
fn damlev_dist_sub_intermittent() {
    let dl = DamLev::new();
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
        assert_eq!(dl.dist(s1, s2), *d);
    }
}

#[test]
fn damlev_dist_swp_intermittent() {
    let dl = DamLev::new();
    let s1 = "captain";
    let sample = [
        (1, "acptain"),
        (2, "acpatin"),

        (1, "captani"),
        (2, "catpani"),
    ];
    for (d, s2) in sample.iter() {
        assert_eq!(dl.dist(s1, s2), *d);
    }
}

#[test]
fn damlev_dist_mixed() {
    let dl = DamLev::new();
    let sample = [
        (2, "ca", "abc"),
        (2, "a tc", "a cat"),
        (3, "a cat", "an abct"),
        (3, "captain", "atpcain"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(dl.dist(s1, s2), *d);
        assert_eq!(dl.dist(s2, s1), *d);
    }
}

#[test]
fn damlev_dist_limit() {
    let dl = DamLev::new();

    for len in 0 .. MAX_CHARS + 1 {
        let s1 = &"a".repeat(len);
        let s2 = &"b".repeat(len);
        assert_eq!(dl.dist(s1, s2), len);
        assert_eq!(dl.dist(s2, s1), len);
    }

    for len in MAX_CHARS + 1 .. MAX_CHARS + 2 {
        let s1 = &"a".repeat(len);
        let s2 = &"b".repeat(len);
        assert_eq!(dl.dist(s1, s2), MAX_CHARS);
        assert_eq!(dl.dist(s2, s1), MAX_CHARS);
    }
}
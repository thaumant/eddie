use super::Hamming;

fn floor3(num: f64) -> f64 {
    let p = 10usize.pow(3) as f64;
    (num * p).floor() / p
}

#[test]
fn equality() {
    let hamming = Hamming::new();
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
        assert_eq!(hamming.distance(s, s), Some(0));
    }
}

#[test]
fn inequality() {
  let hamming = Hamming::new();
  for i in 1..10 {
    let s1 = "a".repeat(i);
    let s2 = "b".repeat(i);
    assert_eq!(hamming.distance(&s1, &s2), Some(i));
  }
}

#[test]
fn length_difference() {
  let hamming = Hamming::new();
  for len1 in 1..10 {
      for len2 in 0 .. len1 - 1 {
          let a1 = "a".repeat(len1);
          let a2 = "a".repeat(len2);
          let b2 = "b".repeat(len2);
          assert_eq!(hamming.distance(&a1, &a2), None);
          assert_eq!(hamming.distance(&a1, &b2), None);
          assert_eq!(hamming.distance(&a2, &a1), None);
          assert_eq!(hamming.distance(&b2, &a1), None);
      }
  }
}

#[test]
fn sub_continuous() {
    let hamming = Hamming::new();
    let sample = [
        (Some(1), "mailbox", "_ailbox"),
        (Some(2), "mailbox", "__ilbox"),
        (Some(3), "mailbox", "___lbox"),
        (Some(4), "mailbox", "____box"),

        (Some(1), "mailbox", "mai_box"),
        (Some(2), "mailbox", "mai__ox"),
        (Some(3), "mailbox", "ma___ox"),
        (Some(4), "mailbox", "ma____x"),

        (Some(1), "mailbox", "mailbo_"),
        (Some(2), "mailbox", "mailb__"),
        (Some(3), "mailbox", "mail___"),
        (Some(4), "mailbox", "mai____"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(hamming.distance(s1, s2), *d);
    }
}

#[test]
fn sub_intermittent() {
    let hamming = Hamming::new();
    let sample = [
        (Some(1), "mailbox", "_ailbox"),
        (Some(2), "mailbox", "_a_lbox"),
        (Some(3), "mailbox", "_a_l_ox"),

        (Some(1), "mailbox", "mailbo_"),
        (Some(2), "mailbox", "mail_o_"),
        (Some(3), "mailbox", "ma_l_o_"),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(hamming.distance(s1, s2), *d);
    }
}

#[test]
fn utf_multibyte() {
    let hamming = Hamming::new();
    let s1 = "もしもし";
    let sample= [
        (Some(0), "もしもし"),
        (Some(1), "もしまし"),
        (Some(1), "もし_し"),
        (None, "もしも"),
    ];
    for (d, s2) in sample.iter() {
        assert_eq!(hamming.distance(s1, s2), *d);
        assert_eq!(hamming.distance(s2, s1), *d);
    }
}

#[test]
fn rel_dist() {
    let hamming = Hamming::new();
    let sample = [
        (Some(0.000), "",        ""),
        (Some(0.000), "mailbox", "mailbox"),
        (Some(1.000), "mailbox", "boxmail"),
        (Some(0.285), "mailbox", "mai__ox"),
        (Some(0.571), "mailbox", "amilobx"),
        (None,        "mailbox", "mail"),
        (None,        "mailbox", ""),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(hamming.rel_dist(s1, s2).map(floor3), *d);
        assert_eq!(hamming.rel_dist(s2, s1).map(floor3), *d);
    }
}

#[test]
fn similarity() {
    let hamming = Hamming::new();
    let sample = [
        (Some(1.000), "",        ""),
        (Some(1.000), "mailbox", "mailbox"),
        (Some(0.000), "mailbox", "boxmail"),
        (Some(0.428), "mailbox", "amilobx"),
        (Some(0.714), "mailbox", "mai__ox"),
        (None,        "mailbox", "mail"),
        (None,        "mailbox", ""),
    ];
    for (d, s1, s2) in sample.iter() {
        assert_eq!(hamming.similarity(s1, s2).map(floor3), *d);
        assert_eq!(hamming.similarity(s2, s1).map(floor3), *d);
    }
}

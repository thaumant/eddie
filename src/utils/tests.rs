use super::write_str;

#[test]
fn write_str_empty() {
  let mut sample = [
    ("", vec!['0'; 0], vec![]),
    ("", vec!['0'; 1], vec![]),
    ("", vec!['0'; 2], vec![]),
  ];
  for (input, store, expected) in &mut sample {
    write_str(input, store);
    assert_eq!(store, expected);
  }
}

#[test]
fn write_str_nonempty() {
  let mut sample = [
    ("foo", vec!['0'; 0], vec!['f', 'o', 'o']),
    ("foo", vec!['0'; 1], vec!['f', 'o', 'o']),
    ("foo", vec!['0'; 2], vec!['f', 'o', 'o']),
    ("foo", vec!['0'; 3], vec!['f', 'o', 'o']),
    ("foo", vec!['0'; 4], vec!['f', 'o', 'o']),
  ];
  for (input, store, expected) in &mut sample {
    write_str(input, store);
    assert_eq!(store, expected);
  }
}

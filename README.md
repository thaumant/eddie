# Eddie

Fast and well-tested implementations of common edit distance/string similarity metrics:
- Levenshtein,
- Damerau-Levenshtein,
- Jaro,
- Jaro-Winkler.


## Usage

Cargo.toml:
```toml
[dependencies]
eddie = "0.1"
 ```

Levenshtein:
```rust
use eddie::Levenshtein;
let lev = Levenshtein::new();
let dist = lev.distance("martha", "marhta");
assert_eq!(dist, 2);
```

Damerau-Levenshtein:
```rust
use eddie::DamerauLevenshtein;
let damlev = DamerauLevenshtein::new();
let dist = damlev.distance("martha", "marhta");
assert_eq!(dist, 1);
```

Jaro:
```rust
use eddie::Jaro;
let jaro = Jaro::new();
let sim = jaro.similarity("martha", "marhta");
assert!((sim - 0.94).abs() < 0.01);
```

Jaro-Winkler:
```rust
use eddie::JaroWinkler;
let jarwin = JaroWinkler::new();
let sim = jarwin.similarity("martha", "marhta");
assert!((sim - 0.96).abs() < 0.01);
```

## Complementary metrics

The main metric methods are complemented with inverted and/or relative versions.
The naming convention across the crate is following:
- `distance` — a number of edits required to transform one string to the other;
- `rel_dist` — a distance between two strings, relative to string length (inversion of similarity);
- `similarity` — similarity between two strings (inversion of relative distance).


## Performance

Below are the tables with performance measurements for typical word lengths, including alternative implementations from crates.io that have Unicode support. Produced on Intel Core i5-4278U 2,6 GHz. All measurements are in microseconds.


### Jaro

|            | size=3 | size=6 | size=9 | size=12 | size=15 |
| :--------- | -----: | -----: | -----: | ------: | ------: |
| **eddie**  |  0.05  |  0.11  |  0.14  |  0.15   |  0.18   |
| **strsim** |  0.11  |  0.19  |  0.24  |  0.31   |  0.41   |


### Jaro-Winkler

|             | size=3 | size=6 | size=9 | size=12 | size=15 |
| :---------- | -----: | -----: | -----: | ------: | ------: |
| **eddie**   |  0.06  |  0.10  |  0.14  |  0.16   |  0.19   |
| **strsim**  |  0.12  |  0.18  |  0.22  |  0.30   |  0.39   |
| **natural** |  0.32  |  0.87  |  0.99  |  1.56   |  1.64   |


### Levenshtein

|                   | size=3 | size=6 | size=9 | size=12 | size=15 |
| :---------------- | -----: | -----: | -----: | ------: | ------: |
| **eddie**         |  0.05  |  0.12  |  0.18  |  0.35   |  0.57   |
| **edit_distance** |  0.15  |  0.19  |  0.26  |  0.49   |  0.60   |
| **strsim**        |  0.13  |  0.23  |  0.27  |  0.43   |  0.64   |
| **distance**      |  0.89  |  1.45  |  2.12  |  2.96   |  4.10   |


### Damerau-Levenshtein

|              | size=3 | size=6 | size=9 | size=12 | size=15 |
| :----------- | -----: | -----: | -----: | ------: | ------: |
| **eddie**    |  0.39  |  0.95  |  1.69  |  2.50   |  4.07   |
| **strsim**   |  1.03  |  2.07  |  3.73  |  5.16   |  7.05   |
| **distance** |  1.74  |  3.40  |  5.17  |  7.16   | 10.66   |

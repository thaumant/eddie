# Eddie

Fast and well-tested implementations of edit distance/string similarity metrics:
- Levenshtein,
- Damerau-Levenshtein,
- Hamming,
- Jaro,
- Jaro-Winkler.


## Usage

Cargo.toml:
```toml
[dependencies]
eddie = "0.2"
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

Hamming:
```rust
use eddie::Hamming;
let hamming = Hamming::new();
let dist = hamming.distance("martha", "marhta");
assert_eq!(dist, Some(2));
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


### Levenshtein

|                       |  len=3 |  len=6 |  len=9 |  len=12 |  len=15 |
| :-------------------- | -----: | -----: | -----: | ------: | ------: |
| **eddie 0.2**         |   0.04 |   0.10 |   0.23 |    0.44 |    0.60 |
| **edit_distance 2.1** |   0.13 |   0.20 |   0.31 |    0.56 |    0.65 |
| **strsim 0.9**        |   0.13 |   0.19 |   0.32 |    0.60 |    0.68 |
| **distance 0.4**      |   0.83 |   1.41 |   2.13 |    3.05 |    3.72 |


### Damerau-Levenshtein

|                  |  len=3 |  len=6 |  len=9 |  len=12 |  len=15 |
| :--------------- | -----: | -----: | -----: | ------: | ------: |
| **eddie 0.2**    |   0.39 |   0.95 |   1.69 |    2.50 |    4.07 |
| **strsim 0.9**   |   1.03 |   2.07 |   3.73 |    5.16 |    7.05 |
| **distance 0.4** |   1.74 |   3.40 |   5.17 |    7.16 |   10.66 |


### Hamming

|                  |  len=3 |  len=6 |  len=9 |  len=12 |  len=15 |
| :--------------- | -----: | -----: | -----: | ------: | ------: |
| **eddie 0.2**    |  0.011 |  0.015 |  0.022 |   0.026 |   0.031 |
| **strsim 0.9**   |  0.012 |  0.020 |  0.031 |   0.041 |   0.048 |
| **distance 0.4** |  0.016 |  0.031 |  0.035 |   0.045 |   0.062 |


### Jaro

|                |  len=3 |  len=6 |  len=9 |  len=12 |  len=15 |
| :------------- | -----: | -----: | -----: | ------: | ------: |
| **eddie 0.2**  |   0.05 |   0.11 |   0.14 |    0.15 |    0.18 |
| **strsim 0.9** |   0.11 |   0.19 |   0.24 |    0.31 |    0.41 |


### Jaro-Winkler

|                 |  len=3 |  len=6 |  len=9 |  len=12 |  len=15 |
| :-------------- | -----: | -----: | -----: | ------: | ------: |
| **eddie 0.2**   |   0.06 |   0.10 |   0.14 |    0.16 |    0.19 |
| **strsim 0.9**  |   0.12 |   0.18 |   0.22 |    0.30 |    0.39 |
| **natural 0.3** |   0.32 |   0.87 |   0.99 |    1.56 |    1.64 |

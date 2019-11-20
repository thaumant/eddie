# Eddie

Fast and well-tested implementations of common edit distance/word similarity metrics: Jaro, Jaro-Winkler, Levenshtein, and Damerau-Levenshtein.


## Metric descriptions and usage

### Levenshtein

See [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance) for the detailed description.

Metrics/methods:
- `distance` — a number of edits (character additions, deletions and substitutions) required to transform one string into the other;
- `rel_dist` — relative distance, a number of edits relative to the length of the longest string, ranging from 1.0 (equality) to 0.0 (nothing in common);
- `similarity` — inversion of relative distance (0.0 for equality).

```rust
let lev = eddie::Levenshtein::new();

println!("{}", lev.distance("martha", "marhta"));   // 2
println!("{}", lev.rel_dist("martha", "marhta"));   // 0.333
println!("{}", lev.similarity("martha", "marhta")); // 0.667
```


### Damerau-Levenshtein

See [Damerau-Levenshtein distance](https://en.wikipedia.org/wiki/Damerau–Levenshtein_distance) for the detailed description.

Metrics/methods:
- `distance` — a number of edits (character additions, deletions, substitutions and transpositions) required to transform one string into the other;
- `rel_dist` — relative distance, a number of edits relative to the length of the longest string, ranging from 0.0 (equality) to 1.0 (nothing in common);
- `similarity` — inversion of relative distance (1.0 for equality).

```rust
let damlev = eddie::DamerauLevenshtein::new();

println!("{}", damlev.distance("martha", "marhta"));   // 1
println!("{}", damlev.rel_dist("martha", "marhta"));   // 0.167
println!("{}", damlev.similarity("martha", "marhta")); // 0.833
```


### Jaro

See [Jaro similarity](https://en.wikipedia.org/wiki/Jaro–Winkler_distance#Jaro_Similarity) for the detailed description.

Metrics/methods:
- `similarity` — reflects how close two strings are, ranging from 1.0 (equality) to 0.0 (nothing in common);
- `rel_dist` — inversion of similarity (0.0 for equality).

```rust
let jaro = eddie::Jaro::new();

println!("{}", jaro.similarity("martha", "marhta")); // 0.944
println!("{}", jaro.rel_dist("martha", "marhta"));   // 0.056
```


### Jaro-Winkler

See [Jaro-Winkler similarity](https://en.wikipedia.org/wiki/Jaro–Winkler_distance#Jaro–Winkler_Similarity) for the detailed description.

Metrics/methods:
- `similarity` — like Jaro similarity, but gives a higher score to the strings that start with the same sequence of characters (1.0 for equality);
- `rel_dist` — inversion of similarity (0.0 for equality).

```rust
let jarwin = eddie::JaroWinkler::new();

println!("{}", jarwin.similarity("martha", "marhta")); // 0.927
println!("{}", jaro.rel_dist("martha", "marhta"));     // 0.073
```


## Implementation details

All algorithms are implemented using structs rather than functions. They initialize and reuse internal mutable state to avoid unnecessary allocations and computations. It has a reasonable default capacity and grows on demand.

Unsafe code (mainly unchecked indexing) is used where it can provide significant performance improvements.


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

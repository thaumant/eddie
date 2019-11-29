# Benchmarks

Below are the detailed tables with performance measurements for typical word lengths, including alternative implementations from crates.io that have Unicode support.

All implementations were measured under two scenarios:
1. Comparing a word, modified by two random typos, with the original.
2. Comparing two unrelated words of the same length ("worst case").

For each implementation two sets of results are provided if performance changes significantly under different scenarios, otherwise just one.

Produced on Intel Core i5-4278U 2,6 GHz. All measurements are in microseconds.


### Levenshtein

|                            |  len=3 |  len=6 |  len=9 |  len=12 |  len=15 |
| :------------------------- | -----: | -----: | -----: | ------: | ------: |
| **eddie 0.2**              |   0.03 |   0.06 |   0.09 |    0.12 |    0.18 |
| **eddie 0.2** (worst case) |   0.05 |   0.13 |   0.26 |    0.41 |    0.61 |
| **strsim 0.9**             |   0.13 |   0.21 |   0.35 |    0.55 |    0.76 |
| **edit_distance 2.1**      |   0.16 |   0.22 |   0.34 |    0.49 |    0.80 |
| **distance 0.4**           |   0.92 |   1.56 |   2.48 |    2.95 |    3.92 |


### Damerau-Levenshtein

|                            |  len=3 |  len=6 |  len=9 |  len=12 |  len=15 |
| :------------------------- | -----: | -----: | -----: | ------: | ------: |
| **eddie 0.2**              |   0.22 |   0.31 |   0.37 |     0.5 |    0.62 |
| **eddie 0.2** (worst case) |   0.32 |   0.74 |   1.41 |    2.44 |    3.94 |
| **strsim 0.9**             |   0.94 |   2.03 |   3.69 |    5.14 |    7.33 |
| **distance 0.4**           |   1.79 |   3.18 |   5.45 |    7.73 |   10.92 |


### Hamming

|                  |  len=3 |  len=6 |  len=9 |  len=12 |  len=15 |
| :--------------- | -----: | -----: | -----: | ------: | ------: |
| **eddie 0.2**    |  0.012 |  0.019 |  0.026 |   0.036 |   0.045 |
| **strsim 0.9*    |  0.012 |  0.019 |  0.026 |   0.036 |   0.045 |
| **distance 0.4** |  0.017 |  0.027 |  0.036 |   0.051 |   0.061 |


### Jaro

|                            |  len=3 |  len=6 |  len=9 |  len=12 |  len=15 |
| :------------------------- | -----: | -----: | -----: | ------: | ------: |
| **eddie 0.2**              |   0.06 |   0.09 |   0.11 |    0.14 |    0.16 |
| **eddie 0.2** (worst case) |   0.06 |   0.09 |   0.13 |    0.18 |    0.22 |
| **strsim 0.9**             |   0.13 |   0.17 |   0.23 |    0.33 |    0.41 |


### Jaro-Winkler

|                            |  len=3 |  len=6 |  len=9 |  len=12 |  len=15 |
| :------------------------- | -----: | -----: | -----: | ------: | ------: |
| **eddie 0.2**              |   0.07 |   0.10 |   0.11 |    0.14 |    0.16 |
| **eddie 0.2** (worst case) |   0.07 |   0.10 |   0.13 |    0.18 |    0.22 |
| **strsim 0.9**             |   0.15 |   0.22 |   0.24 |    0.34 |    0.36 |
| **natural 0.3**            |  panic |   0.86 |   1.33 |    1.50 |    1.69 |

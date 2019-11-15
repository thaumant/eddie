### Levenshtein

|          | 3 chars | 6 chars | 9 chars | 12 chars |
| :------- | ------: | ------: | ------: | -------: |
| eddie    | 0.04 μs | 0.11 μs | 0.36 μs |  0.60 μs |
| strsim   | 0.12 μs | 0.20 μs | 0.45 μs |  0.72 μs |
| distance | 0.77 μs | 1.37 μs | 2.80 μs |  3.82 μs |


### Damerau-Levenshtein

|          | 3 chars | 6 chars | 9 chars | 12 chars |
| :------- | ------: | ------: | ------: | -------: |
| eddie    |  0.3 μs |  0.9 μs |  4.0 μs |   5.9 μs |
| strsim   |  0.9 μs |  1.7 μs |  5.1 μs |   6.9 μs |
| distance |  1.6 μs |  2.8 μs |  7.2 μs |   9.6 μs |

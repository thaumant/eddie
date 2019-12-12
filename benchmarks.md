# Benchmarks

Below are the detailed tables with performance measurements for typical word lengths, including alternative implementations from crates.io that have Unicode support.

Due to optimizations, performance of Eddie implementations (except Hamming) changes significantly depending on how different compared strings are, besides their lengths, which is not the case for all the other measured libraries. For this reason Eddie implementations have a pair of measurements, produced under the following scenarios:
1. Comparing the original world to it's version, modified by two random typos.
2. Comparing two unrelated words of the same length (worst case).

Produced on Intel Core i5-4278U 2,6 GHz. All measurements are in nanoseconds and rounded to 2 significant digits.


### Levenshtein

|                       |   len=3 |    len=6 |    len=9 |    len=12 |    len=15 |
| :-------------------- | ------: | -------: | -------: | --------: | --------: |
| **eddie 0.3**         | 30 - 50 | 60 - 130 | 90 - 260 | 120 - 410 | 180 - 610 |
| **strsim 0.9**        |     130 |      210 |      350 |       550 |       760 |
| **edit_distance 2.1** |     160 |      220 |      340 |       490 |       800 |
| **distance 0.4**      |     920 |     1600 |     2500 |      3000 |      3900 |


### Damerau-Levenshtein

|                   |     len=3 |       len=6 |      len=9 |     len=12 |     len=15 |
| :---------------- | --------: | ----------: | ---------: | ---------: | ---------: |
| **eddie 0.3**     | 220 - 320 |   310 - 740 | 370 - 1400 | 500 - 2400 | 620 - 3900 |
| **strsim 0.9**    |       940 |        2000 |       3700 |       5100 |       7300 |
| **distance 0.4**  |      1800 |        3200 |       5400 |       7700 |      10900 |


### Hamming

|                  | len=3 | len=6 | len=9 | len=12 | len=15 |
| :--------------- | ----: | ----: | ----: | -----: | -----: |
| **eddie 0.3**    |    12 |    19 |    26 |     36 |     45 |
| **strsim 0.9**   |    12 |    19 |    26 |     36 |     45 |
| **distance 0.4** |    17 |    27 |    36 |     51 |     61 |


### Jaro

|                | len=3 | len=6 |     len=9 |    len=12 |    len=15 |
| :------------- | ----: | ----: | --------: | --------: | --------: |
| **eddie 0.3**  |    60 |    90 | 110 - 130 | 140 - 190 | 170 - 230 |
| **strsim 0.9** |   130 |   170 |       230 |       330 |       410 |


### Jaro-Winkler

|                 | len=3 | len=6 |     len=9 |    len=12 |    len=15 |
| :-------------- | ----: | ----: | --------: | --------: | --------: |
| **eddie 0.3**   |    70 |   100 | 110 - 130 | 140 - 190 | 170 - 230 |
| **strsim 0.9**  |   150 |   220 |       240 |       340 |       360 |
| **natural 0.3** | panic |   860 |      1300 |      1500 |      1700 |

# playground

A place to do experiments with Rust.

## suffix_matching

Different implementations for matching string suffixes, e.g., file extensions.

```
cargo bench suffix_matching
```

### Results

Running on a MacBook Pro (M4 Pro, 2024).

```
test suffix_matching::tests::create_few_chumsky       ... bench:         158.70 ns/iter (+/- 13.98)
test suffix_matching::tests::create_few_glob          ... bench:         138.58 ns/iter (+/- 12.83)
test suffix_matching::tests::create_few_ptrie         ... bench:         717.54 ns/iter (+/- 22.63)
test suffix_matching::tests::create_few_simple        ... bench:         115.71 ns/iter (+/- 5.05)
test suffix_matching::tests::create_few_simple2       ... bench:          53.56 ns/iter (+/- 4.17)
test suffix_matching::tests::create_few_triers        ... bench:      20,430.65 ns/iter (+/- 2,375.78)
test suffix_matching::tests::create_few_yada          ... bench:       2,243.87 ns/iter (+/- 81.92)
test suffix_matching::tests::create_many_chumsky      ... bench:       2,779.28 ns/iter (+/- 69.42)
test suffix_matching::tests::create_many_glob         ... bench:         355.09 ns/iter (+/- 16.59)
test suffix_matching::tests::create_many_ptrie        ... bench:       9,539.68 ns/iter (+/- 498.15)
test suffix_matching::tests::create_many_simple       ... bench:       2,455.65 ns/iter (+/- 62.39)
test suffix_matching::tests::create_many_simple2      ... bench:         264.60 ns/iter (+/- 10.08)
test suffix_matching::tests::create_many_triers       ... bench:      53,821.24 ns/iter (+/- 5,347.08)
test suffix_matching::tests::create_many_yada         ... bench:      53,529.99 ns/iter (+/- 2,122.96)
test suffix_matching::tests::match_best_few_chumsky   ... bench:          23.49 ns/iter (+/- 0.37)
test suffix_matching::tests::match_best_few_glob      ... bench:         225.03 ns/iter (+/- 5.00)
test suffix_matching::tests::match_best_few_ptrie     ... bench:          22.17 ns/iter (+/- 1.09)
test suffix_matching::tests::match_best_few_simple    ... bench:           6.53 ns/iter (+/- 0.59)
test suffix_matching::tests::match_best_few_simple2   ... bench:           6.48 ns/iter (+/- 0.67)
test suffix_matching::tests::match_best_few_triers    ... bench:       1,260.85 ns/iter (+/- 35.19)
test suffix_matching::tests::match_best_few_yada      ... bench:          21.58 ns/iter (+/- 1.06)
test suffix_matching::tests::match_best_many_chumsky  ... bench:         128.20 ns/iter (+/- 2.12)
test suffix_matching::tests::match_best_many_glob     ... bench:       3,156.57 ns/iter (+/- 107.21)
test suffix_matching::tests::match_best_many_ptrie    ... bench:          27.61 ns/iter (+/- 1.75)
test suffix_matching::tests::match_best_many_simple   ... bench:         121.00 ns/iter (+/- 3.52)
test suffix_matching::tests::match_best_many_simple2  ... bench:         121.13 ns/iter (+/- 3.68)
test suffix_matching::tests::match_best_many_triers   ... bench:       2,270.74 ns/iter (+/- 74.90)
test suffix_matching::tests::match_best_many_yada     ... bench:          21.77 ns/iter (+/- 1.72)
test suffix_matching::tests::match_worst_few_chumsky  ... bench:          16.93 ns/iter (+/- 0.28)
test suffix_matching::tests::match_worst_few_glob     ... bench:         387.39 ns/iter (+/- 8.00)
test suffix_matching::tests::match_worst_few_ptrie    ... bench:          21.69 ns/iter (+/- 1.43)
test suffix_matching::tests::match_worst_few_simple   ... bench:           4.39 ns/iter (+/- 0.52)
test suffix_matching::tests::match_worst_few_simple2  ... bench:           4.40 ns/iter (+/- 0.70)
test suffix_matching::tests::match_worst_few_triers   ... bench:       1,222.80 ns/iter (+/- 31.33)
test suffix_matching::tests::match_worst_few_yada     ... bench:          22.15 ns/iter (+/- 1.11)
test suffix_matching::tests::match_worst_many_chumsky ... bench:         148.43 ns/iter (+/- 2.66)
test suffix_matching::tests::match_worst_many_glob    ... bench:       5,330.94 ns/iter (+/- 188.98)
test suffix_matching::tests::match_worst_many_ptrie   ... bench:          21.46 ns/iter (+/- 1.15)
test suffix_matching::tests::match_worst_many_simple  ... bench:         132.43 ns/iter (+/- 20.62)
test suffix_matching::tests::match_worst_many_simple2 ... bench:         135.54 ns/iter (+/- 22.00)
test suffix_matching::tests::match_worst_many_triers  ... bench:       2,307.23 ns/iter (+/- 114.60)
test suffix_matching::tests::match_worst_many_yada    ... bench:          22.13 ns/iter (+/- 1.04)
```

### Conclusion

`yada` and `ptrie` are the fastest. `trie-rs` is unexpectedly slow. `glob-match` is the slowest. All tries are slower than a simple loop check when there are few suffixes to match.

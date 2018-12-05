# Advent of Code 2018

## Current benchmarks as [this commit](ff06bfb8d7aaca6f367f55273d748e85e79f7baa)
```
test day1::tests::bench_ex1       ... bench:      27,038 ns/iter (+/- 503)
test day1::tests::bench_ex2       ... bench:  12,568,090 ns/iter (+/- 2,019,486)
test day1::tests::bench_reading   ... bench:      27,121 ns/iter (+/- 123)
test day2::tests::bench_p1        ... bench:     111,647 ns/iter (+/- 3,463)
test day2::tests::bench_p2        ... bench:     847,790 ns/iter (+/- 800,473)
test day2::tests::bench_read      ... bench:      27,828 ns/iter (+/- 473)
test day3::tests::d3_ex1_bench_sl ... bench:   2,458,800 ns/iter (+/- 24,959)
test day3::tests::d3_ex2_bench    ... bench:   2,121,020 ns/iter (+/- 15,273)
test day3::tests::d3_read         ... bench:   1,097,670 ns/iter (+/- 19,793)
test day4::tests::d4_ex1_bench    ... bench:     115,265 ns/iter (+/- 1,663)
test day4::tests::d4_ex2_bench    ... bench:     115,410 ns/iter (+/- 3,003)
test day4::tests::d4_preprocess   ... bench:      85,947 ns/iter (+/- 2,271)
test day4::tests::d4_read         ... bench:      98,925 ns/iter (+/- 1,124)
// Without improvement
test day5::tests::d5_bench_ex1 ... bench:  19,436,470 ns/iter (+/- 1,987,872)
test day5::tests::d5_bench_ex2 ... bench:  76,633,060 ns/iter (+/- 3,325,339)
// With small improvement
test day5::tests::d5_bench_ex1 ... bench:     224,975 ns/iter (+/- 57,992)
test day5::tests::d5_bench_ex2 ... bench:   3,225,215 ns/iter (+/- 773,984)
// With the lookback version of my algorithm that is way better readable.
test day5::tests::d5_bench_ex1  ... bench:     179,878 ns/iter (+/- 10,498)
test day5::tests::d5_bench_ex2  ... bench:   1,882,250 ns/iter (+/- 167,912)
test day5::tests::d5_bench_read ... bench:       1,316 ns/iter (+/- 88)
```

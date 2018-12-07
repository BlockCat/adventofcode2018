# Advent of Code 2018

## Current benchmarks as [this commit](ff06bfb8d7aaca6f367f55273d748e85e79f7baa)
```
test day1::tests::bench_ex1       ... bench:      27,038 ns/iter (+/- 503)
test day1::tests::bench_ex2       ... bench:  12,568,090 ns/iter (+/- 2,019,486)

test day2::tests::bench_p1        ... bench:     111,647 ns/iter (+/- 3,463)
test day2::tests::bench_p2        ... bench:     847,790 ns/iter (+/- 800,473)

test day3::tests::d3_ex1_bench_sl ... bench:   1,463,350 ns/iter (+/- 244,625)
test day3::tests::d3_ex2_bench    ... bench:   1,084,580 ns/iter (+/- 52,693)

test day4::tests::d4_ex1_bench    ... bench:     115,265 ns/iter (+/- 1,663)
test day4::tests::d4_ex2_bench    ... bench:     115,410 ns/iter (+/- 3,003)

// Without improvement
test day5::tests::d5_bench_ex1 ... bench:  19,436,470 ns/iter (+/- 1,987,872)
test day5::tests::d5_bench_ex2 ... bench:  76,633,060 ns/iter (+/- 3,325,339)

// With small improvement
test day5::tests::d5_bench_ex1 ... bench:     224,975 ns/iter (+/- 57,992)
test day5::tests::d5_bench_ex2 ... bench:   3,225,215 ns/iter (+/- 773,984)

// With the lookback version of my algorithm that is way better readable. (sort of stolen btw)
test day5::tests::d5_bench_ex1  ... bench:     179,878 ns/iter (+/- 10,498)
test day5::tests::d5_bench_ex2  ... bench:   1,882,250 ns/iter (+/- 167,912)

test day6::tests::d6_bench_ex1  ... bench:  39,989,280 ns/iter (+/- 8,083,752)
test day6::tests::d6_bench_ex2  ... bench:  14,400,660 ns/iter (+/- 2,441,033)

test day7::tests::d7_bench_ex1  ... bench:       4,991 ns/iter (+/- 2,160)
test day7::tests::d7_bench_ex2  ... bench:       5,903 ns/iter (+/- 2,361)
```

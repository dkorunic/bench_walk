# bench_walk

## About

This is a small benchmarking project to compare different Rust directory walking crates, namely:

- [fts_walkdir](https://github.com/dalance/fts-rs)
- [walkdir](https://github.com/BurntSushi/walkdir)
- [ignore](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore)
- [jwalk](https://github.com/Byron/jwalk)

These are tested against Linux kernel Git repo which is checked out locally during benchmark suite run. All the above crates are tested in single-thread mode as well as multi-thread mode where supported (only ignore and jwalk support parallel execution).

Benchmark suite uses [Criterion](https://github.com/bheisler/criterion.rs) for statistically correct benchmarking and is meant for real life comparison between different walking implementations.

## Results

### Hardware

Low-end server 8-core Xeon E5-1630, 4-drive SATA RAID-10 w/ ext4 filesystem

### Duration report

Benchmarks have been split into two groups, bench_serial and bench_parallel for comparison.

| crate                                      | lower bound | upper bound | best estimate |
| ------------------------------------------ | ----------- | ----------- | ------------- |
| bench_serial/fts_walkdir                   | 59.117 ms   | 59.153 ms   | 59.188 ms     |
| bench_serial/walkdir                       | 54.913 ms   | 54.964 ms   | 55.019 ms     |
| bench_serial/ignore (serial unsorted)      | 59.495 ms   | 59.563 ms   | 59.650 ms     |
| bench_serial/jwalk (serial unsorted)       | 61.733 ms   | 61.823 ms   | 61.918 ms     |
| bench_parallel/ignore (n threads unsorted) | 18.937 ms   | 18.949 ms   | 18.960 ms     |
| bench_parallel/jwalk (n threads, unsorted) | 19.454 ms   | 19.467 ms   | 19.481 ms     |

### Graphs

![](bench_serial_violin.svg)
![](bench_parallel_violin.svg)

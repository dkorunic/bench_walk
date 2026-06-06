# bench_walk

## About

This is a small benchmarking project to compare different Rust directory walking crates, namely:

- [fts_walkdir](https://github.com/dalance/fts-rs)
- [walkdir](https://github.com/BurntSushi/walkdir)
- [walkdir_minimal](https://crates.io/crates/walkdir_minimal)
- [ignore](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore)
- [jwalk](https://github.com/Byron/jwalk)
- [fs-walk](https://github.com/0xrawsec/fs-walk)
- [async-walkdir](https://crates.io/crates/async-walkdir)

The system `find` command (GNU findutils) is also benchmarked as a non-crate baseline reference.

These are tested against Linux kernel Git repo which is checked out locally during benchmark suite run. All the above crates are tested in single-thread mode as well as multi-thread mode where supported (only ignore and jwalk support parallel execution).

Benchmark suite uses [Criterion](https://github.com/bheisler/criterion.rs) for statistically correct benchmarking and is meant for real life comparison between different walking implementations.

## Results

### Hardware

Low-end server 8-core Xeon E5-1630, 4-drive SATA RAID-10 w/ ext4 filesystem

### Duration report

Benchmarks have been split into two groups, bench_serial and bench_parallel for comparison.

| crate                                      | lower bound | best estimate | upper bound |
| ------------------------------------------ | ----------- | ------------- | ----------- |
| bench_serial/find                          | 93.435 ms   | 93.675 ms     | 93.926 ms   |
| bench_serial/fts_walkdir                   | 82.927 ms   | 83.157 ms     | 83.396 ms   |
| bench_serial/walkdir                       | 69.334 ms   | 69.501 ms     | 69.673 ms   |
| bench_serial/walkdir_minimal               | 70.620 ms   | 70.814 ms     | 71.005 ms   |
| bench_serial/ignore (serial unsorted)      | 74.262 ms   | 74.443 ms     | 74.630 ms   |
| bench_serial/jwalk (serial unsorted)       | 79.756 ms   | 80.066 ms     | 80.391 ms   |
| bench_serial/fs_walk (serial unsorted)     | 452.30 ms   | 452.99 ms     | 453.76 ms   |
| bench_serial/async-walkdir (block_on)      | 1.5376 s    | 1.5392 s      | 1.5408 s    |
| bench_parallel/ignore (n threads unsorted) | 24.265 ms   | 24.284 ms     | 24.301 ms   |
| bench_parallel/jwalk (n threads, unsorted) | 26.382 ms   | 26.405 ms     | 26.431 ms   |

> `async-walkdir` is an async `Stream`, so its timing includes per-iteration `block_on` executor and blocking-IO thread-pool overhead from driving it in synchronous benchmark code, rather than pure traversal cost.

### Graphs

![](bench_serial_violin.svg)
![](bench_parallel_violin.svg)

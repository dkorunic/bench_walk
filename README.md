# bench_walk

## About

This is a small benchmarking project that compares different Rust directory-walking crates, namely:

- [fts_walkdir](https://crates.io/crates/fts)
- [walkdir](https://crates.io/crates/walkdir)
- [walkdir_minimal](https://crates.io/crates/walkdir_minimal)
- [ignore](https://crates.io/crates/ignore)
- [jwalk](https://crates.io/crates/jwalk)
- [fs-walk](https://crates.io/crates/fs-walk)
- [async-walkdir](https://crates.io/crates/async-walkdir)

The system `find` command (GNU findutils) is also benchmarked as a non-crate baseline.

All crates are tested against the Linux kernel Git repository, which is checked out locally during the benchmark run. Each crate is exercised in single-threaded mode, and additionally in multi-threaded mode where supported (only `ignore` and `jwalk` offer parallel traversal).

The benchmark suite uses [Criterion](https://github.com/bheisler/criterion.rs) for statistically rigorous measurement, and is intended as a real-life comparison between the different walking implementations.

## Results

### Hardware

Low-end server, 8-core Xeon E5-1630 v3, 4-drive SATA RAID-10 with ext4 filesystem.

### Duration report

Benchmarks are split into two groups for comparison: `bench_serial` (single-threaded) and `bench_parallel` (multi-threaded).

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

# bench_walk

## About

This is a small benchmarking project that compares different Rust directory-walking crates, namely:

- [fts_walkdir](https://crates.io/crates/fts)
- [walkdir](https://crates.io/crates/walkdir)
- [walkdir_minimal](https://crates.io/crates/walkdir_minimal)
- [isideload-walkdir](https://crates.io/crates/isideload-walkdir)
- [walker](https://crates.io/crates/walker)
- [ignore](https://crates.io/crates/ignore)
- [jwalk](https://crates.io/crates/jwalk)
- [fs-walk](https://crates.io/crates/fs-walk)
- [scandir](https://crates.io/crates/scandir)
- [swdir](https://crates.io/crates/swdir)
- [fsindex](https://crates.io/crates/fsindex)
- [async-walkdir](https://crates.io/crates/async-walkdir)

The system `find` command (GNU findutils) is also benchmarked as a non-crate baseline.

All crates are tested against the Linux kernel Git repository, which is checked out locally during the benchmark run. Each crate is exercised in single-threaded mode, and additionally in multi-threaded mode where supported (`ignore`, `jwalk`, `scandir`, `swdir` and `fsindex` offer parallel traversal; `scandir` and `swdir` traverse in parallel internally and so appear only in the multi-threaded group).

Every crate is configured for the fastest possible *full-tree* traversal: sorting is disabled (`jwalk`, `fs-walk` and `swdir` default to unsorted; `swdir` is pinned to raw filesystem order), optional per-entry metadata is not requested (`fts_walkdir` uses `no_metadata`, `scandir` runs without extended metadata and without retaining entries), symlink-following is left off, and any default filtering that would skip part of the tree is removed so all crates walk the same set of files. Concretely: the default `skip_hidden` is turned off for both `jwalk` and `scandir`, `swdir`'s default hidden-file filter is cleared, and `fsindex` is told to ignore `.gitignore`, include hidden files and skip reading file contents (so it measures traversal rather than I/O). Note that `fsindex` is itself built on top of `ignore` and yields only files (not directories), while computing per-file metadata.

The benchmark suite uses [Criterion](https://github.com/bheisler/criterion.rs) for statistically rigorous measurement, and is intended as a real-life comparison between the different walking implementations.

## Results

### Hardware

Low-end server, 8-core Xeon E5-1630 v3, 4-drive SATA RAID-10 with ext4 filesystem.

Measured with Criterion (80 s warm-up, 400 s measurement per benchmark) against a shallow (`--depth 1`) clone of the mainline Linux kernel tree. The filesystem cache is warm, so these numbers reflect in-memory traversal cost rather than cold-cache disk seeks.

### Duration report

Benchmarks are split into two groups for comparison: `bench_serial` (single-threaded) and `bench_parallel` (multi-threaded).

Numbers below are the best estimate plus the 95% confidence interval reported by Criterion, sorted from fastest to slowest within each group.

| crate                                      | lower bound | best estimate | upper bound |
| ------------------------------------------ | ----------- | ------------- | ----------- |
| bench_serial/walkdir                       | 68.161 ms   | 68.393 ms     | 68.628 ms   |
| bench_serial/walkdir_minimal               | 69.529 ms   | 69.769 ms     | 70.022 ms   |
| bench_serial/ignore (serial unsorted)      | 73.172 ms   | 73.457 ms     | 73.754 ms   |
| bench_serial/jwalk (serial unsorted)       | 76.746 ms   | 77.011 ms     | 77.300 ms   |
| bench_serial/fts_walkdir                   | 81.297 ms   | 81.623 ms     | 81.963 ms   |
| bench_serial/find                          | 90.423 ms   | 90.621 ms     | 90.829 ms   |
| bench_serial/walker                        | 219.13 ms   | 219.61 ms     | 220.13 ms   |
| bench_serial/fsindex (serial)              | 222.58 ms   | 223.38 ms     | 224.26 ms   |
| bench_serial/isideload-walkdir             | 340.98 ms   | 341.86 ms     | 342.79 ms   |
| bench_serial/fs_walk (serial unsorted)     | 446.17 ms   | 446.82 ms     | 447.57 ms   |
| bench_serial/async-walkdir (block_on)      | 1.5417 s    | 1.5435 s      | 1.5452 s    |
| bench_parallel/ignore (n threads unsorted) | 24.714 ms   | 24.728 ms     | 24.742 ms   |
| bench_parallel/swdir (n threads)           | 25.209 ms   | 25.228 ms     | 25.247 ms   |
| bench_parallel/jwalk (n threads, unsorted) | 26.332 ms   | 26.349 ms     | 26.366 ms   |
| bench_parallel/scandir (n threads)         | 64.342 ms   | 64.382 ms     | 64.423 ms   |
| bench_parallel/fsindex (parallel)          | 123.56 ms   | 123.96 ms     | 124.38 ms   |

### Analysis

**Single-threaded.** The results fall into three tiers. The `readdir`-based walkers cluster tightly at the top: `walkdir` (68 ms) and `walkdir_minimal` (70 ms) are fastest, followed closely by `ignore` (73 ms) and `jwalk` in serial mode (77 ms). All four read each directory once and use the `d_type` field returned by `readdir(3)` to tell files from directories, so they recurse without a single extra `stat`. `fts_walkdir` (82 ms) wraps the C `fts(3)` routines through FFI and pays a small crossing cost, and the `find` baseline (91 ms) additionally pays process-spawn and output-formatting overhead.

The second tier is several times slower because each crate issues a syscall *per entry* that the leaders avoid. `walker` (220 ms) calls `std::fs::metadata()` on every path to decide whether to recurse, turning one `readdir` per directory into one `readdir` plus one `stat` per entry. `fsindex` (223 ms) is built on top of `ignore`, but it `stat`s every file to populate its metadata struct and yields only files. `isideload-walkdir` (342 ms) is a `walkdir` fork that routes all I/O through the `isideload_vfs` virtual-filesystem abstraction and takes per-entry metadata — portability traded for raw speed. `fs_walk` (447 ms) calls both `is_symlink()` and `is_dir()` (up to two `stat`s) per entry and allocates a `PathBuf` for each.

`async-walkdir` (1.54 s) is in a tier of its own. It is an async `Stream`, so driving it from synchronous benchmark code via `block_on` adds per-item executor scheduling and blocking-I/O thread-pool hand-off on top of the traversal itself; the number reflects that harness cost, not pure walking.

**Multi-threaded.** On 8 physical cores the three work-stealing walkers are essentially tied at the top — `ignore` (24.7 ms), `swdir` (25.2 ms) and `jwalk` (26.3 ms) — each roughly **2.7× faster** than the quickest serial walker. The speedup is sublinear because directory traversal is dominated by I/O and syscall latency rather than CPU, so cores spend much of their time waiting. `scandir` (64 ms) also walks in parallel (via a `jwalk` fork) but streams results through a channel to a background thread and aggregates them into a table-of-contents, and that coordination overhead leaves it ~2.5× behind the leaders. `fsindex` (124 ms) parallelizes only the per-file metadata work; its actual directory walk is the serial `ignore` traversal, and it still `stat`s every file, so it benefits least from the extra cores.

**Takeaway.** For plain recursive traversal, prefer a `readdir`/`d_type` walker (`walkdir` serially, `ignore` or `jwalk` in parallel). Crates that `stat` every entry, add an abstraction layer, or read file contents pay for it linearly in the number of entries — useful features when you need them, but measurable overhead when you only want the paths.

### Graphs

![](bench_serial_violin.svg)
![](bench_parallel_violin.svg)

use bench_walk::{
    async_walkdir, find_walkdir, fs_walk_serial, fts_walkdir, ignore_parallel,
    ignore_serial, jwalk_parallel, jwalk_serial, prepare_test_dir,
    regular_walkdir, walkdir_minimal,
};
use criterion::{criterion_group, criterion_main, Criterion};
use rm_rf::ensure_removed;
use std::hint::black_box;
use std::time::Duration;

const TEST_DIR: &str = "benches/linux_root";
const WARMUP_TIME: u64 = 80;
const MEASURE_TIME: u64 = 400;

/// Runs the serial and parallel directory-walking benchmark groups.
///
/// # Panics
///
/// Panics if the benchmark test directory cannot be created or removed.
pub fn bench_walkdir(c: &mut Criterion) {
    let work_dir =
        prepare_test_dir(TEST_DIR).expect("Unable to create bench directory");

    let mut g = c.benchmark_group("bench_serial");

    // single-thread tests
    g.bench_function("find", |b| {
        b.iter(|| find_walkdir(black_box(&work_dir)));
    });
    g.bench_function("fts_walkdir", |b| {
        b.iter(|| fts_walkdir(black_box(&work_dir)));
    });
    g.bench_function("walkdir", |b| {
        b.iter(|| regular_walkdir(black_box(&work_dir)));
    });
    g.bench_function("walkdir_minimal", |b| {
        b.iter(|| walkdir_minimal(black_box(&work_dir)));
    });
    g.bench_function("ignore (serial unsorted)", |b| {
        b.iter(|| ignore_serial(black_box(&work_dir)));
    });
    g.bench_function("jwalk (serial unsorted)", |b| {
        b.iter(|| jwalk_serial(black_box(&work_dir)));
    });
    g.bench_function("fs_walk (serial unsorted)", |b| {
        b.iter(|| fs_walk_serial(black_box(&work_dir)));
    });
    g.bench_function("async-walkdir (block_on)", |b| {
        b.iter(|| async_walkdir(black_box(&work_dir)));
    });

    g.finish();

    g = c.benchmark_group("bench_parallel");

    // multi-thread tests
    g.bench_function("ignore (n threads unsorted)", |b| {
        b.iter(|| ignore_parallel(black_box(&work_dir)));
    });
    g.bench_function("jwalk (n threads, unsorted)", |b| {
        b.iter(|| jwalk_parallel(black_box(&work_dir)));
    });

    g.finish();

    ensure_removed(work_dir).expect("Unable to remove bench directory");
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(WARMUP_TIME))
        .measurement_time(Duration::from_secs(MEASURE_TIME));
    targets = bench_walkdir
}
criterion_main!(benches);

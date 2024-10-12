use bench_walk::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rm_rf::ensure_removed;
use std::time::Duration;

const TEST_DIR: &str = "benches/linux_root";
const WARMUP_TIME: u64 = 80;
const MEASURE_TIME: u64 = 400;

pub fn bench_walkdir(c: &mut Criterion) {
    let work_dir =
        prepare_test_dir(TEST_DIR).expect("Unable to create bench directory");

    let mut g = c.benchmark_group("bench_serial");

    // single-thread tests
    g.bench_function("fts_walkdir", |b| {
        b.iter(|| black_box(fts_walkdir(&work_dir)))
    });
    g.bench_function("walkdir", |b| {
        b.iter(|| black_box(regular_walkdir(&work_dir)))
    });
    g.bench_function("ignore (serial unsorted)", |b| {
        b.iter(|| black_box(ignore_serial(&work_dir)))
    });
    g.bench_function("jwalk (serial unsorted)", |b| {
        b.iter(|| black_box(jwalk_serial(&work_dir)))
    });

    g.finish();

    g = c.benchmark_group("bench_parallel");

    // multi-thread tests
    g.bench_function("ignore (n threads unsorted)", |b| {
        b.iter(|| black_box(ignore_parallel(&work_dir)))
    });
    g.bench_function("jwalk (n threads, unsorted)", |b| {
        b.iter(|| black_box(jwalk_parallel(&work_dir)))
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

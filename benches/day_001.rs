use aoc::sonar::Report;
use aoc_helpers::load_input;
use criterion::{black_box, criterion_group, Criterion};

use std::convert::TryFrom;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("001 sonar sweep");
    group.bench_function("part 1 counting increases", |b| {
        let lines = load_input("001").expect("could not load input");
        let report = Report::try_from(lines).expect("invalid input");

        b.iter(|| black_box(report.count_increases()))
    });
    group.bench_function("part 2 counting windowed increases", |b| {
        let lines = load_input("001").expect("could not load input");
        let report = Report::try_from(lines).expect("invalid input");

        b.iter(|| black_box(report.count_windowed_increases()))
    });
    group.finish();
}

criterion_group!(benches, bench);

use aoc::util::load_input;
use criterion::{black_box, criterion_group, BenchmarkId, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("001 expense report");
    group.bench_function("part 1 example", |b| {
        b.iter(|| todo!())
    });
    group.finish();
}

criterion_group!(benches, bench);

use aoc::trench::Enhancer;
use aoc_helpers::load_input;
use criterion::{criterion_group, Criterion};
use std::convert::TryFrom;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("020 trench map");
    group.bench_function("part 1 enhanced twice", |b| {
        let lines = load_input("020").expect("could not load input");
        let enhancer = Enhancer::try_from(lines).expect("could not parse input");

        b.iter(|| {
            let mut enhancer = enhancer.clone();
            enhancer.enhance_times(2).num_lit();
        })
    });
    group.bench_function("part 2 enhanced fifty times", |b| {
        let lines = load_input("020").expect("could not load input");
        let enhancer = Enhancer::try_from(lines).expect("could not parse input");

        b.iter(|| {
            let mut enhancer = enhancer.clone();
            enhancer.enhance_times(50).num_lit();
        })
    });
    group.finish();
}

criterion_group!(benches, bench);

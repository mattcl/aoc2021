use aoc::scanner::Mapper;
use aoc_helpers::load_input;
use criterion::{criterion_group, Criterion};
use rustc_hash::FxHashSet;
use std::convert::TryFrom;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("019 beacon scanner");
    group.bench_function("combined solution", |b| {
        let lines = load_input("019").expect("could not load input");
        let mapper = Mapper::try_from(lines).expect("could not parse input");
        b.iter(|| {
            let mut mapper = mapper.clone();
            let mut beacons = FxHashSet::default();
            mapper.correlate(&mut beacons);
            mapper
                .largest_distance()
                .expect("could not find largest distance");
        })
    });
    group.finish();
}

criterion_group!(benches, bench);

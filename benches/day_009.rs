use aoc::{heightmap::HeightMap, util::load_input};
use criterion::{criterion_group, Criterion};
use std::convert::TryFrom;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("009 smoke basin");
    group.bench_function("part 1 finding lowpoints", |b| {
        let lines = load_input("009").expect("could not load input");
        let hm = HeightMap::try_from(lines).expect("could not parse heightmap");

        b.iter(|| hm.total_risk())
    });
    group.bench_function("part 2 finding basins", |b| {
        let lines = load_input("009").expect("could not load input");
        let hm = HeightMap::try_from(lines).expect("could not parse heightmap");

        b.iter(|| hm.largest_basins().expect("coudl not find largest basins"))
    });
    group.finish();
}

criterion_group!(benches, bench);

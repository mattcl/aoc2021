use aoc::amphipod::{LargeBurrow, SmallBurrow};
use aoc_helpers::load_input;
use criterion::{criterion_group, Criterion};
use std::convert::TryFrom;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("023 amphipod");
    group.bench_function("part 1 small burrow", |b| {
        let lines = load_input("023").expect("could not load input");
        let burrow = SmallBurrow::try_from(lines.clone()).expect("could not parse input");

        b.iter(|| burrow.minimize().expect("could not solve"))
    });
    group.bench_function("part 2 large burrow", |b| {
        let lines = load_input("023").expect("could not load input");
        let burrow = LargeBurrow::try_from(lines).expect("could not parse input");

        b.iter(|| burrow.minimize().expect("could not solve"))
    });
    group.finish();
}

criterion_group!(benches, bench);

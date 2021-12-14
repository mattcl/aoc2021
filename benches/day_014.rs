use aoc::{polymer::Polymerizer, util::load_input};
use criterion::{criterion_group, Criterion};
use std::convert::TryFrom;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("014 extended polymerization");
    group.bench_function("part 1 10 iterations", |b| {
        let lines = load_input("014").expect("could not load input");
        let poly = Polymerizer::try_from(lines).expect("could not parse input");

        b.iter(|| poly.iterations(10))
    });
    group.bench_function("part 2 40 iterations", |b| {
        let lines = load_input("014").expect("could not load input");
        let poly = Polymerizer::try_from(lines).expect("could not parse input");

        b.iter(|| poly.iterations(40))
    });
    group.finish();
}

criterion_group!(benches, bench);

use aoc::{
    crab::{ArithmeticSub, LinearSub, Swarm},
    util::load_input,
};
use criterion::{criterion_group, Criterion};

use std::str::FromStr;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("007 the treachery of whales ");
    group.bench_function("part 1 linear cost", |b| {
        let lines = load_input("007").expect("could not load input");
        let line = lines.first().expect("input was empty");
        let swarm: Swarm<LinearSub> = Swarm::from_str(&line).expect("Could not make swarm");

        b.iter(|| swarm.cheapest_expenditure())
    });
    group.bench_function("part 2 arithmetic cost", |b| {
        let lines = load_input("007").expect("could not load input");
        let line = lines.first().expect("input was empty");
        let swarm: Swarm<ArithmeticSub> = Swarm::from_str(&line).expect("Could not make swarm");

        b.iter(|| swarm.cheapest_expenditure())
    });
    group.finish();
}

criterion_group!(benches, bench);

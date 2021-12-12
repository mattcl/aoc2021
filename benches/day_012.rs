use aoc::{cave::CaveSystem, util::load_input};
use criterion::{criterion_group, Criterion};
use std::convert::TryFrom;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("012 passage pathing");
    group.bench_function("part 1 visiting small caves at most once", |b| {
        let lines = load_input("012").expect("could not load input");
        let cave_system = CaveSystem::try_from(lines).expect("could not parse input");

        b.iter(|| cave_system.paths_fast(false).expect("could not find paths"));
    });
    group.bench_function("part 2 allowing a single double visit", |b| {
        let lines = load_input("012").expect("could not load input");
        let cave_system = CaveSystem::try_from(lines).expect("could not parse input");

        b.iter(|| cave_system.paths_fast(true).expect("could not find paths"));
    });
    group.finish();
}

criterion_group!(benches, bench);

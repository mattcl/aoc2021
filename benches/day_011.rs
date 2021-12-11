use aoc::{octopus::Grid, util::load_input};
use criterion::{black_box, criterion_group, Criterion};
use std::convert::TryFrom;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("011 dumbo octopus");
    group.bench_function("part 1 simulating 100 steps", |b| {
        let lines = load_input("011").expect("could not load input");
        let grid = Grid::try_from(lines).expect("could not parse input");

        b.iter(|| {
            let mut grid = grid.clone();
            grid.simulate(black_box(100))
        })
    });
    group.bench_function("part 2 finding the first sync step", |b| {
        let lines = load_input("011").expect("could not load input");
        let grid = Grid::try_from(lines).expect("could not parse input");

        b.iter(|| {
            let mut grid = grid.clone();
            grid.simulate_until_sync()
        })
    });
    group.finish();
}

criterion_group!(benches, bench);

use aoc::{chiton::Grid, generic::Location, util::load_input};
use criterion::{criterion_group, Criterion};
use std::convert::TryFrom;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("015 chiton");
    group.bench_function("part 1 finding safest path", |b| {
        let lines = load_input("015").expect("could not load input");
        let grid = Grid::try_from(lines).expect("could not parse input");
        let scale = 1;

        b.iter(|| {
            grid.shortest(scale, &Location::new(0, 0), &grid.bottom_right())
                .expect("could not find path")
        })
    });
    group.bench_function("part 2 five times as big", |b| {
        let lines = load_input("015").expect("could not load input");
        let grid = Grid::try_from(lines).expect("could not parse input");
        let scale = 5;

        b.iter(|| {
            grid.shortest(
                scale,
                &Location::new(0, 0),
                &grid.scaled_bottom_right(scale),
            )
            .expect("could not find path")
        })
    });
    group.finish();
}

criterion_group!(benches, bench);

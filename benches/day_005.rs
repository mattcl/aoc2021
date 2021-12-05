use aoc::{util::load_input, vents::Grid};
use criterion::{criterion_group, Criterion};

use std::convert::TryFrom;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("005 hydrothermal venture");
    group.bench_function("part 1 only horizontal and vertical", |b| {
        let lines = load_input("005").expect("could not load input");
        let mut grid = Grid::try_from(&lines).expect("Could not construct grid");
        grid.prune_unmappable();
        grid.prune_diagonal();

        b.iter(|| {
            grid.count_multi_overlap();
        })
    });
    group.bench_function("part 2 including perfect diagonal", |b| {
        let lines = load_input("005").expect("could not load input");
        let mut grid = Grid::try_from(&lines).expect("Could not construct grid");
        grid.prune_unmappable();

        b.iter(|| {
            grid.count_multi_overlap();
        })
    });
    group.finish();
}

criterion_group!(benches, bench);

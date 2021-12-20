use aoc::fish::Sim;
use aoc_helpers::load_input;
use criterion::{black_box, criterion_group, Criterion};

use std::str::FromStr;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("006 lanternfish");
    group.bench_function("part 1 80 days/recursive", |b| {
        let lines = load_input("006").expect("could not load input");
        let line = lines.first().expect("Input was empty");
        let sim = Sim::from_str(&line).expect("Could not make sim");

        b.iter(|| {
            sim.population_after(black_box(80));
        })
    });
    group.bench_function("part 1 80 days/counts array", |b| {
        let lines = load_input("006").expect("could not load input");
        let line = lines.first().expect("Input was empty");
        let sim = Sim::from_str(&line).expect("Could not make sim");

        b.iter(|| {
            sim.fast_population_after(black_box(80));
        })
    });
    group.bench_function("part 2 256 days/recursive", |b| {
        let lines = load_input("006").expect("could not load input");
        let line = lines.first().expect("Input was empty");
        let sim = Sim::from_str(&line).expect("Could not make sim");

        b.iter(|| {
            sim.population_after(black_box(256));
        })
    });
    group.bench_function("part 2 256 days/counts array", |b| {
        let lines = load_input("006").expect("could not load input");
        let line = lines.first().expect("Input was empty");
        let sim = Sim::from_str(&line).expect("Could not make sim");

        b.iter(|| {
            sim.fast_population_after(black_box(256));
        })
    });
    group.finish();
}

criterion_group!(benches, bench);

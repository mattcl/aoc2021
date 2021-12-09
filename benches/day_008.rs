use aoc::{ssd::Solver, util::load_input};
use criterion::{criterion_group, Criterion};
use std::convert::TryFrom;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("008 seven segment search");
    group.bench_function("part 1 unambiguous count", |b| {
        let lines = load_input("008").expect("could not load input");
        let solver = Solver::try_from(lines).expect("Could not parse input");

        b.iter(|| solver.rhs_count_known())
    });
    group.bench_function("part 2 solving for digits/sequential", |b| {
        let lines = load_input("008").expect("could not load input");
        let solver = Solver::try_from(lines).expect("Could not parse input");

        b.iter(|| solver.rhs_values_sum().expect("could not find solution"))
    });
    group.bench_function("part 2 solving for digits/prallel", |b| {
        let lines = load_input("008").expect("could not load input");
        let solver = Solver::try_from(lines).expect("Could not parse input");

        b.iter(|| {
            solver
                .par_rhs_values_sum()
                .expect("could not find solution")
        })
    });
    group.finish();
}

criterion_group!(benches, bench);

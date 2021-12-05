use aoc::{bingo::{Runner, Board, FastBoard}, util::load_input};
use criterion::{criterion_group, Criterion};

use std::convert::TryFrom;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("004 giant squid/normal board");
    group.bench_function("part 1 first to score", |b| {
        let lines = load_input("004").expect("could not load input");
        let runner: Runner<Board> = Runner::try_from(lines).expect("Input was invalid");

        b.iter(|| {
            let mut runner = runner.clone();
            runner.play().expect("Could not find a winner");
        })
    });
    group.bench_function("part 2 last to score/sequential", |b| {
        let lines = load_input("004").expect("could not load input");
        let runner: Runner<Board> = Runner::try_from(lines).expect("Input was invalid");

        b.iter(|| {
            let mut runner = runner.clone();
            runner.play_all().last().expect("Could not find a winner");
        })
    });
    group.bench_function("part 2 last to score/parallel", |b| {
        let lines = load_input("004").expect("could not load input");
        let runner: Runner<Board> = Runner::try_from(lines).expect("Input was invalid");

        b.iter(|| {
            let mut runner = runner.clone();
            runner
                .par_find_last_scoring()
                .expect("Could not find a winner");
        })
    });
    group.finish();

    let mut group = c.benchmark_group("004 giant squid/fast board");
    group.bench_function("part 1 first to score", |b| {
        let lines = load_input("004").expect("could not load input");
        let runner: Runner<FastBoard> = Runner::try_from(lines).expect("Input was invalid");

        b.iter(|| {
            let mut runner = runner.clone();
            runner.play().expect("Could not find a winner");
        })
    });
    group.bench_function("part 2 last to score/sequential", |b| {
        let lines = load_input("004").expect("could not load input");
        let runner: Runner<FastBoard> = Runner::try_from(lines).expect("Input was invalid");

        b.iter(|| {
            let mut runner = runner.clone();
            runner.play_all().last().expect("Could not find a winner");
        })
    });
    group.bench_function("part 2 last to score/parallel", |b| {
        let lines = load_input("004").expect("could not load input");
        let runner: Runner<FastBoard> = Runner::try_from(lines).expect("Input was invalid");

        b.iter(|| {
            let mut runner = runner.clone();
            runner
                .par_find_last_scoring()
                .expect("Could not find a winner");
        })
    });
    group.finish();
}

criterion_group!(benches, bench);

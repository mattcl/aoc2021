use aoc::dirac::{DeterministicDie, Game, QuantumGame};
use aoc_helpers::load_input;
use criterion::{criterion_group, Criterion};
use std::convert::TryFrom;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("021 dirac dice");
    group.bench_function("part 1 deterministic", |b| {
        let lines = load_input("021").expect("could not load input");
        let game: Game<DeterministicDie> =
            Game::try_from(lines.as_ref()).expect("could not parse input");

        b.iter(|| {
            let mut game = game.clone();
            game.play().expect("unable to play game");
        })
    });
    group.bench_function("part 2 quantum", |b| {
        let lines = load_input("021").expect("could not load input");
        let quantum = QuantumGame::try_from(lines.as_ref()).expect("could not parse input");
        b.iter(|| quantum.play())
    });
    group.finish();
}

criterion_group!(benches, bench);

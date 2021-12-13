use aoc::{camera::Manual, util::load_input};
use criterion::{criterion_group, Criterion};
use std::convert::TryFrom;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("013 transparent origami");
    group.bench_function("part 1 first fold", |b| {
        let lines = load_input("013").expect("could not load input");
        let manual = Manual::try_from(lines).expect("could not parse input");

        b.iter(|| manual.first_instruction().count_visible())
    });
    group.bench_function("part 2 finding code", |b| {
        let lines = load_input("013").expect("could not load input");
        let manual = Manual::try_from(lines).expect("could not parse input");

        b.iter(|| manual.folded().to_string())
    });
    group.finish();
}

criterion_group!(benches, bench);

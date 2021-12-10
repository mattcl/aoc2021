use aoc::{
    navigation::Program,
    util::{load_input, parse_input},
};
use criterion::{criterion_group, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("010 syntax scoring");
    group.bench_function("part 1 score corrupted", |b| {
        let input = load_input("010").expect("could not load input");
        let program = Program::from(parse_input(&input).expect("could not parse input"));

        b.iter(|| {
            let check = program.check();
            check.score_corruptions();
        })
    });
    group.bench_function("part 2 score completions", |b| {
        let input = load_input("010").expect("could not load input");
        let program = Program::from(parse_input(&input).expect("could not parse input"));

        b.iter(|| {
            let check = program.check();
            check.score_completions();
        })
    });
    group.finish();
}

criterion_group!(benches, bench);

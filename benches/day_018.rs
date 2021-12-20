use aoc::fish::Homework;
use aoc_helpers::load_input;
use criterion::{criterion_group, Criterion};
use std::convert::TryFrom;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("018 snailfish");
    group.bench_function("part 1 finding magnitude of sum", |b| {
        let lines = load_input("018").expect("could not load input");
        let homework = Homework::try_from(lines).expect("could not parse input");

        b.iter(|| homework.sum().expect("could not find sum").magnitude());
    });
    group.bench_function(
        "part 2 finding largest magnitude of any two elements",
        |b| {
            let lines = load_input("018").expect("could not load input");
            let homework = Homework::try_from(lines).expect("could not parse input");

            b.iter(|| {
                homework
                    .largest_magnitude_of_pairs()
                    .expect("could not find magnitude")
            });
        },
    );
    group.finish();
}

criterion_group!(benches, bench);

use std::str::FromStr;

use aoc::probe::{Launcher, Target};
use aoc_helpers::load_input;
use criterion::{criterion_group, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("017 trick shot");
    group.bench_function("combined highest and distinct", |b| {
        let line = load_input("017")
            .expect("could not load input")
            .first()
            .cloned()
            .expect("input was empty");

        let target = Target::from_str(&line).expect("could not parse input");
        let launcher = Launcher {};

        b.iter(|| launcher.launch(&target))
    });
    group.finish();
}

criterion_group!(benches, bench);

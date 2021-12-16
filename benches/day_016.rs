use std::str::FromStr;

use aoc::{decoder::Transmission, util::load_input};
use criterion::{criterion_group, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("016 packet decoder");
    group.bench_function("part 1 summing versions", |b| {
        let line = load_input("016")
            .expect("could not load input")
            .first()
            .cloned()
            .expect("input was empty");

        b.iter(|| {
            let transmission = Transmission::from_str(&line).expect("could not parse input");
            transmission.version_sum()
        })
    });
    group.bench_function("part 2 finding value", |b| {
        let line = load_input("016")
            .expect("could not load input")
            .first()
            .cloned()
            .expect("input was empty");

        b.iter(|| {
            let transmission = Transmission::from_str(&line).expect("could not parse input");
            transmission.value()
        })
    });
    group.finish();
}

criterion_group!(benches, bench);

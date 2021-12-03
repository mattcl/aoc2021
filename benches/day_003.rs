use aoc::{diagnostic::Diagnostic, util::load_input};
use criterion::{criterion_group, Criterion};

use std::convert::TryFrom;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("003 binary diagnostic");
    let lines = load_input("003").expect("could not load input");

    group.bench_function("part 1 power consumption", |b| {
        b.iter(|| {
            let d = Diagnostic::try_from(&lines).expect("Could not create diagnostic");
            d.power_consumption();
        })
    });
    group.bench_function("part 2 life support rating", |b| {
        let d = Diagnostic::try_from(&lines).expect("Could not create diagnostic");

        b.iter(|| {
            d.life_support_rating()
                .expect("Could not get life support rating");
        })
    });
    group.finish();
}

criterion_group!(benches, bench);

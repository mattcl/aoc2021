use aoc::submarine::{AimableSubmarine, Moveable, Submarine};
use aoc_helpers::{load_input, parse_input};
use criterion::{criterion_group, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("002 dive!");
    group.bench_function("part 1 submarine movement", |b| {
        let lines = load_input("002").expect("could not load input");
        let commands = parse_input(&lines).expect("invalid input");

        let sub = Submarine::new();

        b.iter(|| {
            let mut sub = sub.clone();
            for command in &commands {
                sub.execute(command);
            }
            sub.location_hash();
        })
    });
    group.bench_function("part 2 aimable submarine movement", |b| {
        let lines = load_input("002").expect("could not load input");
        let commands = parse_input(&lines).expect("invalid input");

        let sub = AimableSubmarine::new();

        b.iter(|| {
            let mut sub = sub.clone();
            for command in &commands {
                sub.execute(command);
            }
            sub.location_hash();
        })
    });
    group.finish();
}

criterion_group!(benches, bench);

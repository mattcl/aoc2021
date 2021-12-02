use aoc::{
    submarine::{AimableSubmarine, Moveable, Submarine},
    util::{load_input, parse_input},
};
use criterion::{criterion_group, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("002 dive!");
    let lines = load_input("002").expect("could not load input");
    let commands = parse_input(&lines).expect("invalid input");

    let sub = Submarine::new();
    let aimable_sub = AimableSubmarine::new();

    group.bench_function("part 1 submarine movement", |b| {
        b.iter(|| {
            let mut sub = sub.clone();
            for command in &commands {
                sub.execute(command);
            }
            sub.location_hash();
        })
    });
    group.bench_function("part 2 aimable submarine movement", |b| {
        b.iter(|| {
            let mut sub = aimable_sub.clone();
            for command in &commands {
                sub.execute(command);
            }
            sub.location_hash();
        })
    });
    group.finish();
}

criterion_group!(benches, bench);

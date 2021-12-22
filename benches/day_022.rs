use aoc::reactor::{Cuboid, Instructions, Reactor};
use aoc_helpers::load_input;
use criterion::{black_box, criterion_group, Criterion};
use std::convert::TryFrom;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("022 reactor reboot");
    group.bench_function("part 1 region limited", |b| {
        let lines = load_input("022").expect("could not load input");
        let instructions = Instructions::try_from(lines).expect("could not parse input");
        let mut reactor = Reactor::default();
        reactor.reboot(&instructions);

        let limit = Cuboid::new((-50, -50, -50).into(), (50, 50, 50).into());

        b.iter(|| reactor.volume(black_box(&Some(limit))))
    });
    group.bench_function("part 2 total volume", |b| {
        let lines = load_input("022").expect("could not load input");
        let instructions = Instructions::try_from(lines).expect("could not parse input");
        let mut reactor = Reactor::default();
        reactor.reboot(&instructions);

        b.iter(|| reactor.volume(black_box(&None)))
    });
    group.finish();
}

criterion_group!(benches, bench);

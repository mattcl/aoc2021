use aoc::amphipod::Amphipod;
use aoc_helpers::{aoc_bench, Solver};
use criterion::{criterion_group, Criterion};

aoc_bench!{
    day_023,
    Amphipod,
    "part 1 small burrow",
    "part 2 large burrow"
}

criterion_group!(benches, day_023);

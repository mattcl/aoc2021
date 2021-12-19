use std::convert::TryFrom;

use aoc::{
    scanner::Mapper,
    util::{load_input, Solution},
};
use rustc_hash::FxHashSet;

fn main() {
    let lines = load_input("019").expect("could not load input");
    let mut mapper = Mapper::try_from(lines).expect("could not parse input");
    let mut beacons = FxHashSet::default();
    mapper.correlate(&mut beacons);

    println!(
        "{}",
        Solution::new(
            beacons.len(),
            mapper
                .largest_distance()
                .expect("could not find largest distance")
        )
    );
}

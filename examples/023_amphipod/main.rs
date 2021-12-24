use aoc::amphipod::{LargeBurrow, SmallBurrow};
use aoc_helpers::{load_input, Solution};
use std::convert::TryFrom;

fn main() {
    let lines = load_input("023").expect("could not load input");
    let small = SmallBurrow::try_from(lines.clone()).expect("could not parse input");
    let large = LargeBurrow::try_from(lines).expect("could not parse input");

    println!(
        "{}",
        Solution::new(
            small.minimize().expect("could not find solution"),
            large.minimize().expect("could not find solution"),
        )
    );
}

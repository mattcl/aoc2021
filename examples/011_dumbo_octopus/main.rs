use std::convert::TryFrom;

use aoc::{octopus::Grid, util::load_input};

fn main() {
    let lines = load_input("011").expect("could not load input");
    let mut grid = Grid::try_from(lines).expect("could not parse input");

    println!("part 1: {}", grid.simulate(100));
    println!("part 2: {}", grid.simulate_until_sync());
}

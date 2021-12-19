use std::convert::TryFrom;

use aoc::{
    octopus::OctopusGrid,
    util::{load_input, Solution},
};

fn main() {
    let lines = load_input("011").expect("could not load input");
    let mut grid = OctopusGrid::try_from(lines).expect("could not parse input");

    println!(
        "{}",
        Solution::new(grid.simulate(100), grid.simulate_until_sync())
    );
}

use std::convert::TryFrom;

use aoc::{
    octopus::Grid,
    util::{load_input, Solution},
};

fn main() {
    let lines = load_input("011").expect("could not load input");
    let mut grid = Grid::try_from(lines).expect("could not parse input");

    println!(
        "{}",
        Solution::new(grid.simulate(100), grid.simulate_until_sync())
    );
}

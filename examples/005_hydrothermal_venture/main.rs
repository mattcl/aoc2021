use std::convert::TryFrom;

use aoc::vents::Vents;
use aoc_helpers::{load_input, Solution};

fn main() {
    let lines = load_input("005").expect("could not load input");
    let mut grid = Vents::try_from(&lines).expect("Could not construct grid");
    let part2 = grid.count_multi_overlap();
    grid.prune_diagonal();

    println!("{}", Solution::new(grid.count_multi_overlap(), part2));
}

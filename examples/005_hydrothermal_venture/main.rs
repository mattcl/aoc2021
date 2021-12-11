use std::convert::TryFrom;

use aoc::{
    util::{load_input, Solution},
    vents::Grid,
};

fn main() {
    let lines = load_input("005").expect("could not load input");
    let mut grid = Grid::try_from(&lines).expect("Could not construct grid");
    let part2 = grid.count_multi_overlap();
    grid.prune_diagonal();

    println!("{}", Solution::new(grid.count_multi_overlap(), part2));
}

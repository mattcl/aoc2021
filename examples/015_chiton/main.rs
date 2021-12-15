use std::convert::TryFrom;

use aoc::{
    chiton::Grid,
    generic::Location,
    util::{load_input, Solution},
};

fn main() {
    let lines = load_input("015").expect("could not load input");
    let grid = Grid::try_from(lines).expect("could not parse input");

    println!(
        "{}",
        Solution::new(
            grid.shortest(1, &Location::new(0, 0), &grid.bottom_right())
                .expect("could not find cheapest path"),
            grid.shortest(5, &Location::new(0, 0), &grid.scaled_bottom_right(5))
                .expect("could not find cheapest path"),
        )
    );
}

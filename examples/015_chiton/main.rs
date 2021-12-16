use std::convert::TryFrom;

use aoc::{
    chiton::{ChitonGrid, Pathfinding},
    generic::prelude::*,
    util::{load_input, Solution},
};

fn main() {
    let lines = load_input("015").expect("could not load input");
    let grid = ChitonGrid::try_from(lines).expect("could not parse input");

    println!(
        "{}",
        Solution::new(
            grid.shortest(1, &grid.top_left(), &grid.bottom_right())
                .expect("could not find cheapest path"),
            grid.shortest(5, &grid.top_left(), &grid.scaled_bottom_right(5))
                .expect("could not find cheapest path"),
        )
    );
}

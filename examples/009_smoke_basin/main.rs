use std::convert::TryFrom;

use aoc::{
    heightmap::HeightMap,
    util::{load_input, Solution},
};

fn main() {
    let lines = load_input("009").expect("could not load input");
    let hm = HeightMap::try_from(lines).expect("could not parse heightmap");

    println!(
        "{}",
        Solution::new(
            hm.total_risk(),
            hm.largest_basins().expect("could not find largest basins")
        )
    );
}

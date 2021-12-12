use std::convert::TryFrom;

use aoc::{
    cave::CaveSystem,
    util::{load_input, Solution},
};

fn main() {
    let lines = load_input("012").expect("could not load input");
    let cave_system = CaveSystem::try_from(lines).expect("could not parse input");
    let paths = cave_system.paths_fast(false).expect("could not find paths");
    let multi_paths = cave_system.paths_semi_par(true).expect("could not find paths");

    println!("{}", Solution::new(paths, multi_paths));
}

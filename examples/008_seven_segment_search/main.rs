use std::convert::TryFrom;

use aoc::{util::load_input, ssd::Solver};

fn main() {
    let lines = load_input("008").expect("could not load input");
    let solver = Solver::try_from(lines).expect("Could not parse input");

    println!("part 1: {}", solver.rhs_count_known());
    println!("part 2: {}", solver.rhs_values_sum().expect("Unable to find solution"));
}

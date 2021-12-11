use std::convert::TryFrom;

use aoc::{
    ssd::Solver,
    util::{load_input, Solution},
};

fn main() {
    let lines = load_input("008").expect("could not load input");
    let solver = Solver::try_from(lines).expect("Could not parse input");

    println!(
        "{}",
        Solution::new(
            solver.rhs_count_known(),
            solver.rhs_values_sum().expect("Unable to find solution")
        )
    );
}

use std::str::FromStr;

use aoc::crab::{ArithmeticSub, LinearSub, Swarm};
use aoc_helpers::{load_input, Solution};

fn main() {
    let lines = load_input("007").expect("could not load input");
    let line = lines.first().expect("input was empty");
    let linear_swarm: Swarm<LinearSub> = Swarm::from_str(&line).expect("Could not make swarm");
    let arithmetic_swarm: Swarm<ArithmeticSub> =
        Swarm::from_str(&line).expect("Could not make swarm");

    println!(
        "{}",
        Solution::new(
            linear_swarm.cheapest_expenditure(),
            arithmetic_swarm.cheapest_expenditure()
        )
    );
}

use aoc::fish::Sim;
use aoc_helpers::{load_input, Solution};
use std::str::FromStr;

fn main() {
    let lines = load_input("006").expect("could not load input");
    let line = lines.first().expect("Input was empty");

    let sim = Sim::from_str(&line).expect("Could not make sim");

    println!(
        "{}",
        Solution::new(
            sim.fast_population_after(80),
            sim.fast_population_after(256)
        )
    );
}

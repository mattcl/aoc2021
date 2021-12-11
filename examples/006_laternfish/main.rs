use aoc::{
    fish::Sim,
    util::{load_input, Solution},
};
use std::str::FromStr;

fn main() {
    let lines = load_input("006").expect("could not load input");
    let line = lines.first().expect("Input was empty");

    let sim = Sim::from_str(&line).expect("Could not make sim");

    println!(
        "{}",
        Solution::new(sim.population_after(80), sim.population_after(256))
    );
}

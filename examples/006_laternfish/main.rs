use aoc::{fish::Sim, util::load_input};
use std::str::FromStr;

fn main() {
    let lines = load_input("006").expect("could not load input");
    let line = lines.first().expect("Input was empty");

    let sim = Sim::from_str(&line).expect("Could not make sim");

    println!("part 1: {}", sim.population_after(80));
    println!("part 2: {}", sim.population_after(256));
}

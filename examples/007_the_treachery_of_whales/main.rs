use std::str::FromStr;

use aoc::{util::load_input, crab::{ArithmeticSub, LinearSub, Swarm}};

fn main() {
    let lines = load_input("007").expect("could not load input");
    let line = lines.first().expect("input was empty");
    let linear_swarm: Swarm<LinearSub> = Swarm::from_str(&line).expect("Could not make swarm");
    let arithmetic_swarm: Swarm<ArithmeticSub> = Swarm::from_str(&line).expect("Could not make swarm");

    println!("part 1: {}", linear_swarm.cheapest_expenditure());
    println!("part 2: {}", arithmetic_swarm.cheapest_expenditure());
}

use std::convert::TryFrom;

use aoc::trench::Enhancer;
use aoc_helpers::{load_input, Solution};

fn main() {
    let lines = load_input("020").expect("could not load input");
    let mut enhancer = Enhancer::try_from(lines).expect("could not parse input");
    let num_lit_2 = enhancer.enhance_times(2).num_lit();
    let num_lit_50 = enhancer.enhance_times(48).num_lit();

    println!("{}", Solution::new(num_lit_2, num_lit_50));
}

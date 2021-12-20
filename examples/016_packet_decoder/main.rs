use std::str::FromStr;

use aoc::decoder::Transmission;
use aoc_helpers::{load_input, Solution};

fn main() {
    let line = load_input("016")
        .expect("could not load input")
        .first()
        .cloned()
        .expect("input was empty");

    let transmission = Transmission::from_str(&line).expect("could not parse input");

    println!(
        "{}",
        Solution::new(transmission.version_sum(), transmission.value(),)
    );
}

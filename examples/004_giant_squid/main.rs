use aoc::{util::load_input, bingo::Runner};
use std::convert::TryFrom;

fn main() {
    let lines = load_input("004").expect("could not load input");
    let mut runner = Runner::try_from(&lines).expect("Input was invalid");

    println!("part 1: {}", runner.play().expect("Could not find a winner"));

    let mut runner = Runner::try_from(&lines).expect("Input was invalid");
    let score = runner.par_find_last_scoring().expect("Could not find the last winner");
    println!("part 2: {}", score);
}

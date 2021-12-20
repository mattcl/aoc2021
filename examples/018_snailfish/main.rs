use std::convert::TryFrom;

use aoc::fish::Homework;
use aoc_helpers::{load_input, Solution};

fn main() {
    let lines = load_input("018").expect("could not load input");
    let homework = Homework::try_from(lines).expect("could not parse input");
    let sum = homework.sum().expect("could not find sum");
    let mag = homework
        .largest_magnitude_of_pairs()
        .expect("could not find magnitude");

    println!("{}", Solution::new(sum.magnitude(), mag));
}

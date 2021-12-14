use std::convert::TryFrom;

use aoc::{
    polymer::Polymerizer,
    util::{load_input, Solution},
};

fn main() {
    let lines = load_input("014").expect("could not load input");
    let poly = Polymerizer::try_from(lines).expect("could not parse input");

    let p1 = poly.iterations_fast(10);
    let p2 = poly.iterations_fast(40);

    println!("{}", Solution::new(p1, p2));
}

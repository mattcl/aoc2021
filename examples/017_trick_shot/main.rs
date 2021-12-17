use std::str::FromStr;

use aoc::{
    probe::{Launcher, Target},
    util::{load_input, Solution},
};

fn main() {
    let line = load_input("017")
        .expect("could not load input")
        .first()
        .cloned()
        .expect("input was empty");

    let target = Target::from_str(&line).expect("could not parse input");
    let launcher = Launcher {};
    let (highest, distinct) = launcher.launch(&target);

    println!("{}", Solution::new(highest, distinct));
}

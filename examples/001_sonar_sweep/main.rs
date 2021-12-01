use aoc::sonar::Report;
use aoc::util::load_input;

use std::convert::TryFrom;

fn main() {
    let lines = load_input("001").expect("could not load input");
    let report = Report::try_from(lines).expect("invalid input");

    println!("part 1: {}", report.count_increases());
    println!("part 2: {}", report.count_windowed_increases());
}

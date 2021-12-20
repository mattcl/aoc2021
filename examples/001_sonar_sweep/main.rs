use aoc::sonar::Report;
use aoc_helpers::{load_input, Solution};

use std::convert::TryFrom;

fn main() {
    let lines = load_input("001").expect("could not load input");
    let report = Report::try_from(lines).expect("invalid input");

    println!(
        "{}",
        Solution::new(report.count_increases(), report.count_windowed_increases())
    );
}

use aoc::{bingo::{FastBoard, Runner}, util::load_input};
use std::convert::TryFrom;

fn main() {
    let lines = load_input("004").expect("could not load input");
    let mut runner: Runner<FastBoard> = Runner::try_from(lines).expect("Input was invalid");

    println!(
        "part 1: {}",
        runner.play().expect("Could not find a winner")
    );

    let scores = runner
        .play_all();

    let score = scores
        .last()
        .expect("Could not find the last winner");
    println!("part 2: {}", score);
}

use aoc::{
    bingo::{FastBoard, Runner},
    util::{load_input, Solution},
};
use std::convert::TryFrom;

fn main() {
    let lines = load_input("004").expect("could not load input");
    let mut runner: Runner<FastBoard> = Runner::try_from(lines).expect("Input was invalid");

    println!(
        "{}",
        Solution::new(
            runner.play().expect("Could not find a winner"),
            runner
                .play_all()
                .last()
                .expect("Could not find the last winner")
        )
    );
}

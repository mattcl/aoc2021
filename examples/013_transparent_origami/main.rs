use std::convert::TryFrom;

use aoc::{
    camera::Manual,
    util::{load_input, Solution},
};

fn main() {
    let lines = load_input("013").expect("could not load input");
    let manual = Manual::try_from(lines).expect("could not parse input");
    let first_instruction = manual.first_instruction();

    println!(
        "{}",
        Solution::new(
            first_instruction.count_visible(),
            manual.folded().to_string()
        )
    );
}

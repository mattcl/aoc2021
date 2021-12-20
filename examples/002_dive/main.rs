use aoc::submarine::{AimableSubmarine, Moveable, Submarine};
use aoc_helpers::{load_input, parse_input, Solution};

fn main() {
    let lines = load_input("002").expect("could not load input");
    let commands = parse_input(&lines).expect("invalid input");

    let mut sub = Submarine::new();
    let mut aimable_sub = AimableSubmarine::new();

    for command in &commands {
        sub.execute(command);
        aimable_sub.execute(command);
    }

    println!(
        "{}",
        Solution::new(sub.location_hash(), aimable_sub.location_hash())
    );
}

use std::convert::TryFrom;

use aoc::reactor::{Cuboid, Instructions, Reactor};
use aoc_helpers::{load_input, Solution};

fn main() {
    let lines = load_input("022").expect("could not load input");
    let instructions = Instructions::try_from(lines).expect("could not parse input");
    let mut reactor = Reactor::default();
    reactor.reboot(&instructions);

    let limit = Cuboid::new((-50, -50, -50).into(), (50, 50, 50).into());

    let volume = reactor.volume(&Some(limit));
    let total_volume = reactor.volume(&None);

    println!("{}", Solution::new(volume, total_volume));
}

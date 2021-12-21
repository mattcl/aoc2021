use std::convert::TryFrom;

use aoc::dirac::{DeterministicDie, Game, QuantumGame};
use aoc_helpers::{load_input, Solution};

fn main() {
    let lines = load_input("021").expect("could not load input");
    let mut game: Game<DeterministicDie> =
        Game::try_from(lines.as_ref()).expect("could not parse input");
    let val = game.play().expect("unable to play game");

    let quantum = QuantumGame::try_from(lines.as_ref()).expect("could not parse input");
    let qval = quantum.play();

    println!("{}", Solution::new(val, qval));
}

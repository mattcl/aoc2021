use aoc::{
    navigation::Program,
    util::{load_input, parse_input},
};

fn main() {
    let input = load_input("010").expect("could not load input");
    let program = Program::from(parse_input(&input).expect("could not parse input"));
    let check = program.check();

    println!("part 1: {}", check.score_corruptions());
    println!("part 2: {}", check.score_completions());
}

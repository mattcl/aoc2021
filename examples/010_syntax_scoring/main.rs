use aoc::{
    navigation::Program,
    util::{load_input, parse_input, Solution},
};

fn main() {
    let input = load_input("010").expect("could not load input");
    let program = Program::from(parse_input(&input).expect("could not parse input"));
    let check = program.check();

    println!(
        "{}",
        Solution::new(check.score_corruptions(), check.score_completions())
    );
}

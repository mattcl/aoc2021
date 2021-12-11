use aoc::diagnostic::Diagnostic;
use aoc::util::{load_input, Solution};
use std::convert::TryFrom;

fn main() {
    let lines = load_input("003").expect("could not load input");
    let diagnostic = Diagnostic::try_from(&lines).expect("could not construct diagnositc");

    println!(
        "{}",
        Solution::new(
            diagnostic.power_consumption(),
            diagnostic
                .life_support_rating()
                .expect("could not get life support rating")
        )
    );
}

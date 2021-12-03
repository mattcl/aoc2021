use aoc::diagnostic::Diagnostic;
use aoc::util::load_input;
use std::convert::TryFrom;

fn main() {
    let lines = load_input("003").expect("could not load input");
    let diagnostic = Diagnostic::try_from(&lines).expect("could not construct diagnositc");

    println!("part 1: {}", diagnostic.power_consumption());
    println!(
        "part 2: {}",
        diagnostic
            .life_support_rating()
            .expect("could not get life support rating")
    );
}

use std::{convert::TryFrom, str::FromStr};

use anyhow::Result;
use aoc_helpers::{parse_input, Solver};
use itertools::Itertools;

// So, yeah... I'm not going to apologize for doing this
pub trait Delimiter {
    fn closes(&self, other: &Self) -> bool;
    fn points(&self) -> i64;
}

impl Delimiter for char {
    fn closes(&self, other: &Self) -> bool {
        match other {
            '(' => *self == ')',
            '[' => *self == ']',
            '{' => *self == '}',
            '<' => *self == '>',
            _ => false,
        }
    }

    fn points(&self) -> i64 {
        match self {
            // completion
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            // corrupted
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CheckResult {
    pub corrupted_char: Option<char>,
    pub remaining_openings: Vec<char>,
}

impl CheckResult {
    pub fn is_corrupted(&self) -> bool {
        self.corrupted_char.is_some()
    }

    pub fn score_corrupt(&self) -> i64 {
        self.corrupted_char.map(|ch| ch.points()).unwrap_or(0)
    }

    pub fn score_completion(&self) -> i64 {
        self.remaining_openings
            .iter()
            .rev()
            .fold(0, |acc, ch| acc * 5 + ch.points())
    }
}

impl From<(Option<char>, Vec<char>)> for CheckResult {
    fn from(value: (Option<char>, Vec<char>)) -> Self {
        Self {
            corrupted_char: value.0,
            remaining_openings: value.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    chars: Vec<char>,
}

impl Line {
    pub fn check_corrupt(&self) -> CheckResult {
        let mut remainder = Vec::with_capacity(self.chars.len());
        for ch in self.chars.iter() {
            match ch {
                '(' | '[' | '<' | '{' => {
                    remainder.push(*ch);
                }
                ')' | ']' | '>' | '}' => {
                    if let Some(last) = remainder.pop() {
                        if !ch.closes(&last) {
                            return (Some(*ch), remainder).into();
                        }
                    } else {
                        return (Some(*ch), remainder).into();
                    }
                }
                _ => unreachable!("todo: fix this"),
            };
        }

        (None, remainder).into()
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Line {
            chars: s.chars().collect(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ProgramCheckResult {
    results: Vec<CheckResult>,
}

impl ProgramCheckResult {
    pub fn score_corruptions(&self) -> i64 {
        self.results
            .iter()
            .filter_map(|r| r.corrupted_char.map(|ch| ch.points()))
            .sum()
    }

    pub fn score_completions(&self) -> i64 {
        let scores: Vec<i64> = self
            .results
            .iter()
            .filter_map(|r| {
                if r.is_corrupted() {
                    None
                } else {
                    Some(r.score_completion())
                }
            })
            .sorted()
            .collect();

        let middle = scores.len() / 2;
        scores.get(middle).copied().unwrap_or(0)
    }
}

impl From<Vec<CheckResult>> for ProgramCheckResult {
    fn from(value: Vec<CheckResult>) -> Self {
        ProgramCheckResult { results: value }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    lines: Vec<Line>,
}

impl Program {
    pub fn new(lines: Vec<Line>) -> Self {
        Self { lines }
    }

    pub fn check(&self) -> ProgramCheckResult {
        self.lines
            .iter()
            .map(|l| l.check_corrupt())
            .collect::<Vec<CheckResult>>()
            .into()
    }
}

impl From<Vec<Line>> for Program {
    fn from(value: Vec<Line>) -> Self {
        Program::new(value)
    }
}

impl TryFrom<Vec<String>> for Program {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let lines = parse_input(&value)?;
        Ok(Self::from(lines))
    }
}

impl Solver for Program {
    const ID: &'static str = "syntax scoring";
    const DAY: usize = 10;

    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Self::P1 {
        self.check().score_corruptions()
    }

    fn part_two(&mut self) -> Self::P2 {
        self.check().score_completions()
    }

    // need to override this for combined solution because of the intermediate
    // check result
    fn solve() -> aoc_helpers::Solution<Self::P1, Self::P2> {
        let instance = Self::instance();
        let check = instance.check();
        aoc_helpers::Solution::new(check.score_corruptions(), check.score_completions())
    }
}

#[cfg(test)]
mod tests {
    mod delimeter {
        use super::super::*;

        #[test]
        fn closes() {
            assert!(')'.closes(&'('));
            assert!(']'.closes(&'['));
            assert!('}'.closes(&'{'));
            assert!('>'.closes(&'<'));

            assert!(!'>'.closes(&'a'));

            assert!(!'>'.closes(&'('));
            assert!(!'}'.closes(&'['));
            assert!(!']'.closes(&'{'));
            assert!(!')'.closes(&'<'));
        }

        #[test]
        fn points() {
            // corrupted
            assert_eq!(')'.points(), 3);
            assert_eq!(']'.points(), 57);
            assert_eq!('}'.points(), 1197);
            assert_eq!('>'.points(), 25137);

            // completion
            assert_eq!('('.points(), 1);
            assert_eq!('['.points(), 2);
            assert_eq!('{'.points(), 3);
            assert_eq!('<'.points(), 4);

            assert_eq!('a'.points(), 0);
        }
    }

    mod program {
        use aoc_helpers::util::{parse_input, test_input};

        use super::super::*;

        #[test]
        fn score_corrupted() {
            let input = test_input(
                "
                [({(<(())[]>[[{[]{<()<>>
                [(()[<>])]({[<{<<[]>>(
                {([(<{}[<>[]}>{[]{[(<()>
                (((({<>}<{<{<>}{[]{[]{}
                [[<[([]))<([[{}[[()]]]
                [{[{({}]{}}([{[{{{}}([]
                {<[[]]>}<{[{[{[]{()[[[]
                [<(<(<(<{}))><([]([]()
                <{([([[(<>()){}]>(<<{{
                <{([{{}}[<[[[<>{}]]]>[]]
                ",
            );

            let lines: Vec<Line> = parse_input(&input).expect("could not parse input");
            let program = Program::from(lines);
            assert_eq!(program.check().score_corruptions(), 26397);
        }

        #[test]
        fn score_completions() {
            let input = test_input(
                "
                [({(<(())[]>[[{[]{<()<>>
                [(()[<>])]({[<{<<[]>>(
                {([(<{}[<>[]}>{[]{[(<()>
                (((({<>}<{<{<>}{[]{[]{}
                [[<[([]))<([[{}[[()]]]
                [{[{({}]{}}([{[{{{}}([]
                {<[[]]>}<{[{[{[]{()[[[]
                [<(<(<(<{}))><([]([]()
                <{([([[(<>()){}]>(<<{{
                <{([{{}}[<[[[<>{}]]]>[]]
                ",
            );

            let lines: Vec<Line> = parse_input(&input).expect("could not parse input");
            let program = Program::from(lines);

            assert_eq!(program.check().score_completions(), 288957);
        }
    }
}

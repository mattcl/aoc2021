use std::{convert::TryFrom, iter::FromIterator, ops::Deref, str::FromStr};

use anyhow::{anyhow, bail, Result};
use aoc_helpers::Solver;
use rayon::prelude::*;
use rustc_hash::FxHashSet;

pub enum Digit {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl TryFrom<usize> for Digit {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self> {
        Ok(match value {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            _ => bail!("Digits can only be 0-9 but got: {}", value),
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Classification {
    One,
    Four,
    Seven,
    Eight,
    Unknown(usize),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Signal(pub FxHashSet<char>);

impl Signal {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_superset_of(&self, other: &Signal) -> bool {
        self.0.is_superset(other)
    }

    pub fn classify(&self) -> Classification {
        match self.len() {
            2 => Classification::One,
            3 => Classification::Seven,
            4 => Classification::Four,
            7 => Classification::Eight,
            x => Classification::Unknown(x),
        }
    }
}

impl Deref for Signal {
    type Target = FxHashSet<char>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Signal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let initial = s.len();
        if !(2..=7).contains(&initial) {
            bail!("invalid signal length: {}", s);
        }

        let hash = FxHashSet::from_iter(s.chars().filter(|ch| matches!(ch, 'a'..='g')));

        // we filtered some chars, so the signal must be invalid
        if hash.len() != initial {
            bail!("invalid character(s) in signal input: {}", s);
        }

        Ok(Signal(hash))
    }
}

#[derive(Debug, Clone)]
pub struct Solution<'a>(pub Vec<Option<&'a Signal>>);

impl<'a> Default for Solution<'a> {
    fn default() -> Self {
        Self(vec![None; 10])
    }
}

impl<'a> Solution<'a> {
    pub fn set(&mut self, digit: Digit, value: &'a Signal) {
        self.0[digit as usize] = Some(value);
    }

    pub fn get(&self, digit: Digit) -> Option<&'a Signal> {
        self.0[digit as usize]
    }

    pub fn get_digit(&self, signal: &Signal) -> Result<Digit> {
        for (i, sig) in self.0.iter().enumerate() {
            if let Some(s) = sig {
                if *s == signal {
                    return Digit::try_from(i);
                }
            }
        }
        bail!("could not determine digit for signal: {:?}", signal);
    }

    pub fn solved(&self) -> bool {
        self.0.iter().all(|s| s.is_some())
    }
}

impl<'a> Deref for Solution<'a> {
    type Target = Vec<Option<&'a Signal>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Observation {
    left: Vec<Signal>,
    right: Vec<Signal>,
}

impl Observation {
    pub fn rhs_count_known(&self) -> usize {
        self.right
            .iter()
            .filter(|s| !matches!(s.classify(), Classification::Unknown(_)))
            .count()
    }

    pub fn rhs_value(&self) -> Result<u64> {
        let solution = self.analyze()?;
        let mut v = 0;
        for s in self.right.iter() {
            let digit = solution.get_digit(s)?;
            v = v * 10 + digit as u64;
        }
        Ok(v)
    }

    pub fn analyze(&self) -> Result<Solution> {
        let mut fives: Vec<&Signal> = Vec::new();
        let mut sixes: Vec<&Signal> = Vec::new();

        let mut solution = Solution::default();

        // partition the data
        for s in self.left.iter() {
            match s.classify() {
                Classification::Unknown(x) if x == 5 || x == 6 => {
                    if x == 5 {
                        fives.push(s);
                    } else {
                        sixes.push(s);
                    }
                }
                Classification::Unknown(x) => bail!("Invalid signal len: {}! {:?}", x, s),
                Classification::One => solution.set(Digit::One, s),
                Classification::Four => solution.set(Digit::Four, s),
                Classification::Seven => solution.set(Digit::Seven, s),
                Classification::Eight => solution.set(Digit::Eight, s),
            }
        }

        // sanity check
        if sixes.len() != 3 || fives.len() != 3 {
            bail!(
                "incorrect number of sixes or fives values: 6 -> {:?}, 5 -> {:?}",
                sixes,
                fives
            );
        }

        let one = solution
            .get(Digit::One)
            .ok_or_else(|| anyhow!("attempted solution without One set"))?;
        let four = solution
            .get(Digit::Four)
            .ok_or_else(|| anyhow!("attempted solution without Four set"))?;

        for s in sixes.iter() {
            if !s.is_superset_of(one) {
                // we know this is the 6
                solution.set(Digit::Six, s);
            } else if s.is_superset_of(four) {
                // we know this the 9
                solution.set(Digit::Nine, s);
            } else {
                // this is the 0
                solution.set(Digit::Zero, s);
            }
        }

        let nine = solution
            .get(Digit::Nine)
            .ok_or_else(|| anyhow!("attempted solution without Nine set"))?;
        for s in fives.iter() {
            if s.is_superset_of(one) {
                // we know this is the 3
                solution.set(Digit::Three, s);
            } else if nine.is_superset_of(s) {
                // we know this is the 5
                solution.set(Digit::Five, s);
            } else {
                solution.set(Digit::Two, s);
            }
        }

        if !solution.solved() {
            bail!("Could not find a solution: {:?}", solution);
        }

        Ok(solution)
    }
}

impl FromStr for Observation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(" | ");
        let lhs = parts
            .next()
            .ok_or_else(|| anyhow!("input missing left hand side {}", s))?;
        let rhs = parts
            .next()
            .ok_or_else(|| anyhow!("input missing right hand side {}", s))?;

        let left = lhs
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<Signal>>>()?;
        let right = rhs
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<Signal>>>()?;

        Ok(Observation { left, right })
    }
}

#[derive(Debug, Clone)]
pub struct Matcher {
    observations: Vec<Observation>,
}

impl Matcher {
    pub fn new(observations: Vec<Observation>) -> Self {
        Self { observations }
    }

    pub fn rhs_count_known(&self) -> usize {
        self.observations.iter().map(|o| o.rhs_count_known()).sum()
    }

    pub fn rhs_values_sum(&self) -> Result<u64> {
        Ok(self
            .observations
            .iter()
            .map(|o| o.rhs_value())
            .collect::<Result<Vec<u64>>>()?
            .iter()
            .sum())
    }

    pub fn par_rhs_values_sum(&self) -> Result<u64> {
        Ok(self
            .observations
            .par_iter()
            .map(|o| o.rhs_value())
            .collect::<Result<Vec<u64>>>()?
            .iter()
            .sum())
    }
}

impl TryFrom<Vec<String>> for Matcher {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        Ok(Matcher::new(
            value
                .iter()
                .map(|s| Observation::from_str(s))
                .collect::<Result<Vec<Observation>>>()?,
        ))
    }
}

impl Solver for Matcher {
    const ID: &'static str = "seven segment search";
    const DAY: usize = 8;

    type P1 = usize;
    type P2 = u64;

    fn part_one(&mut self) -> Self::P1 {
        self.rhs_count_known()
    }

    fn part_two(&mut self) -> Self::P2 {
        self.rhs_values_sum().expect("unable to find solution")
    }
}

#[cfg(test)]
mod tests {
    mod signal {
        use super::super::*;

        use std::str::FromStr;

        #[test]
        fn from_str() {
            let s = Signal::from_str("fdgacbe").expect("could not make signal");
            assert_eq!(s.classify(), Classification::Eight);

            let s = Signal::from_str("ab").expect("could not make signal");
            assert_eq!(s.classify(), Classification::One);

            let s = Signal::from_str("abc").expect("could not make signal");
            assert_eq!(s.classify(), Classification::Seven);

            let s = Signal::from_str("abcd").expect("could not make signal");
            assert_eq!(s.classify(), Classification::Four);

            let s = Signal::from_str("abcde").expect("could not make signal");
            assert_eq!(s.classify(), Classification::Unknown(5));

            let s = Signal::from_str("abcdz");
            assert!(s.is_err());

            let s = Signal::from_str("abcdefga");
            assert!(s.is_err());
        }

        #[test]
        fn equality() {
            let s1 = Signal::from_str("fba").expect("could not make signal");
            let s2 = Signal::from_str("abf").expect("could not make signal");

            assert_eq!(s1, s2);
        }

        #[test]
        fn supersets() {
            let s1 = Signal::from_str("fdgacbe").expect("could not make signal");
            let s2 = Signal::from_str("bf").expect("could not make signal");

            assert!(s1.is_superset_of(&s2));
            assert!(!s2.is_superset_of(&s1));
            assert!(s1.is_superset_of(&s1));
        }
    }

    mod observation {
        use super::super::*;

        #[test]
        fn rhs_value() {
            let o = Observation::from_str("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf").expect("Could not make observation");
            assert_eq!(o.rhs_value().expect("could not solve"), 5353);
        }
    }

    mod solver {
        use aoc_helpers::util::test_input;

        use super::super::*;

        #[test]
        fn counting_unambiguious_digits() {
            let input = test_input("
                be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
                edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
                fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
                fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
                aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
                fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
                dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
                bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
                egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
                gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
                ");

            let solver = Matcher::try_from(input).expect("Could not parse input");

            assert_eq!(solver.rhs_count_known(), 26)
        }

        #[test]
        fn solving() {
            let input = test_input("
                be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
                edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
                fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
                fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
                aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
                fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
                dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
                bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
                egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
                gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
                ");

            let solver = Matcher::try_from(input).expect("Could not parse input");
            let res = solver.rhs_values_sum().expect("Could not solve");
            assert_eq!(res, 61229);

            let res = solver.par_rhs_values_sum().expect("Could not solve");
            assert_eq!(res, 61229);
        }
    }
}

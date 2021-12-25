use std::convert::TryFrom;

use anyhow::{bail, Result};
use aoc_helpers::Solver;

#[derive(Debug, Clone, Default)]
pub struct Diagnostic {
    num_bits: usize,
    values: Vec<u64>,
    gamma: u64,
    epsilon: u64,
}

impl Diagnostic {
    pub fn new(num_bits: usize, values: Vec<u64>) -> Self {
        let mut bits = vec![0_i64; num_bits];
        let base: u64 = 2;
        let masks: Vec<u64> = (0..bits.len()).rev().map(|i| base.pow(i as u32)).collect();

        for value in &values {
            for (i, mask) in masks.iter().enumerate() {
                if value & mask > 0 {
                    bits[i] += 1;
                } else {
                    bits[i] -= 1;
                }
            }
        }

        let mut gamma = 0;
        let mut epsilon = 0;

        for (i, bit) in bits.iter().enumerate() {
            if *bit >= 0 {
                gamma += masks[i];
            } else {
                epsilon += masks[i];
            }
        }

        Diagnostic {
            num_bits,
            values,
            gamma,
            epsilon,
        }
    }

    pub fn power_consumption(&self) -> u64 {
        self.gamma * self.epsilon
    }

    pub fn oxygen_generator_rating(&self) -> Result<u64> {
        self.filter_values((self.num_bits - 1) as u32, true)
    }

    pub fn co2_scrubber_rating(&self) -> Result<u64> {
        self.filter_values((self.num_bits - 1) as u32, false)
    }

    pub fn life_support_rating(&self) -> Result<u64> {
        Ok(self.oxygen_generator_rating()? * self.co2_scrubber_rating()?)
    }

    fn filter_values(&self, bit: u32, use_gamma: bool) -> Result<u64> {
        if self.values.is_empty() {
            bail!("Cannot filter an empty set");
        }

        let base: u64 = 2;
        let mask = base.pow(bit);

        let cmp = if use_gamma { self.gamma } else { self.epsilon };

        let new_set: Vec<u64> = self
            .values
            .iter()
            .cloned()
            .filter(|e| *e & mask == cmp & mask)
            .collect();

        if new_set.len() == 1 {
            return Ok(new_set[0]);
        }

        if bit == 0 {
            bail!("Could not filter to a unique value");
        }

        // This isn't so much recurse as "create a new Diagnositc with the
        // remaining values then just run filter_values on that"
        let tmp = Diagnostic::new(bit as usize, new_set);
        tmp.filter_values(bit - 1, use_gamma)
    }
}

impl TryFrom<&Vec<String>> for Diagnostic {
    type Error = anyhow::Error;

    fn try_from(value: &Vec<String>) -> anyhow::Result<Self> {
        if value.is_empty() {
            return Ok(Diagnostic::default());
        }

        let num_bits = value[0].len();

        if num_bits == 0 {
            bail!("Invalid diagnostic values: {:?}", value);
        }

        let mut parsed_values = Vec::new();
        for num in value {
            if num.len() != num_bits {
                bail!("Not all diagnositc values are the same length {:?}", &value);
            }

            let parsed = u64::from_str_radix(&num, 2)?;
            parsed_values.push(parsed);
        }

        Ok(Diagnostic::new(num_bits, parsed_values))
    }
}

#[derive(Debug, Clone, Default)]
pub struct DiagnosticWrapper {
    input: Vec<String>,
}

impl TryFrom<Vec<String>> for DiagnosticWrapper {
    type Error = anyhow::Error;

    fn try_from(input: Vec<String>) -> Result<Self> {
        Ok(Self {input})
    }
}

impl Solver for DiagnosticWrapper {
    const ID: &'static str = "binary diagnostic";
    const DAY: usize = 3;

    type P1 = u64;
    type P2 = u64;

    fn part_one(&mut self) -> Self::P1 {
        let d = Diagnostic::try_from(&self.input)
            .expect("could not parse input");
        d.power_consumption()
    }

    fn part_two(&mut self) -> Self::P2 {
        let d = Diagnostic::try_from(&self.input)
            .expect("could not parse input");
        d.life_support_rating()
            .expect("could not get life support rating")
    }

    // so the solve for this is a little different, because it's really
    // the construction of the diagnostic that does most of the work
    fn solve() -> aoc_helpers::Solution<Self::P1, Self::P2> {
        let instance = Self::instance();
        let d = Diagnostic::try_from(&instance.input)
            .expect("could not parse input");

        aoc_helpers::Solution::new(
            d.power_consumption(),
            d.life_support_rating()
                .expect("could not get life support rating")
        )
    }
}

#[cfg(test)]
mod tests {
    use aoc_helpers::util::test_input;

    use super::*;
    use std::convert::TryFrom;

    fn input() -> Vec<String> {
        test_input(
            "
            00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010
            ",
        )
    }

    #[test]
    fn try_from() {
        let input = input();
        let res = Diagnostic::try_from(&input);
        assert!(res.is_ok());

        let d = res.unwrap();

        assert_eq!(d.gamma, 22);
        assert_eq!(d.epsilon, 9);
        assert_eq!(d.power_consumption(), 198);
    }

    #[test]
    fn oxygen_generator_rating() {
        let input = input();
        let diagnostic = Diagnostic::try_from(&input).expect("invalid input");
        let res = diagnostic.oxygen_generator_rating();

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 23);
    }

    #[test]
    fn co2_scrubber_rating() {
        let input = input();
        let diagnostic = Diagnostic::try_from(&input).expect("invalid input");
        let res = diagnostic.co2_scrubber_rating();

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 10);
    }

    #[test]
    fn life_support_rating() {
        let input = input();
        let diagnostic = Diagnostic::try_from(&input).expect("invalid input");
        let res = diagnostic.life_support_rating();

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 230);
    }
}

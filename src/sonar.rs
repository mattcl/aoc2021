use std::{convert::TryFrom, num::ParseIntError};

use aoc_helpers::Solver;

#[derive(Debug, Clone)]
pub struct Report {
    pub depths: Vec<u64>,
}

impl Report {
    pub fn count_increases(&self) -> u64 {
        let mut count = 0;
        let mut prev = 0;
        for (idx, d) in self.depths.iter().enumerate() {
            if idx > 0 && prev < *d {
                count += 1;
            }
            prev = *d;
        }
        count
    }

    pub fn count_windowed_increases(&self) -> u64 {
        let mut count = 0;
        let mut window = 0;
        for (idx, d) in self.depths.iter().enumerate() {
            if idx > 2 {
                let new = window - self.depths[idx - 3] + d;
                if new > window {
                    count += 1;
                }
                window = new;
            } else {
                window += d;
            }
        }
        count
    }
}

impl TryFrom<Vec<String>> for Report {
    type Error = ParseIntError;

    fn try_from(value: Vec<String>) -> Result<Self, ParseIntError> {
        Ok(Report {
            depths: value
                .into_iter()
                .map(|v| v.parse())
                .collect::<Result<Vec<u64>, ParseIntError>>()?,
        })
    }
}

impl Solver for Report {
    const ID: &'static str = "sonar sweep";
    const DAY: usize = 1;

    type P1 = u64;
    type P2 = u64;

    fn part_one(&mut self) -> Self::P1 {
        self.count_increases()
    }

    fn part_two(&mut self) -> Self::P2 {
        self.count_windowed_increases()
    }
}

#[cfg(test)]
mod tests {
    use aoc_helpers::util;

    use super::*;
    use std::convert::TryInto;

    #[test]
    fn increase_counting() {
        let input = util::test_input(
            "
            199
            200
            208
            210
            200
            207
            240
            269
            260
            263
        ",
        );

        let report: Report = input.try_into().expect("could not convert to report");
        assert_eq!(report.count_increases(), 7);
    }

    #[test]
    fn windowed_increase_counting() {
        let input = util::test_input(
            "
            199
            200
            208
            210
            200
            207
            240
            269
            260
            263
        ",
        );

        let report: Report = input.try_into().expect("could not convert to report");
        assert_eq!(report.count_windowed_increases(), 5);
    }
}

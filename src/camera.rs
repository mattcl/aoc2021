use anyhow::{anyhow, bail, Result};
use rustc_hash::FxHashSet;
use std::{convert::TryFrom, fmt, str::FromStr};

use crate::generic::Location;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Instruction {
    X(usize),
    Y(usize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some(l) = s.split_whitespace().last() {
            let mut parts = l.split('=');
            let axis = parts
                .next()
                .ok_or_else(|| anyhow!("invalid instruction: missing axis {}", s))?;
            let val: usize = parts
                .next()
                .ok_or_else(|| anyhow!("invalid instruction: missing axis {}", s))?
                .parse()?;

            Ok(match axis {
                "x" => Instruction::X(val),
                "y" => Instruction::Y(val),
                _ => bail!("Unknown axis: {}", s),
            })
        } else {
            bail!("Invalid instruction: {}", s);
        }
    }
}

pub trait Reflect {
    fn reflect(&self, instruction: &Instruction) -> Self;
}

impl Reflect for Location {
    fn reflect(&self, instruction: &Instruction) -> Self {
        // So, Location, as implemented, is row, col. But, this problem is
        // specifying x, y. Rather than flip them, just treat row as x and
        // col as y, which is confusing, but I'm lazy.
        match instruction {
            Instruction::X(m) if self.row > *m => {
                Location::new(self.row - 2 * (self.row - m), self.col)
            }
            Instruction::Y(m) if self.col > *m => {
                Location::new(self.row, self.col - 2 * (self.col - m))
            }
            _ => *self,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Page {
    dots: FxHashSet<Location>,
}

impl Page {
    pub fn fold(&self, instruction: &Instruction) -> Self {
        self.dots
            .iter()
            .map(|d| d.reflect(instruction))
            .collect::<FxHashSet<Location>>()
            .into()
    }

    pub fn count_visible(&self) -> usize {
        self.dots.len()
    }
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut max_x = 0;
        let mut max_y = 0;

        for d in &self.dots {
            if d.row > max_x {
                max_x = d.row;
            }

            if d.col > max_y {
                max_y = d.col;
            }
        }

        let mut grid = vec![vec![' '; max_x + 1]; max_y + 1];

        for d in &self.dots {
            grid[d.col][d.row] = '#';
        }

        let out = grid
            .iter()
            .map(|r| r.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "\n{}", out)
    }
}

impl From<FxHashSet<Location>> for Page {
    fn from(value: FxHashSet<Location>) -> Self {
        Self { dots: value }
    }
}

impl TryFrom<&[String]> for Page {
    type Error = anyhow::Error;

    fn try_from(value: &[String]) -> Result<Self> {
        let dots = value
            .iter()
            .map(|l| Location::from_str(l))
            .collect::<Result<FxHashSet<Location>>>()?;
        Ok(dots.into())
    }
}

#[derive(Debug, Clone)]
pub struct Manual {
    page: Page,
    instructions: Vec<Instruction>,
}

impl Manual {
    pub fn new(page: Page, instructions: Vec<Instruction>) -> Self {
        Self { page, instructions }
    }

    pub fn first_instruction(&self) -> Page {
        self.instructions
            .get(0)
            .map(|i| self.page.fold(i))
            .unwrap_or_else(|| self.page.clone())
    }

    pub fn folded(&self) -> Page {
        self.instructions
            .iter()
            .fold(self.page.clone(), |acc, inst| acc.fold(inst))
    }
}

impl TryFrom<Vec<String>> for Manual {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut parts = value.split(|l| l.is_empty());

        let dots = parts
            .next()
            .ok_or_else(|| anyhow!("input is missing dots"))?;
        let instructions = parts
            .next()
            .ok_or_else(|| anyhow!("input is missing instructions"))?;

        Ok(Self::new(
            Page::try_from(dots)?,
            instructions
                .iter()
                .map(|i| Instruction::from_str(i))
                .collect::<Result<Vec<Instruction>>>()?,
        ))
    }
}

#[cfg(test)]
mod tests {

    mod manual {
        use crate::util::test_input;

        use super::super::*;

        #[test]
        fn first_fold() {
            let input = test_input(
                "
                6,10
                0,14
                9,10
                0,3
                10,4
                4,11
                6,0
                6,12
                4,1
                0,13
                10,12
                3,4
                3,0
                8,4
                1,10
                2,14
                8,10
                9,0

                fold along y=7
                fold along x=5
                ",
            );
            let manual = Manual::try_from(input).expect("could not parse input");
            let p = manual.first_instruction();
            assert_eq!(p.count_visible(), 17);
        }

        #[test]
        fn folded() {
            let input = test_input(
                "
                6,10
                0,14
                9,10
                0,3
                10,4
                4,11
                6,0
                6,12
                4,1
                0,13
                10,12
                3,4
                3,0
                8,4
                1,10
                2,14
                8,10
                9,0

                fold along y=7
                fold along x=5
                ",
            );
            let manual = Manual::try_from(input).expect("could not parse input");
            let p = manual.folded();
            // This is a little different than what was provided, but, since I
            // don't use a grid until the very end, mine will be truncated
            let expected = "
#####
#   #
#   #
#   #
#####";

            println!("{}", p.to_string());
            assert_eq!(p.to_string(), expected);
        }
    }
}

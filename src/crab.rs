use std::{convert::TryFrom, hash::Hash, num::ParseIntError, str::FromStr};

use anyhow::{anyhow, Result};
use aoc_helpers::Solver;
use itertools::{Itertools, MinMaxResult};
use rayon::prelude::*;

pub trait Moveable: FromStr + Eq + PartialEq + Hash + Ord + PartialOrd + Send + Sync {
    fn location(&self) -> i64;
    fn cost_to_move(&self, target: i64) -> i64;
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct LinearSub(pub i64);

impl Moveable for LinearSub {
    fn location(&self) -> i64 {
        self.0
    }

    fn cost_to_move(&self, target: i64) -> i64 {
        (self.0 - target).abs()
    }
}

impl FromStr for LinearSub {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct ArithmeticSub(pub i64);

impl Moveable for ArithmeticSub {
    fn location(&self) -> i64 {
        self.0
    }

    fn cost_to_move(&self, target: i64) -> i64 {
        let dist = (self.0 - target).abs();
        (dist + 1) * dist / 2
    }
}

impl FromStr for ArithmeticSub {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

#[derive(Debug, Clone)]
pub struct Swarm<T>
where
    T: Moveable,
{
    submarines: Vec<T>,
}

impl<T> Swarm<T>
where
    T: Moveable,
{
    pub fn new(submarines: Vec<T>) -> Self {
        Self { submarines }
    }

    pub fn cheapest_expenditure(&self) -> i64 {
        let (min, max) = match self.submarines.iter().minmax() {
            MinMaxResult::NoElements => return -1,
            MinMaxResult::OneElement(only) => (only.location(), only.location()),
            MinMaxResult::MinMax(min, max) => (min.location(), max.location()),
        };

        (min..=max)
            .into_par_iter()
            .map(|t| {
                self.submarines
                    .iter()
                    .fold(0, |acc, s| acc + s.cost_to_move(t))
            })
            .min()
            .unwrap_or(-1)
    }
}

impl<T> FromStr for Swarm<T>
where
    T: Moveable,
{
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            submarines: s
                .split(',')
                .map(T::from_str)
                .collect::<Result<Vec<T>, <T as FromStr>::Err>>()?,
        })
    }
}

pub struct Crabs {
    linear: Swarm<LinearSub>,
    arithmetic: Swarm<ArithmeticSub>,
}

impl TryFrom<Vec<String>> for Crabs {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let line = value.first().ok_or_else(|| anyhow!("input empty"))?;
        let linear = Swarm::from_str(line)?;
        let arithmetic = Swarm::from_str(line)?;

        Ok(Self { linear, arithmetic })
    }
}

impl Solver for Crabs {
    const ID: &'static str = "the treachery of whales";
    const DAY: usize = 7;

    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Self::P1 {
        self.linear.cheapest_expenditure()
    }

    fn part_two(&mut self) -> Self::P2 {
        self.arithmetic.cheapest_expenditure()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    #[test]
    fn cheapest_expenditure() {
        let swarm: Swarm<LinearSub> =
            Swarm::from_str("16,1,2,0,4,2,7,1,2,14").expect("Could not create swarm");
        assert_eq!(swarm.cheapest_expenditure(), 37);
    }

    #[test]
    fn arithmetic_expenditure() {
        let swarm: Swarm<ArithmeticSub> =
            Swarm::from_str("16,1,2,0,4,2,7,1,2,14").expect("Could not create swarm");
        assert_eq!(swarm.cheapest_expenditure(), 168);
    }
}

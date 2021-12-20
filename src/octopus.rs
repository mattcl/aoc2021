use std::convert::{TryFrom, TryInto};

use anyhow::{anyhow, Result};
use rustc_hash::FxHashSet;

use aoc_helpers::generic::{prelude::*, Grid, Location};

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq)]
pub struct Octopus(pub i64);

impl Octopus {
    pub fn new(v: i64) -> Self {
        Self(v)
    }

    pub fn reset(&mut self) {
        self.0 = 0;
    }

    /// Increase the current energy level by one and return `true` if the
    /// octopus would now flash
    pub fn charge(&mut self) -> bool {
        self.0 += 1;
        self.0 > 9
    }
}

impl From<i64> for Octopus {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Default)]
pub struct OctopusGrid {
    octopuses: Grid<Octopus>,
    syncd_genrations: Vec<usize>,
    generations: usize,
}

impl OctopusGrid {
    /// Charge the octopus specified by `loc` and return `true` if it flashes
    pub fn charge(&mut self, loc: &Location) -> bool {
        self.octopuses
            .get_mut(loc)
            .map(|oct| oct.charge())
            .unwrap_or(false)
    }

    /// Reset the octopus specified by `loc`
    pub fn reset(&mut self, loc: &Location) {
        if let Some(oct) = self.octopuses.get_mut(loc) {
            oct.reset();
        }
    }

    /// Simulate the grid of octopi for `genrations` generations and return the
    /// total number of flashes in that time
    pub fn simulate(&mut self, generations: usize) -> usize {
        (0..generations).map(|_| self.step()).sum()
    }

    /// Simulate the grid of octopi until the first generation where they all
    /// flash at the same time. Return that generation.
    ///
    /// This can be safely called multiple times or after any number of calls
    /// to `simulate(...)`, and will always return the true first generation
    /// that the sync happened.
    pub fn simulate_until_sync(&mut self) -> usize {
        if let Some(gen) = self.syncd_genrations.first() {
            return *gen;
        }

        loop {
            if self.octopuses.size() == self.step() {
                break self.generations;
            }
        }
    }

    /// Perform one step of the simulation, returning the number of octopi that
    /// flashed during the step
    pub fn step(&mut self) -> usize {
        self.generations += 1;
        // 1. increase every octopus by 1, storing the locations of flashes
        let mut flashes: FxHashSet<Location> = FxHashSet::default();
        for row in 0..self.octopuses.rows() {
            for col in 0..self.octopuses.cols() {
                let loc = (row, col).into();
                if self.charge(&loc) {
                    self.reset(&loc);
                    flashes.insert(loc);
                }
            }
        }
        // 2. rerusively propagate flash
        self.recur(&flashes.clone(), &mut flashes);

        // 3. since we reset during the charge check, and, since the cache
        // prevents us from ever modifying an octopus that's already flashed
        // this step, there's no need to zero the octopuses that flashed at
        // this point

        let count = flashes.len();
        if count == self.octopuses.size() {
            self.syncd_genrations.push(self.generations);
        }

        count
    }

    fn recur(
        &mut self,
        flash_locations: &FxHashSet<Location>,
        already_flashed: &mut FxHashSet<Location>,
    ) {
        if flash_locations.is_empty() {
            return;
        }
        // for every location that flashed, modify all neighboring locations by
        // one, storing any "new" flashes
        let mut flashes: FxHashSet<Location> = FxHashSet::default();
        for loc in flash_locations.iter() {
            for neighbor in loc.neighbors() {
                if already_flashed.contains(&neighbor) {
                    continue;
                }

                // charge the neighbor and, if it flashes, add it to the new
                // list of flashes and the already_flashed cache
                if self.charge(&neighbor) {
                    self.reset(&neighbor);
                    flashes.insert(neighbor);
                    already_flashed.insert(neighbor);
                }
            }
        }

        self.recur(&flashes, already_flashed);
    }
}

impl TryFrom<Vec<String>> for OctopusGrid {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let octopuses = value
            .iter()
            .map(|s| {
                s.chars()
                    .map(|ch| {
                        ch.to_digit(10)
                            .map(|d| Octopus::new(d as i64))
                            .ok_or_else(|| anyhow!("Invalid characters"))
                    })
                    .collect::<Result<Vec<Octopus>>>()
            })
            .collect::<Result<Vec<Vec<Octopus>>>>()?;

        Ok(Self {
            octopuses: octopuses.try_into()?,
            syncd_genrations: Vec::new(),
            generations: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    mod octopus {
        use super::super::*;

        #[test]
        fn reset() {
            let mut o = Octopus::new(10);
            o.reset();
            assert_eq!(o, Octopus::default())
        }
    }

    mod grid {
        use aoc_helpers::util::test_input;

        use super::super::*;

        #[test]
        fn flashes_after_hundred_steps() {
            let input = test_input(
                "
                5483143223
                2745854711
                5264556173
                6141336146
                6357385478
                4167524645
                2176841721
                6882881134
                4846848554
                5283751526
                ",
            );
            let mut grid = OctopusGrid::try_from(input).expect("could not construt grid");
            assert_eq!(grid.simulate(100), 1656);
        }

        #[test]
        fn simulate_until_sync() {
            let input = test_input(
                "
                5483143223
                2745854711
                5264556173
                6141336146
                6357385478
                4167524645
                2176841721
                6882881134
                4846848554
                5283751526
                ",
            );
            let mut grid = OctopusGrid::try_from(input).expect("could not construt grid");
            assert_eq!(grid.simulate_until_sync(), 195);
        }
    }
}

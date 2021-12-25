use std::convert::TryFrom;

use anyhow::{bail, Result};
use aoc_helpers::{
    generic::{prelude::*, Grid, Location},
    Solver,
};
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Spot {
    East,
    South,
    Empty,
}

impl TryFrom<char> for Spot {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        Ok(match value {
            '>' => Self::East,
            'v' => Self::South,
            '.' => Self::Empty,
            _ => bail!("cannot make Spot from: {}", value),
        })
    }
}

impl Default for Spot {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug, Clone, Default)]
pub struct CucumberGrid {
    grid: Grid<Spot>,
    east_locations: FxHashSet<Location>,
    south_locations: FxHashSet<Location>,
}

impl CucumberGrid {
    pub fn stabilize(&mut self) -> usize {
        let mut count = 0;

        loop {
            count += 1;
            if !self.step() {
                break;
            }
        }

        count
    }

    pub fn step(&mut self) -> bool {
        // we don't want to short-circuit
        let east = self.move_east();
        let south = self.move_south();
        east || south
    }

    pub fn move_east(&mut self) -> bool {
        let mut east_moves = Vec::new();

        for loc in self.east_locations.iter() {
            if let Some(east) = loc.east() {
                let dest = match self.grid.get(&east) {
                    Some(_) => east,
                    None => Location::new(loc.row, 0),
                };

                if self.grid.get(&dest).unwrap_or(&Spot::Empty) == &Spot::Empty {
                    // this is valid move, so record it
                    east_moves.push((*loc, dest))
                }
            }
        }

        // for row in 0..self.grid.rows() {
        //     for col in 0..self.grid.cols() {
        //         let loc: Location = (row, col).into();
        //         let s = self.grid.locations[row][col];
        //         if s == Spot::East {
        //             if let Some(east) = loc.east() {
        //                 let dest = match self.grid.get(&east) {
        //                     Some(_) => east,
        //                     None => Location::new(row, 0)
        //                 };

        //                 if self.grid.get(&dest).unwrap_or(&Spot::Empty) == &Spot::Empty {
        //                     // this is valid move, so record it
        //                     east_moves.push((loc, dest))
        //                 }
        //             }
        //         }
        //     }
        // }

        if east_moves.is_empty() {
            return false;
        }

        // apply east moves
        for (origin, dest) in east_moves.iter() {
            self.grid.locations[origin.row][origin.col] = Spot::Empty;
            self.grid.locations[dest.row][dest.col] = Spot::East;
            self.east_locations.remove(origin);
            self.east_locations.insert(*dest);
        }

        true
    }

    pub fn move_south(&mut self) -> bool {
        let mut south_moves = Vec::new();

        for loc in self.south_locations.iter() {
            if let Some(south) = loc.south() {
                let dest = match self.grid.get(&south) {
                    Some(_) => south,
                    None => Location::new(0, loc.col),
                };

                if self.grid.get(&dest).unwrap_or(&Spot::Empty) == &Spot::Empty {
                    // this is valid move, so record it
                    south_moves.push((*loc, dest))
                }
            }
        }

        // for row in 0..self.grid.rows() {
        //     for col in 0..self.grid.cols() {
        //         let loc: Location = (row, col).into();
        //         let s = self.grid.locations[row][col];
        //         if s == Spot::south {
        //             if let Some(south) = loc.south() {
        //                 let dest = match self.grid.get(&south) {
        //                     Some(_) => south,
        //                     None => Location::new(row, 0)
        //                 };

        //                 if self.grid.get(&dest).unwrap_or(&Spot::Empty) == &Spot::Empty {
        //                     // this is valid move, so record it
        //                     south_moves.push((loc, dest))
        //                 }
        //             }
        //         }
        //     }
        // }

        if south_moves.is_empty() {
            return false;
        }

        // apply south moves
        for (origin, dest) in south_moves.iter() {
            self.grid.locations[origin.row][origin.col] = Spot::Empty;
            self.grid.locations[dest.row][dest.col] = Spot::South;
            self.south_locations.remove(origin);
            self.south_locations.insert(*dest);
        }

        true
    }
}

impl TryFrom<Vec<String>> for CucumberGrid {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let spots = value
            .iter()
            .map(|s| {
                s.chars()
                    .map(|ch| Spot::try_from(ch))
                    .collect::<Result<Vec<Spot>>>()
            })
            .collect::<Result<Vec<Vec<Spot>>>>()?;
        let grid = Grid::new(spots);

        let mut east_locations = FxHashSet::default();
        let mut south_locations = FxHashSet::default();

        for row in 0..grid.rows() {
            for col in 0..grid.cols() {
                let loc = Location::new(row, col);
                match grid.get(&loc) {
                    Some(Spot::East) => east_locations.insert(loc),
                    Some(Spot::South) => south_locations.insert(loc),
                    _ => false,
                };
            }
        }

        Ok(Self {
            grid,
            east_locations,
            south_locations,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Cucumber {
    grid: CucumberGrid,
}

impl TryFrom<Vec<String>> for Cucumber {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        Ok(Self {
            grid: CucumberGrid::try_from(value)?,
        })
    }
}

impl Solver for Cucumber {
    const ID: &'static str = "sea cucumber";
    const DAY: usize = 25;

    type P1 = usize;
    type P2 = String;

    fn part_one(&mut self) -> Self::P1 {
        let mut g = self.grid.clone();
        g.stabilize()
    }

    fn part_two(&mut self) -> Self::P2 {
        String::from("No part 2 for day 25")
    }
}

#[cfg(test)]
mod tests {
    use aoc_helpers::util::test_input;

    use super::*;

    #[test]
    fn stabilizing() {
        let input = test_input(
            "
            v...>>.vv>
            .vv>>.vv..
            >>.>v>...v
            >>v>>.>.v.
            v>v.vv.v..
            >.>>..v...
            .vv..>.>v.
            v.v..>>v.v
            ....v..v.>
            ",
        );

        let mut grid = CucumberGrid::try_from(input).expect("could not parse input");
        assert_eq!(grid.stabilize(), 58);
    }
}

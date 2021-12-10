use std::convert::TryFrom;

use anyhow::{anyhow, bail, Result};
use rayon::prelude::*;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq)]
pub struct Loc {
    row: usize,
    col: usize,
}

impl Loc {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn north(&self) -> Option<Loc> {
        if self.row == 0 {
            return None;
        }

        Some((self.row - 1, self.col).into())
    }

    pub fn south(&self) -> Option<Loc> {
        Some((self.row + 1, self.col).into())
    }

    pub fn west(&self) -> Option<Loc> {
        if self.col == 0 {
            return None;
        }

        Some((self.row, self.col - 1).into())
    }

    pub fn east(&self) -> Option<Loc> {
        Some((self.row, self.col + 1).into())
    }
}

impl From<(usize, usize)> for Loc {
    fn from(v: (usize, usize)) -> Self {
        Self { row: v.0, col: v.1 }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Basin {
    loc: Loc,
    size: usize,
}

impl Basin {
    pub fn new(loc: Loc) -> Self {
        Self { loc, size: 0 }
    }
}

#[derive(Debug, Clone, Default)]
pub struct HeightMap {
    locations: Vec<Vec<i64>>,
}

impl HeightMap {
    pub fn total_risk(&self) -> i64 {
        self.lowpoints()
            .iter()
            .fold(0, |acc, loc| acc + self.risk(*loc).unwrap_or(0))
    }

    pub fn largest_basins(&self) -> Result<usize> {
        let mut basins = self.basins();
        basins.sort_by(|a, b| b.size.cmp(&a.size));

        if basins.len() < 3 {
            bail!("not enough basins to satisfy problem");
        }

        Ok(basins[0].size * basins[1].size * basins[2].size)
    }

    pub fn lowpoints(&self) -> Vec<Loc> {
        let mut points = Vec::new();
        for row in 0..self.locations.len() {
            for col in 0..self.locations[row].len() {
                let loc: Loc = (row, col).into();
                // direct lookup this, since we know it exists
                let value = self.locations[row][col];
                if loc
                    .north()
                    .and_then(|l| self.get(l))
                    .map(|other| other > value)
                    .unwrap_or(true)
                    && loc
                        .south()
                        .and_then(|l| self.get(l))
                        .map(|other| other > value)
                        .unwrap_or(true)
                    && loc
                        .east()
                        .and_then(|l| self.get(l))
                        .map(|other| other > value)
                        .unwrap_or(true)
                    && loc
                        .west()
                        .and_then(|l| self.get(l))
                        .map(|other| other > value)
                        .unwrap_or(true)
                {
                    points.push(loc);
                }
            }
        }

        points
    }

    pub fn basins(&self) -> Vec<Basin> {
        let mut basins: Vec<Basin> = self.lowpoints().into_iter().map(Basin::new).collect();
        basins.par_iter_mut().for_each(|b| self.determine_size(b));
        basins
    }

    pub fn determine_size(&self, basin: &mut Basin) {
        let mut checked: FxHashSet<Loc> = FxHashSet::default();
        self.recur(basin.loc, basin, &mut checked);
    }

    pub fn recur(&self, cur: Loc, basin: &mut Basin, checked: &mut FxHashSet<Loc>) {
        checked.insert(cur);
        if self.get(cur) == Some(9) {
            return;
        }

        basin.size += 1;

        if let Some(north) = cur.north().and_then(|l| self.get(l).map(|_| l)) {
            if !checked.contains(&north) {
                self.recur(north, basin, checked);
            }
        }

        if let Some(south) = cur.south().and_then(|l| self.get(l).map(|_| l)) {
            if !checked.contains(&south) {
                self.recur(south, basin, checked);
            }
        }

        if let Some(east) = cur.east().and_then(|l| self.get(l).map(|_| l)) {
            if !checked.contains(&east) {
                self.recur(east, basin, checked);
            }
        }

        if let Some(west) = cur.west().and_then(|l| self.get(l).map(|_| l)) {
            if !checked.contains(&west) {
                self.recur(west, basin, checked);
            }
        }
    }

    pub fn get(&self, loc: Loc) -> Option<i64> {
        self.locations
            .get(loc.row)
            .and_then(|r| r.get(loc.col).copied())
    }

    pub fn risk(&self, loc: Loc) -> Option<i64> {
        self.get(loc).map(|v| v + 1)
    }
}

impl TryFrom<Vec<String>> for HeightMap {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let locations = value
            .iter()
            .map(|s| {
                s.chars()
                    .map(|ch| {
                        ch.to_digit(10)
                            .map(|d| d as i64)
                            .ok_or_else(|| anyhow!("Invalid characters"))
                    })
                    .collect::<Result<Vec<i64>>>()
            })
            .collect::<Result<Vec<Vec<i64>>>>()?;

        Ok(HeightMap { locations })
    }
}

#[cfg(test)]
mod tests {
    mod loc {
        use super::super::*;

        #[test]
        fn north() {
            let l = Loc::new(2, 2);
            assert_eq!(l.north(), Some(Loc::new(1, 2)));

            let l = Loc::new(0, 2);
            assert_eq!(l.north(), None);
        }

        #[test]
        fn south() {
            let l = Loc::new(2, 2);
            assert_eq!(l.south(), Some(Loc::new(3, 2)));
        }

        #[test]
        fn east() {
            let l = Loc::new(2, 2);
            assert_eq!(l.east(), Some(Loc::new(2, 3)));
        }

        #[test]
        fn west() {
            let l = Loc::new(2, 2);
            assert_eq!(l.west(), Some(Loc::new(2, 1)));

            let l = Loc::new(2, 0);
            assert_eq!(l.west(), None);
        }
    }

    mod heightmap {
        use crate::util::test_input;

        use super::super::*;

        #[test]
        fn total_risk() {
            let input = test_input(
                "
                2199943210
                3987894921
                9856789892
                8767896789
                9899965678
                ",
            );

            let h = HeightMap::try_from(input).expect("could not make heightmap");
            assert_eq!(h.total_risk(), 15);
        }

        #[test]
        fn largest_basins() {
            let input = test_input(
                "
                2199943210
                3987894921
                9856789892
                8767896789
                9899965678
                ",
            );

            let h = HeightMap::try_from(input).expect("could not make heightmap");
            assert_eq!(
                h.largest_basins().expect("could not find largest basins"),
                1134
            );
        }
    }
}
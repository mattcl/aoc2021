use std::str::FromStr;

use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq)]
pub struct Location {
    pub row: usize,
    pub col: usize,
}

impl Location {
    const ORTH_LOCS: [(i64, i64); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Self> {
        let current_row = self.row as i64;
        let current_col = self.col as i64;
        (-1..=1)
            .cartesian_product(-1..=1)
            .into_iter()
            .filter_map(move |(r, c)| {
                if (r == -1 && current_row == 0)
                    || (c == -1 && current_col == 0)
                    || (r == 0 && c == 0)
                {
                    None
                } else {
                    Some(Self::from((
                        (current_row + r) as usize,
                        (current_col + c) as usize,
                    )))
                }
            })
    }

    pub fn orthogonal_neighbors(&self) -> impl Iterator<Item = Self> {
        let current_row = self.row as i64;
        let current_col = self.col as i64;
        Self::ORTH_LOCS.iter().filter_map(move |(r, c)| {
            if (*r == -1 && current_row == 0) || (*c == -1 && current_col == 0) {
                None
            } else {
                Some(Self::from((
                    (current_row + r) as usize,
                    (current_col + c) as usize,
                )))
            }
        })
    }

    pub fn north(&self) -> Option<Location> {
        if self.row == 0 {
            return None;
        }

        Some((self.row - 1, self.col).into())
    }

    pub fn south(&self) -> Option<Location> {
        Some((self.row + 1, self.col).into())
    }

    pub fn west(&self) -> Option<Location> {
        if self.col == 0 {
            return None;
        }

        Some((self.row, self.col - 1).into())
    }

    pub fn east(&self) -> Option<Location> {
        Some((self.row, self.col + 1).into())
    }
}

impl From<(usize, usize)> for Location {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl FromStr for Location {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(',');
        let row: usize = parts
            .next()
            .ok_or_else(|| anyhow!("missing row: {}", s))?
            .parse()?;
        let col: usize = parts
            .next()
            .ok_or_else(|| anyhow!("missing col: {}", s))?
            .parse()?;
        Ok(Self::new(row, col))
    }
}

#[cfg(test)]
mod tests {
    mod location {
        use std::collections::HashSet;
        use std::iter::FromIterator;

        use super::super::*;

        #[test]
        fn neighbors() {
            let l = Location::from((1, 1));
            let neighbors: HashSet<Location> = l.neighbors().collect();
            let expected: HashSet<Location> = HashSet::from_iter(
                vec![
                    Location::new(0, 0),
                    Location::new(0, 1),
                    Location::new(0, 2),
                    Location::new(1, 0),
                    Location::new(1, 2),
                    Location::new(2, 0),
                    Location::new(2, 1),
                    Location::new(2, 2),
                ]
                .into_iter(),
            );
            assert_eq!(neighbors.len(), 8);
            assert_eq!(neighbors, expected);

            let l = Location::from((0, 0));
            let neighbors: HashSet<Location> = l.neighbors().collect();
            let expected: HashSet<Location> = HashSet::from_iter(
                vec![
                    Location::new(0, 1),
                    Location::new(1, 1),
                    Location::new(1, 0),
                ]
                .into_iter(),
            );
            assert_eq!(neighbors.len(), 3);
            assert_eq!(neighbors, expected);

            let l = Location::from((0, 1));
            let neighbors: HashSet<Location> = l.neighbors().collect();
            let expected: HashSet<Location> = HashSet::from_iter(
                vec![
                    Location::new(0, 0),
                    Location::new(0, 2),
                    Location::new(1, 0),
                    Location::new(1, 1),
                    Location::new(1, 2),
                ]
                .into_iter(),
            );
            assert_eq!(neighbors.len(), 5);
            assert_eq!(neighbors, expected);
        }

        #[test]
        fn orthogonal_neighbors() {
            let l = Location::from((1, 1));
            let neighbors: HashSet<Location> = l.orthogonal_neighbors().collect();
            let expected: HashSet<Location> = HashSet::from_iter(
                vec![
                    Location::new(0, 1),
                    Location::new(1, 0),
                    Location::new(1, 2),
                    Location::new(2, 1),
                ]
                .into_iter(),
            );
            assert_eq!(neighbors.len(), 4);
            assert_eq!(neighbors, expected);

            let l = Location::from((0, 0));
            let neighbors: HashSet<Location> = l.orthogonal_neighbors().collect();
            let expected: HashSet<Location> =
                HashSet::from_iter(vec![Location::new(0, 1), Location::new(1, 0)].into_iter());
            assert_eq!(neighbors.len(), 2);
            assert_eq!(neighbors, expected);

            let l = Location::from((0, 1));
            let neighbors: HashSet<Location> = l.orthogonal_neighbors().collect();
            let expected: HashSet<Location> = HashSet::from_iter(
                vec![
                    Location::new(0, 0),
                    Location::new(0, 2),
                    Location::new(1, 1),
                ]
                .into_iter(),
            );
            assert_eq!(neighbors.len(), 3);
            assert_eq!(neighbors, expected);
        }

        #[test]
        fn north() {
            let l = Location::new(2, 2);
            assert_eq!(l.north(), Some(Location::new(1, 2)));

            let l = Location::new(0, 2);
            assert_eq!(l.north(), None);
        }

        #[test]
        fn south() {
            let l = Location::new(2, 2);
            assert_eq!(l.south(), Some(Location::new(3, 2)));
        }

        #[test]
        fn east() {
            let l = Location::new(2, 2);
            assert_eq!(l.east(), Some(Location::new(2, 3)));
        }

        #[test]
        fn west() {
            let l = Location::new(2, 2);
            assert_eq!(l.west(), Some(Location::new(2, 1)));

            let l = Location::new(2, 0);
            assert_eq!(l.west(), None);
        }
    }
}

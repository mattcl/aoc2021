use itertools::Itertools;

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq)]
pub struct Location {
    pub row: usize,
    pub col: usize,
}

impl Location {
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
}

impl From<(usize, usize)> for Location {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
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
    }
}

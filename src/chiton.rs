use std::{collections::BinaryHeap, convert::TryFrom};

use anyhow::{anyhow, Result};

use crate::generic::Location;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Chiton(pub usize);

impl Chiton {
    pub fn new(val: usize) -> Self {
        Self(val)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
    idx: usize,
    cost: usize,
}

impl Node {
    pub fn new(idx: usize, cost: usize) -> Self {
        Self { idx, cost }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.idx.cmp(&other.idx))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Grid {
    locations: Vec<Vec<Chiton>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    pub fn bottom_right(&self) -> Location {
        Location::new(self.rows - 1, self.cols - 1)
    }

    pub fn scaled_bottom_right(&self, scale: usize) -> Location {
        Location::new(self.rows * scale - 1, self.cols * scale - 1)
    }

    pub fn get(&self, location: &Location) -> Option<&Chiton> {
        self.locations
            .get(location.row)
            .and_then(|r| r.get(location.col))
    }

    pub fn get_scaled(&self, location: &Location, scale: usize) -> Option<Chiton> {
        // we're out of bounds here
        let r_fac = location.row / self.rows;
        let c_fac = location.col / self.cols;
        if r_fac >= scale || c_fac >= scale {
            return None;
        }

        let row = location.row % self.rows;
        let col = location.col % self.cols;
        self.locations
            .get(row)
            .and_then(|r| r.get(col))
            .copied()
            .map(|chiton| {
                let mut v = chiton.0 + r_fac + c_fac;
                if v > 9 {
                    v = v % 10 + 1;
                }
                Chiton(v)
            })
    }

    pub fn shortest(&self, scale: usize, start: &Location, end: &Location) -> Option<usize> {
        let size = (self.rows * scale) * (self.cols * scale);
        let largest = size * 9;
        let mut lowest = vec![largest; size];

        let s_idx = start.as_rm_index(self.rows * scale);
        let e_idx = end.as_rm_index(self.rows * scale);

        let mut heap = BinaryHeap::new();
        heap.push(Node::new(s_idx, 0));
        lowest[s_idx] = 0;

        while let Some(cur) = heap.pop() {
            if cur.idx == e_idx {
                return Some(cur.cost);
            }

            if cur.cost > lowest[cur.idx] {
                continue;
            }

            let loc = Location::from_rm_index(cur.idx, self.rows * scale);

            for n in loc.orthogonal_neighbors() {
                if let Some(chiton) = self.get_scaled(&n, scale) {
                    let n_idx = n.as_rm_index(self.rows * scale);

                    let next = Node::new(n_idx, cur.cost + chiton.0);

                    if next.cost < lowest[next.idx] {
                        lowest[next.idx] = next.cost;
                        heap.push(next);
                    }
                }
            }
        }

        None
    }
}

impl TryFrom<Vec<String>> for Grid {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let locations = value
            .iter()
            .map(|s| {
                s.chars()
                    .map(|ch| {
                        ch.to_digit(10)
                            .map(|d| Chiton::new(d as usize))
                            .ok_or_else(|| anyhow!("Invalid characters"))
                    })
                    .collect::<Result<Vec<Chiton>>>()
            })
            .collect::<Result<Vec<Vec<Chiton>>>>()?;

        let rows = locations.len();
        let cols = locations.get(0).map(|r| r.len()).unwrap_or_default();

        Ok(Self {
            locations,
            rows,
            cols,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::util::test_input;

    use super::*;

    #[test]
    fn cheapest_path() {
        let input = test_input(
            "
            1163751742
            1381373672
            2136511328
            3694931569
            7463417111
            1319128137
            1359912421
            3125421639
            1293138521
            2311944581
            ",
        );
        let grid = Grid::try_from(input).expect("could not parse input");
        assert_eq!(grid.rows, 10);
        assert_eq!(grid.cols, 10);
        assert_eq!(grid.bottom_right(), Location::new(9, 9));
        assert_eq!(
            grid.shortest(1, &Location::new(0, 0), &grid.bottom_right()),
            Some(40)
        );
    }

    #[test]
    fn scale() {
        let input = test_input(
            "
            8
            ",
        );
        let grid = Grid::try_from(input).expect("could not parse input");
        let scale = 5;
        assert_eq!(
            grid.get_scaled(&Location::new(0, 0), scale),
            Some(Chiton(8))
        );
        assert_eq!(
            grid.get_scaled(&Location::new(1, 1), scale),
            Some(Chiton(1))
        );
        assert_eq!(
            grid.get_scaled(&Location::new(1, 4), scale),
            Some(Chiton(4))
        );
        assert_eq!(
            grid.get_scaled(&Location::new(2, 2), scale),
            Some(Chiton(3))
        );
        assert_eq!(
            grid.get_scaled(&Location::new(3, 3), scale),
            Some(Chiton(5))
        );
        assert_eq!(
            grid.get_scaled(&Location::new(4, 4), scale),
            Some(Chiton(7))
        );
    }

    #[test]
    fn cheapest_scaled_path() {
        let input = test_input(
            "
            1163751742
            1381373672
            2136511328
            3694931569
            7463417111
            1319128137
            1359912421
            3125421639
            1293138521
            2311944581
            ",
        );
        let grid = Grid::try_from(input).expect("could not parse input");
        let scale = 5;
        assert_eq!(grid.rows, 10);
        assert_eq!(grid.cols, 10);
        assert_eq!(grid.scaled_bottom_right(scale), Location::new(49, 49));
        assert_eq!(
            grid.shortest(
                scale,
                &Location::new(0, 0),
                &grid.scaled_bottom_right(scale)
            ),
            Some(315)
        );
    }
}

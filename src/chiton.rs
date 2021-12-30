use std::{
    convert::{TryFrom, TryInto},
    ops::Deref,
};

use anyhow::{anyhow, Result};

use aoc_helpers::{
    generic::{
        pathing::{dijkstra_cost, DEdge, DefaultLocationCache},
        prelude::*,
        Grid, Location,
    },
    Solver,
};

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
    fscore: usize,
}

impl Node {
    pub fn new(idx: usize, cost: usize, fscore: usize) -> Self {
        Self { idx, cost, fscore }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .fscore
            .cmp(&self.fscore)
            // .then_with(|| other.cost.cmp(&self.cost))
            .then_with(|| self.idx.cmp(&other.idx))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct ChitonGrid(Grid<Chiton>);

impl Deref for ChitonGrid {
    type Target = Grid<Chiton>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ChitonGrid {
    pub fn shortest(&self, scale: usize, start: &Location, end: &Location) -> Option<usize> {
        let mut cache: DefaultLocationCache<usize> =
            DefaultLocationCache::new(self.size() * scale * scale, self.rows() * scale);

        dijkstra_cost(*start, *end, &mut cache, |loc| {
            // so this is a little weird, but we actually have much better
            // performance pre-allocating then extending. I would rather return
            // an iterator from the closure, but existential types, not really
            // a thing in that regard yet.
            let mut edges = Vec::with_capacity(4);
            edges.extend(loc.orthogonal_neighbors().filter_map(|n| {
                self.get_scaled(&n, scale, |chiton, r_fac, c_fac| {
                    let mut v = chiton.0 + r_fac + c_fac;
                    if v > 9 {
                        v = v % 10 + 1;
                    }
                    Chiton(v)
                })
                .map(|cost| DEdge::new(n, cost.0))
            }));
            edges
        })
    }
}

impl TryFrom<Vec<String>> for ChitonGrid {
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

        Ok(Self(locations.try_into()?))
    }
}

impl Solver for ChitonGrid {
    const ID: &'static str = "chiton";
    const DAY: usize = 15;

    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Self::P1 {
        self.shortest(1, &self.top_left(), &self.bottom_right())
            .expect("could not find cheapest path")
    }

    fn part_two(&mut self) -> Self::P2 {
        let scale = 5;
        self.shortest(scale, &self.top_left(), &self.scaled_bottom_right(scale))
            .expect("could not find cheapest path")
    }
}

#[cfg(test)]
mod tests {
    use aoc_helpers::util::test_input;

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
        let grid = ChitonGrid::try_from(input).expect("could not parse input");
        assert_eq!(grid.rows(), 10);
        assert_eq!(grid.cols(), 10);
        assert_eq!(grid.bottom_right(), Location::new(9, 9));
        assert_eq!(
            grid.shortest(1, &Location::new(0, 0), &grid.bottom_right()),
            Some(40)
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
        let grid = ChitonGrid::try_from(input).expect("could not parse input");
        let scale = 5;
        assert_eq!(grid.rows(), 10);
        assert_eq!(grid.cols(), 10);
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

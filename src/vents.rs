use std::num::ParseIntError;
use std::str::FromStr;
use std::{cmp::Ordering, convert::TryFrom};

use anyhow::{anyhow, bail, Result};
use aoc_helpers::Solver;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let values: Vec<i64> = s
            .split(',')
            .map(|v| v.parse())
            .collect::<std::result::Result<Vec<i64>, ParseIntError>>()?;
        if values.len() != 2 {
            bail!("Invalid input for point: {}", s);
        }
        Ok(Point {
            x: values[0],
            y: values[1],
        })
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    pub fn is_unmappable(&self) -> bool {
        !(self.start.x == self.end.x || self.start.y == self.end.y || self.is_diagonal())
    }

    pub fn is_diagonal(&self) -> bool {
        (self.start.x - self.end.x).abs() == (self.start.y - self.end.y).abs()
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        // this works if we only consider vertical or horizontal
        let xadj = match self.start.x.cmp(&self.end.x) {
            Ordering::Greater => -1,
            Ordering::Equal => 0,
            Ordering::Less => 1,
        };

        let yadj = match self.start.y.cmp(&self.end.y) {
            Ordering::Greater => -1,
            Ordering::Equal => 0,
            Ordering::Less => 1,
        };

        let count = (self.start.x - self.end.x)
            .abs()
            .max((self.start.y - self.end.y).abs())
            + 1;

        let sx = self.start.x;
        let sy = self.start.y;

        (0..count).map(move |i| Point::new(sx + i * xadj, sy + i * yadj))
    }

    // originally I was going to attempt to use this more cleverly to check for
    // intersections, but I think it actually would have been slower
    pub fn intersects(&self, point: &Point) -> bool {
        if *point == self.start || *point == self.end {
            return true;
        }

        let cross = (point.y - self.start.y) * (self.end.x - self.start.x)
            - (point.x - self.start.x) * (self.end.y - self.start.y);

        if cross.abs() > 0 {
            return false;
        }

        let dot = (point.x - self.start.x) * (self.end.x - self.start.x)
            + (point.y - self.start.y) * (self.end.y - self.start.y);
        if dot < 0 {
            return false;
        }

        let l = (self.end.x - self.start.x).pow(2) + (self.end.y - self.start.y).pow(2);
        if dot > l {
            return false;
        }

        true
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(" -> ");
        let start = Point::from_str(
            parts
                .next()
                .ok_or_else(|| anyhow!("Missing first point: {}", s))?,
        )?;
        let end = Point::from_str(
            parts
                .next()
                .ok_or_else(|| anyhow!("Missing second point: {}", s))?,
        )?;
        Ok(Line::new(start, end))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Vents {
    lines: Vec<Line>,
}

impl Vents {
    pub fn new(lines: Vec<Line>) -> Self {
        Self { lines }
    }

    pub fn prune_unmappable(&mut self) {
        self.lines.retain(|l| !l.is_unmappable());
    }

    pub fn prune_diagonal(&mut self) {
        self.lines.retain(|l| !l.is_diagonal());
    }

    pub fn count_multi_overlap(&self) -> usize {
        let mut checked: FxHashMap<Point, u64> = FxHashMap::default();

        for line in &self.lines {
            for point in line.points() {
                checked.entry(point).and_modify(|e| *e += 1).or_insert(1);
            }
        }

        checked.values().filter(|v| **v > 1).count()
    }
}

impl TryFrom<Vec<String>> for Vents {
    type Error = anyhow::Error;

    fn try_from(input: Vec<String>) -> Result<Self> {
        let lines = input
            .par_iter()
            .map(|l| Line::from_str(l))
            .filter(|l| {
                if let Ok(ref line) = l {
                    !line.is_unmappable()
                } else {
                    true
                }
            })
            .collect::<Result<Vec<Line>>>()?;

        Ok(Vents::new(lines))
    }
}

impl Solver for Vents {
    const ID: &'static str = "hydrothermal vents";
    const DAY: usize = 5;

    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Self::P1 {
        self.prune_diagonal();
        self.count_multi_overlap()
    }

    fn part_two(&mut self) -> Self::P2 {
        self.count_multi_overlap()
    }

    // this needs to happen separately because we need to calculate
    // the part 2 solution prior to the part 1 solution
    fn solve() -> aoc_helpers::Solution<Self::P1, Self::P2> {
        let mut instance = Self::instance();
        let part2 = instance.count_multi_overlap();
        instance.prune_diagonal();

        aoc_helpers::Solution::new(instance.count_multi_overlap(), part2)
    }
}

#[cfg(test)]
mod tests {
    mod line {
        use super::super::*;

        #[test]
        fn diagonal() {
            let line = Line::from_str("1,1 -> 1,3").expect("Could not make line");
            assert!(!line.is_diagonal());

            let line = Line::from_str("1,1 -> 3,3").expect("Could not make line");
            assert!(line.is_diagonal());

            let line = Line::from_str("1,3 -> 3,1").expect("Could not make line");
            assert!(line.is_diagonal());
        }

        #[test]
        fn unmappable() {
            let line = Line::from_str("1,1 -> 1,3").expect("Could not make line");
            assert!(!line.is_unmappable());

            let line = Line::from_str("1,1 -> 3,3").expect("Could not make line");
            assert!(!line.is_unmappable());

            let line = Line::from_str("1,3 -> 3,1").expect("Could not make line");
            assert!(!line.is_unmappable());

            let line = Line::from_str("1,3 -> 3,0").expect("Could not make line");
            assert!(line.is_unmappable());
        }

        #[test]
        fn points() {
            let line = Line::from_str("1,1 -> 1,3").expect("Could not make line");
            let expected = vec![Point::new(1, 1), Point::new(1, 2), Point::new(1, 3)];
            let points = line.points();

            assert_eq!(points.collect::<Vec<Point>>(), expected);

            let line = Line::from_str("1,1 -> 3,1").expect("Could not make line");
            let expected = vec![Point::new(1, 1), Point::new(2, 1), Point::new(3, 1)];
            let points = line.points();

            assert_eq!(points.collect::<Vec<Point>>(), expected);

            let line = Line::from_str("1,1 -> 3,3").expect("Could not make line");
            let expected = vec![Point::new(1, 1), Point::new(2, 2), Point::new(3, 3)];
            let points = line.points();

            assert_eq!(points.collect::<Vec<Point>>(), expected);

            let line = Line::from_str("1,3 -> 3,1").expect("Could not make line");
            let expected = vec![Point::new(1, 3), Point::new(2, 2), Point::new(3, 1)];
            let points = line.points();

            assert_eq!(points.collect::<Vec<Point>>(), expected);
        }

        #[test]
        fn intersect_with_point() {
            let line = Line::from_str("1,1 -> 1,3").expect("Could not make line");

            let p1 = Point::new(1, 1);
            let p2 = Point::new(1, 2);
            let p3 = Point::new(1, 3);

            assert!(line.intersects(&p1));
            assert!(line.intersects(&p2));
            assert!(line.intersects(&p3));

            let p4 = Point::new(1, 4);
            let p5 = Point::new(1, -1);
            let p6 = Point::new(2, 1);
            let p7 = Point::new(0, 1);

            assert!(!line.intersects(&p4));
            assert!(!line.intersects(&p5));
            assert!(!line.intersects(&p6));
            assert!(!line.intersects(&p7));
        }
    }

    mod grid {
        use aoc_helpers::util::test_input;

        use super::super::*;

        #[test]
        fn count_multiple_overlaps_without_diagonal() {
            let input = test_input(
                "
                0,9 -> 5,9
                8,0 -> 0,8
                9,4 -> 3,4
                2,2 -> 2,1
                7,0 -> 7,4
                6,4 -> 2,0
                0,9 -> 2,9
                3,4 -> 1,4
                0,0 -> 8,8
                5,5 -> 8,2
                ",
            );
            let mut grid = Vents::try_from(input).expect("Could not construct grid");
            grid.prune_unmappable();
            grid.prune_diagonal();
            assert_eq!(grid.count_multi_overlap(), 5);
        }

        #[test]
        fn count_multiple_overlaps() {
            let input = test_input(
                "
                0,9 -> 5,9
                8,0 -> 0,8
                9,4 -> 3,4
                2,2 -> 2,1
                7,0 -> 7,4
                6,4 -> 2,0
                0,9 -> 2,9
                3,4 -> 1,4
                0,0 -> 8,8
                5,5 -> 8,2
                ",
            );
            let mut grid = Vents::try_from(input).expect("Could not construct grid");
            grid.prune_unmappable();
            assert_eq!(grid.count_multi_overlap(), 12);
        }
    }
}

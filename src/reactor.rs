//! So can we just store cubes and calculate the intersections of cubes via
//! sweeping a plane through the region of space? Like we would for intersection
//! of rectangles but with another dimension?
//!
//! How to handle the situations where some regions turn things on and others
//! turn things off? And there's a sequence to it...
use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
use rayon::prelude::*;
use rustc_hash::FxHashSet;
use std::{convert::TryFrom, iter::FromIterator, str::FromStr};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl From<(i64, i64, i64)> for Point {
    fn from(v: (i64, i64, i64)) -> Self {
        Self {
            x: v.0,
            y: v.1,
            z: v.2,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Line {
    start: i64,
    end: i64,
}

impl Line {
    pub fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    pub fn length(&self) -> i64 {
        (self.end - self.start).abs() + 1
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.start >= other.start && self.start <= other.end
            || other.start >= self.start && other.start <= self.end
            || self.start >= other.start && self.end <= other.end
            || other.start >= self.start && other.end <= self.end
    }

    pub fn merge(&mut self, other: &Self) -> bool {
        if !self.intersects(other) {
            return false;
        }

        self.start = self.start.min(other.start);
        self.end = self.end.max(other.end);

        true
    }

    pub fn overlap(&self, other: &Self) -> Self {
        Self::new(self.start.max(other.start), self.end.min(other.end))
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Rectangle {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Rectangle {
    pub fn new(min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.x_instersects(other) && self.y_instersects(other)
    }

    fn x_instersects(&self, other: &Self) -> bool {
        other.min_x >= self.min_x && other.min_x <= self.max_x
            || self.min_x >= other.min_x && self.min_x <= other.max_x
            // they contain us
            || self.min_x >= other.min_x && self.max_x <= other.max_x
            // we contain them
            || other.min_x >= self.min_x && other.max_x <= self.max_x
    }

    fn y_instersects(&self, other: &Self) -> bool {
        other.min_y >= self.min_y && other.min_y <= self.max_y
            || self.min_y >= other.min_y && self.min_y <= other.max_y
            // they contain us
            || self.min_y >= other.min_y && self.max_y <= other.max_y
            // we contain them
            || other.min_y >= self.min_y && other.max_y <= self.max_y
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self::new(
            self.min_x.max(other.min_x),
            self.max_x.min(other.max_x),
            self.min_y.max(other.min_y),
            self.max_y.min(other.max_y),
        )
    }

    pub fn width(&self) -> i64 {
        (self.max_x - self.min_x).abs() + 1
    }

    pub fn height(&self) -> i64 {
        (self.max_y - self.min_y).abs() + 1
    }

    pub fn area(&self) -> i64 {
        self.width() * self.height()
    }

    fn intersects_x(&self, x: i64) -> bool {
        x >= self.min_x && x <= self.max_x
    }

    fn y_line(&self) -> Line {
        Line::new(self.min_y, self.max_y)
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Cuboid {
    begin: Point,
    end: Point,
}

impl Cuboid {
    pub fn new(begin: Point, end: Point) -> Self {
        Self { begin, end }
    }

    pub fn intersects_plane(&self, z: i64) -> bool {
        z >= self.begin.z && z <= self.end.z
    }

    pub fn rect_for_intersect(&self) -> Rectangle {
        Rectangle::new(self.begin.x, self.end.x, self.begin.y, self.end.y)
    }

    pub fn width(&self) -> i64 {
        (self.end.x - self.begin.x).abs() + 1
    }

    pub fn height(&self) -> i64 {
        (self.begin.y - self.end.y).abs() + 1
    }

    pub fn depth(&self) -> i64 {
        (self.begin.z - self.end.z).abs() + 1
    }

    pub fn volume(&self) -> i64 {
        self.width() * self.height() * self.depth()
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let int_b_x = self.begin.x.max(other.begin.x);
        let int_b_y = self.begin.y.max(other.begin.y);
        let int_b_z = self.begin.z.max(other.begin.z);
        let int_e_x = self.end.x.min(other.end.x);
        let int_e_y = self.end.y.min(other.end.y);
        let int_e_z = self.end.z.min(other.end.z);

        int_b_x <= int_e_x && int_b_y <= int_e_y && int_b_z <= int_e_z
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self::new(
            (
                self.begin.x.max(other.begin.x),
                self.begin.y.max(other.begin.y),
                self.begin.z.max(other.begin.z),
            )
                .into(),
            (
                self.end.x.min(other.end.x),
                self.end.y.min(other.end.y),
                self.end.z.min(other.end.z),
            )
                .into(),
        )
    }

    pub fn fully_contains(&self, other: &Self) -> bool {
        other.begin.x >= self.begin.x
            && other.end.x <= self.end.x
            && other.begin.y >= self.begin.y
            && other.end.y <= self.end.y
            && other.begin.z >= self.begin.z
            && other.end.z <= self.end.z
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Region {
    /// we can track when this region was created
    index: usize,
    cuboid: Cuboid,
    on: bool,
}

impl Region {
    pub fn new(index: usize, cuboid: Cuboid, on: bool) -> Self {
        Self { index, cuboid, on }
    }

    pub fn volume(&self) -> i64 {
        if self.on {
            self.cuboid.volume()
        } else {
            -self.cuboid.volume()
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.cuboid.intersects(&other.cuboid)
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self::new(
            self.index,
            self.cuboid.intersection(&other.cuboid),
            !self.on,
        )
    }

    pub fn intersects_plane(&self, z: i64) -> bool {
        self.cuboid.intersects_plane(z)
    }
}

impl FromStr for Region {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (_, (on, ranges)) = region_parser(s).map_err(|_| anyhow!("could not parse input"))?;

        if ranges.len() != 3 {
            bail!("invalid number of ranges: {}", s);
        }

        let begin = Point::from((ranges[0].0, ranges[1].0, ranges[2].0));
        let end = Point::from((ranges[0].1, ranges[1].1, ranges[2].1));
        let cuboid = Cuboid { begin, end };

        Ok(Region {
            cuboid,
            on,
            ..Region::default()
        })
    }
}

fn range_parser(input: &str) -> IResult<&str, (i64, i64)> {
    preceded(
        tuple((alt((tag("x"), tag("y"), tag("z"))), tag("="))),
        separated_pair(complete::i64, tag(".."), complete::i64),
    )(input)
}

fn region_parser(input: &str) -> IResult<&str, (bool, Vec<(i64, i64)>)> {
    let (input, (state, ranges)) = tuple((
        terminated(alt((tag("on"), tag("off"))), space1),
        separated_list1(tag(","), range_parser),
    ))(input)?;

    let on = match state {
        "on" => true,
        "off" => false,
        _ => unreachable!("this should not be possible"),
    };

    Ok((input, (on, ranges)))
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Instructions {
    regions: Vec<Region>,
}

impl TryFrom<Vec<String>> for Instructions {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let regions = value
            .iter()
            .enumerate()
            .map(|(idx, s)| {
                Region::from_str(s).map(|mut r| {
                    r.index = idx;
                    r
                })
            })
            .collect::<Result<Vec<Region>>>()?;

        Ok(Self { regions })
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Reactor {
    regions: Vec<Region>,
}

impl Reactor {
    pub fn reboot(&mut self, instructions: &Instructions) {
        self.regions = instructions.regions.clone();
    }

    pub fn volume(&self, limit: &Option<Cuboid>) -> i64 {
        let regions: Vec<Region> = if let Some(limit) = limit {
            self.regions
                .iter()
                .cloned()
                .filter(|r| limit.fully_contains(&r.cuboid))
                .collect()
        } else {
            self.regions.clone()
        };

        let mut final_regions: Vec<Region> = Vec::with_capacity(regions.len() * 200);

        for region in regions.iter() {
            if final_regions.is_empty() {
                if region.on {
                    final_regions.push(*region);
                }
                continue;
            }

            for fr_idx in 0..final_regions.len() {
                let f = final_regions[fr_idx];
                if f.intersects(region) {
                    final_regions.push(f.intersection(region));
                }
            }

            if region.on {
                final_regions.push(*region);
            }
        }

        final_regions.iter().fold(0, |acc, r| acc + r.volume())
    }

    pub fn compute_volume_of_on_cubes(&self, limit: &Option<Cuboid>) -> i64 {
        // sort by Z values
        let mut regions: Vec<Region> = if let Some(limit) = limit {
            self.regions
                .iter()
                .cloned()
                .filter(|r| limit.fully_contains(&r.cuboid))
                .collect()
        } else {
            self.regions.clone()
        };

        regions.sort_by(|a, b| a.cuboid.begin.z.cmp(&b.cuboid.begin.z));
        // sweep an x, y plane across the z values
        let min_z = regions
            .first()
            .map(|r| r.cuboid.begin.z)
            .unwrap_or_default();
        let max_z = regions
            .iter()
            .map(|r| r.cuboid.end.z)
            .max()
            .unwrap_or_default();

        (min_z..=max_z).fold(0_i64, |vol, z| {
            let mut local_intersections = Vec::with_capacity(regions.len());
            for region in regions.iter() {
                // this is safe because the list will be sorted
                if region.cuboid.begin.z > z {
                    break;
                }

                if region.intersects_plane(z) {
                    // we need to additionally track which region made this
                    // particular intersection
                    local_intersections.push((region.index, region.cuboid.rect_for_intersect()));
                }
            }

            // don't bother with other calculations
            if local_intersections.is_empty() {
                return vol;
            }

            // now that we have all the overlapping regions, we need to figure
            // out which regions correspond to lit areas by finding the actual
            // region overlaps

            // sort this so that they're in order from min_x to max_x
            local_intersections.sort_by(|a, b| a.1.min_x.cmp(&b.1.min_x));
            let local_area = self.reduce(&local_intersections);
            // area is the area of all rectangles that intersected the plane
            // minus the regions of overlaps

            vol + local_area
        })
    }

    fn reduce(&self, rects: &[(usize, Rectangle)]) -> i64 {
        let mut sum = 0;

        let mut unintersected = FxHashSet::from_iter(rects.iter());
        let mut intersected = FxHashSet::default();

        for (r1, r2) in rects.iter().tuple_combinations() {
            if r1.1.intersects(&r2.1) {
                unintersected.remove(r1);
                unintersected.remove(r2);
                intersected.insert(r1);
                intersected.insert(r2);
            }
        }

        sum += unintersected.iter().fold(0, |acc, (i, r)| {
            if self.regions[*i].on {
                acc + r.area()
            } else {
                acc
            }
        });

        if intersected.is_empty() {
            return sum;
        }

        let mut remaining = intersected
            .into_iter()
            .sorted_by(|a, b| a.1.min_x.cmp(&b.1.min_x))
            .collect::<Vec<_>>();

        // there was only a single intersection, special case
        if remaining.len() == 2 {
            remaining.sort_by(|a, b| b.0.cmp(&a.0));

            let top = remaining[0];
            let top_reg = self.regions[top.0];

            let bot = remaining[1];
            let bot_reg = self.regions[bot.0];

            // if both are off
            if !top_reg.on && !bot_reg.on {
                return sum;
            }

            let inter_area = top.1.intersection(&bot.1).area();

            // if both are on
            if top_reg.on && bot_reg.on {
                return sum + top.1.area() + bot.1.area() - inter_area;
            }

            // if the top is off, take the chunk out
            if !top_reg.on && bot_reg.on {
                return sum + bot.1.area() - inter_area;
            }

            return sum + top.1.area();
        }

        // for the remaining rectangles, we need to perform the same procedure
        // as the cubes, but with one less dimension
        let min_x = remaining.first().map(|(_, r)| r.min_x).unwrap_or_default();
        let max_x = remaining
            .iter()
            .map(|(_, r)| r.max_x)
            .max()
            .unwrap_or_default();

        sum += (min_x..=max_x)
            .into_par_iter()
            .map(|x| {
                let mut local_lines = Vec::with_capacity(remaining.len());
                for (idx, rect) in remaining.iter() {
                    if rect.min_x > x {
                        break;
                    }

                    if rect.intersects_x(x) {
                        local_lines.push((*idx, rect.y_line()));
                    }
                }

                if local_lines.is_empty() {
                    return 0;
                }

                // let's be stupid

                local_lines.sort_by(|a, b| a.0.cmp(&b.0));

                let min_y = local_lines
                    .iter()
                    .map(|(_, line)| line.start)
                    .min()
                    .unwrap_or_default();
                let max_y = local_lines
                    .iter()
                    .map(|(_, line)| line.end)
                    .max()
                    .unwrap_or_default();

                let mut tracking = vec![false; (max_y - min_y) as usize + 1];

                for (idx, line) in local_lines.iter() {
                    let region = self.regions[*idx];
                    for y in line.start..=line.end {
                        let v = region.on;
                        tracking[(y - min_y) as usize] = v;
                    }
                }

                tracking.iter().filter(|v| **v).count() as i64

                // let overlaps = self.reduce_lines(&local_lines);

                // let local_area = local_lines
                //     .iter()
                //     .fold(0, |acc, (idx, line)| {
                //         if self.regions[*idx].on {
                //             acc + line.length()
                //         } else {
                //             acc
                //         }
                //     });

                // let overlap_area = overlaps.iter().fold(0, |acc, (_, line)| acc + line.length());

                // tot + local_area - overlap_area
            })
            .sum::<i64>();

        sum
    }

    // pub fn reduce_lines(&self, lines: &Vec<(usize, Line)>) -> Vec<(usize, Line)> {
    //     let mut overlaps: Vec<(usize, Line)> = Vec::with_capacity(lines.len());

    //     for (a, b) in lines.iter().tuple_combinations() {
    //         if a.1.intersects(&b.1) {
    //             overlaps.push((a.0.max(b.0), a.1.overlap(&b.1)));
    //         }
    //     }

    //     if overlaps.len() < 2 {
    //         return overlaps;
    //     }

    //     overlaps.sort_by(|a, b| {
    //         b.1.start
    //             .cmp(&a.1.start)
    //             .then_with(|| b.1.length().cmp(&a.1.length()))
    //     });

    //     let mut unique: Vec<(usize, Line)> = Vec::with_capacity(overlaps.len());

    //     while let Some(last) = overlaps.pop() {
    //         if !unique.iter_mut().any(|(idx, line)| line.merge(&last.1)) {
    //             unique.push(last);
    //         }
    //     }

    //     unique
    // }
}

#[cfg(test)]
mod tests {
    mod region {
        use super::super::*;

        #[test]
        fn from_str() {
            Region::from_str("on x=-20..26,y=-36..17,z=-47..7").expect("could not parse region");
        }
    }

    mod reactor {
        use aoc_helpers::util::test_input;

        use super::super::*;

        #[test]
        fn solving() {
            let input = test_input(
                "
                on x=-20..26,y=-36..17,z=-47..7
                on x=-20..33,y=-21..23,z=-26..28
                on x=-22..28,y=-29..23,z=-38..16
                on x=-46..7,y=-6..46,z=-50..-1
                on x=-49..1,y=-3..46,z=-24..28
                on x=2..47,y=-22..22,z=-23..27
                on x=-27..23,y=-28..26,z=-21..29
                on x=-39..5,y=-6..47,z=-3..44
                on x=-30..21,y=-8..43,z=-13..34
                on x=-22..26,y=-27..20,z=-29..19
                off x=-48..-32,y=26..41,z=-47..-37
                on x=-12..35,y=6..50,z=-50..-2
                off x=-48..-32,y=-32..-16,z=-15..-5
                on x=-18..26,y=-33..15,z=-7..46
                off x=-40..-22,y=-38..-28,z=23..41
                on x=-16..35,y=-41..10,z=-47..6
                off x=-32..-23,y=11..30,z=-14..3
                on x=-49..-5,y=-3..45,z=-29..18
                off x=18..30,y=-20..-8,z=-3..13
                on x=-41..9,y=-7..43,z=-33..15
                on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
                on x=967..23432,y=45373..81175,z=27513..53682
                ",
            );

            let insts = Instructions::try_from(input).expect("could not parse input");

            let limit = Cuboid {
                begin: (-50, -50, -50).into(),
                end: (50, 50, 50).into(),
            };
            let mut reactor = Reactor::default();
            reactor.reboot(&insts);

            assert_eq!(reactor.volume(&Some(limit)), 590784);
        }
    }
}

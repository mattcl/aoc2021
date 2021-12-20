use std::{
    convert::{TryFrom, TryInto},
    fmt,
    str::FromStr,
};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Algorithm {
    lookup: [bool; 512],
}

impl Algorithm {
    pub fn is_light(&self, val: usize) -> bool {
        self.lookup[val]
    }
}

impl FromStr for Algorithm {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let vals: Vec<bool> = s.chars().map(|ch| ch == '#').collect();
        Ok(Self {
            lookup: vals
                .try_into()
                .map_err(|_| anyhow!("Failed to parse algorithm"))?,
        })
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Bound {
    min_row: i64,
    max_row: i64,
    min_col: i64,
    max_col: i64,
}

impl Bound {
    pub fn width(&self) -> usize {
        (self.max_col - self.min_col).abs() as usize + 1
    }

    pub fn height(&self) -> usize {
        (self.max_row - self.min_row).abs() as usize + 1
    }

    pub fn translate(&self, pixel: &Pixel) -> (usize, usize) {
        (
            (pixel.0 - self.min_row) as usize,
            (pixel.1 - self.min_col) as usize,
        )
    }

    pub fn size(&self) -> usize {
        self.width() * self.height()
    }

    pub fn contains(&self, pixel: &Pixel) -> bool {
        pixel.0 >= self.min_row
            && pixel.0 <= self.max_row
            && pixel.1 >= self.min_col
            && pixel.1 <= self.max_col
    }
}

pub const NEIGHBOR_ORDER: [(i64, i64); 9] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

type Pixel = (i64, i64);

#[derive(Debug, Clone, Default)]
pub struct Image {
    pixels: FxHashSet<Pixel>,
    bounds: Bound,
    gen: usize,
}

impl Image {
    pub fn bounds(&self) -> &Bound {
        &self.bounds
    }

    pub fn enhance(&self, algo: &Algorithm) -> Self {
        let mut new_image = Self {
            gen: self.gen + 1,
            ..Self::default()
        };

        let iter = ((self.bounds.min_row - 1)..=(self.bounds.max_row + 1))
            .into_par_iter()
            .map(move |row| {
                ((self.bounds.min_col - 1)..=(self.bounds.max_col + 1)).filter_map(move |col| {
                    let pix = (row, col);
                    let val = self.value_for_square(&pix, algo);

                    if algo.is_light(val) {
                        Some(pix)
                    } else {
                        None
                    }
                })
            })
            .flatten_iter();

        new_image.pixels = FxHashSet::from_par_iter(iter);

        new_image.recalc_bounds();
        new_image
    }

    pub fn num_lit(&self) -> usize {
        self.pixels.len()
    }

    pub fn value_for_square(&self, pix: &Pixel, algo: &Algorithm) -> usize {
        NEIGHBOR_ORDER
            .iter()
            .enumerate()
            .fold(0, |acc, (i, (r, c))| {
                let p: Pixel = (pix.0 + r, pix.1 + c);
                if self.pixels.contains(&p)
                    || (algo.is_light(0) && self.gen % 2 == 1 && !self.bounds.contains(&p))
                {
                    acc + 2_usize.pow(8 - i as u32)
                } else {
                    acc
                }
            })
    }

    pub fn set_pixel(&mut self, pixel: &Pixel) {
        self.pixels.insert(*pixel);
    }

    pub fn delete_pixel(&mut self, pixel: &Pixel) {
        self.pixels.remove(pixel);
    }

    pub fn recalc_bounds(&mut self) {
        let mut min_row = i64::MAX;
        let mut max_row = i64::MIN;
        let mut min_col = i64::MAX;
        let mut max_col = i64::MIN;

        for p in self.pixels.iter() {
            if p.0 < min_row {
                min_row = p.0;
            }

            if p.1 < min_col {
                min_col = p.1;
            }

            if p.0 > max_row {
                max_row = p.0;
            }

            if p.1 > max_col {
                max_col = p.1;
            }
        }

        self.bounds = Bound {
            min_row,
            max_row,
            min_col,
            max_col,
        };
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bound = self.bounds();
        let mut output = vec![vec!['.'; bound.width()]; bound.height()];
        for pix in self.pixels.iter() {
            let (row, col) = bound.translate(pix);
            output[row][col] = '#';
        }

        let disp: String = output
            .iter()
            .map(|r| r.iter().collect::<String>())
            .join("\n");
        write!(f, "{}", disp)
    }
}

impl From<&[String]> for Image {
    fn from(value: &[String]) -> Self {
        let pixels: FxHashSet<Pixel> = value
            .iter()
            .enumerate()
            .map(move |(row, s)| {
                s.chars().enumerate().filter_map(move |(col, ch)| match ch {
                    '#' => Some((row as i64, col as i64)),
                    _ => None,
                })
            })
            .flatten()
            .collect();

        let mut img = Self {
            pixels,
            ..Self::default()
        };
        img.recalc_bounds();
        img
    }
}

#[derive(Debug, Clone)]
pub struct Enhancer {
    pub algorithm: Algorithm,
    pub image: Image,
}

impl Enhancer {
    pub fn enhance(&mut self) {
        self.image = self.image.enhance(&self.algorithm);
    }

    pub fn enhance_times(&mut self, times: usize) -> &Image {
        for _ in 0..times {
            self.enhance();
        }
        &self.image
    }
}

impl TryFrom<Vec<String>> for Enhancer {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut parts = value.split(|s| s.is_empty());
        let algorithm = Algorithm::from_str(
            parts
                .next()
                .and_then(|s| s.first())
                .ok_or_else(|| anyhow!("Input too short"))?,
        )?;

        let image = Image::try_from(parts.next().ok_or_else(|| anyhow!("Input too short"))?)?;

        Ok(Self { algorithm, image })
    }
}

#[cfg(test)]
mod tests {
    mod image {
        use aoc_helpers::util::test_input;

        use super::super::*;

        #[test]
        fn parsing() {
            let input = test_input(
                "
                #..#.
                #....
                ##..#
                ..#..
                ..###
                ",
            );
            let image = Image::from(input.as_ref());
            assert_eq!(image.pixels.len(), 10);
        }

        #[test]
        fn enhancing() {
            let input = test_input("
                ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

                #..#.
                #....
                ##..#
                ..#..
                ..###
                ");

            let mut enhancer = Enhancer::try_from(input).expect("could not parse input");
            let img = enhancer.enhance_times(2);
            assert_eq!(img.num_lit(), 35);
        }
    }
}

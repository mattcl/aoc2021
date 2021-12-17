use anyhow::{bail, Result};
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Target {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl Target {
    pub fn new(x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    pub fn contains(&self, point: (i64, i64)) -> bool {
        point.0 >= self.x_min
            && point.0 <= self.x_max
            && point.1 >= self.y_min
            && point.1 <= self.y_max
    }
}

use std::{num::ParseIntError, str::FromStr};

impl FromStr for Target {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<_> = s
            .split_whitespace()
            .map(|s| {
                s.chars()
                    .filter(|ch| ch.is_digit(10) || *ch == '-' || *ch == '.')
                    .collect::<String>()
            })
            .filter(|s| !s.is_empty())
            .collect();

        if parts.len() != 2 {
            bail!("Invalid input: {}", s);
        }

        let x = parts[0]
            .split("..")
            .map(|part| part.parse())
            .collect::<std::result::Result<Vec<i64>, ParseIntError>>()?;
        let y = parts[1]
            .split("..")
            .map(|part| part.parse())
            .collect::<std::result::Result<Vec<i64>, ParseIntError>>()?;

        if x.len() != 2 || y.len() != 2 {
            bail!("Invalid input: {}", s);
        }

        Ok(Self::new(x[0], x[1], y[0], y[1]))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Probe {
    vx: i64,
    vy: i64,
}

impl Probe {
    pub fn new(vx: i64, vy: i64) -> Self {
        Self { vx, vy }
    }

    pub fn xt(&self, t: i64) -> i64 {
        // after vx steps, there's no additional change in x, since vx would
        // then be zero
        let t_max = self.vx.abs().min(t);
        self.vx * t_max - (t_max * (t_max - 1)) / 2
    }

    pub fn yt(&self, t: i64) -> i64 {
        self.vy * t - (t * (t - 1)) / 2
    }

    pub fn min_t_to_x(&self, x: i64) -> Option<i64> {
        if x > self.max_x() {
            None
        } else {
            let v = self.vx as f64;
            let b = 2_f64 * v + 1_f64;
            let t1 = (0.5 * ((b * b - 8_f64 * x as f64).sqrt() + b)).floor() as i64;
            let t2 = (-0.5 * ((b * b - 8_f64 * x as f64).sqrt() + 0.5 * b)).floor() as i64;
            Some(0.max(t1.min(t2)))
        }
    }

    pub fn min_t_to_y(&self, y: i64) -> Option<i64> {
        let v = self.vy as f64;
        let b = 2_f64 * v + 1_f64;
        let t1 = (0.5 * ((b * b - 8_f64 * y as f64).sqrt() + b)).floor() as i64;
        let t2 = (-0.5 * ((b * b - 8_f64 * y as f64).sqrt() + 0.5 * b)).floor() as i64;
        Some(0.max(t1.min(t2)))
    }

    pub fn point_at(&self, t: i64) -> (i64, i64) {
        (self.xt(t), self.yt(t))
    }

    pub fn max_x(&self) -> i64 {
        self.xt(self.vx.abs())
    }

    pub fn max_height(&self) -> i64 {
        if self.vy <= 0 {
            0
        } else {
            self.yt(self.vy.abs())
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Launcher;

impl Launcher {
    pub fn launch(&self, target: &Target) -> (i64, usize) {
        let mut cache: FxHashSet<Probe> = FxHashSet::default();
        let min_vx = (0.5 * ((target.x_min as f64 * 8_f64 + 1_f64).sqrt() - 1_f64)).ceil() as i64;
        let max_vx = target.x_max;

        // given min/max vx, figure all all times t which are valid in target area
        let mut max = 0;
        // similar for vx, our starting min is the y_min of the target
        // (reaching in 1 step)
        for vx in min_vx..=max_vx {
            let mut probe = Probe::new(vx, 0);
            if let Some(t_min) = probe.min_t_to_x(target.x_min) {
                for vy in target.y_min..=target.y_min.abs() {
                    probe.vy = vy;

                    let mut t = t_min;
                    // find first t where x is in the target in the target
                    // sim until x pos is in target
                    let contained = loop {
                        // we know this is guaranteed to happen because of min_x
                        let x = probe.xt(t);
                        if target.contains((x, target.y_min)) {
                            break true;
                        }
                        t += 1;

                        if x > target.x_max {
                            break false;
                        }
                    };

                    if !contained {
                        // we couldn't actually get a valid x position for any t,
                        // so continue
                        continue;
                    }

                    // adjust t to the time the probe would be crossing the zero
                    // line again
                    if vy > 0 && t < vy * 2 {
                        t = vy * 2;
                    }

                    // we now know the first t to start simulation of y from
                    loop {
                        let p = probe.point_at(t);
                        if target.contains(p) {
                            // this probe would be valid
                            cache.insert(probe);
                            let cur_max = probe.yt(probe.vy.min(t));
                            if cur_max > max {
                                max = cur_max;
                            }
                            break;
                        }

                        if p.1 < target.y_min {
                            // this probe is not valid
                            break;
                        }

                        t += 1;
                    }
                }
            }
        }
        (max, cache.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let target = Target::new(20, 30, -10, -5);
        let l = Launcher {};
        let (highest, num) = l.launch(&target);
        assert_eq!(highest, 45);
        assert_eq!(num, 112);
    }
}

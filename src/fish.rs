use std::{num::ParseIntError, str::FromStr};

use rustc_hash::FxHashMap;

const SPAWN_INTERVAL: i64 = 7;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Lanternfish(pub i64);

impl Lanternfish {
    pub fn new() -> Self {
        Self(8)
    }

    /// Compute the day this fish next spawns
    pub fn next_spawn(&self) -> i64 {
        self.0 + 1
    }

    /// Compute the number of direct children this fish produces in `days` time
    pub fn num_children(&self, days: i64) -> i64 {
        let next = self.next_spawn();
        if days < 1 || next > days {
            return 0;
        }
        (days - next) / SPAWN_INTERVAL + 1
    }

    /// Return an iterator of the future days where spawns occur in `days` time
    pub fn days_spawns_occur(&self, days: i64) -> impl Iterator<Item = i64> {
        let count = self.num_children(days);
        let next = self.next_spawn();
        (0..count)
            .into_iter()
            .map(move |i| i * SPAWN_INTERVAL + next)
    }

    pub fn num_descendants(
        &self,
        days: i64,
        cache: &mut FxHashMap<(Lanternfish, i64), usize>,
    ) -> usize {
        if let Some(v) = cache.get(&(*self, days)) {
            return *v;
        }

        let spawn_days = self.days_spawns_occur(days);
        let count = spawn_days
            .map(|day| 1 + Lanternfish::new().num_descendants(days - day, cache))
            .sum();

        cache.insert((*self, days), count);

        count
    }
}

impl Default for Lanternfish {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Lanternfish {
    type Err = ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

#[derive(Debug, Clone)]
pub struct Sim {
    starting_fish: Vec<Lanternfish>,
}

impl Sim {
    pub fn new(starting_fish: Vec<Lanternfish>) -> Self {
        Self { starting_fish }
    }

    pub fn population_after(&self, days: i64) -> usize {
        let mut cache = FxHashMap::default();
        self.starting_fish
            .iter()
            .map(|f| f.num_descendants(days, &mut cache))
            .sum::<usize>()
            + self.starting_fish.len()
    }
}

impl FromStr for Sim {
    type Err = ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self::new(
            s.split(',')
                .map(|p| p.parse())
                .collect::<std::result::Result<Vec<Lanternfish>, ParseIntError>>()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    mod lanternfish {
        use super::super::*;

        #[test]
        fn next_spawn() {
            // fish has exactly seven days to spawn
            let fish = Lanternfish(6);
            assert_eq!(fish.next_spawn(), 7);

            let fish = Lanternfish(8);
            assert_eq!(fish.next_spawn(), 9);

            let fish = Lanternfish(2);
            assert_eq!(fish.next_spawn(), 3);

            let fish = Lanternfish(0);
            assert_eq!(fish.next_spawn(), 1);
        }

        #[test]
        fn num_direct_children() {
            // fish has exactly seven days to spawn
            let fish = Lanternfish(6);

            assert_eq!(fish.num_children(6), 0);
            assert_eq!(fish.num_children(7), 1);
            assert_eq!(fish.num_children(13), 1);
            assert_eq!(fish.num_children(14), 2);

            // fish is on the way to spawning
            let fish = Lanternfish(3);

            assert_eq!(fish.num_children(6), 1);
            assert_eq!(fish.num_children(7), 1);
            assert_eq!(fish.num_children(13), 2);
            assert_eq!(fish.num_children(14), 2);

            // new fish, takes longer to spawn
            let fish = Lanternfish(8);

            assert_eq!(fish.num_children(6), 0);
            assert_eq!(fish.num_children(7), 0);
            assert_eq!(fish.num_children(8), 0);
            assert_eq!(fish.num_children(9), 1);

            // just spawned today, but that shouldn't be in the future count
            let fish = Lanternfish(0);

            assert_eq!(fish.num_children(0), 0);
            assert_eq!(fish.num_children(1), 1);
            assert_eq!(fish.num_children(13), 2);
            assert_eq!(fish.num_children(14), 2);

            // failed with this on the sample input, so have a test for it
            let fish = Lanternfish(4);

            assert_eq!(fish.num_children(4), 0);
            assert_eq!(fish.num_children(5), 1);
        }

        #[test]
        fn days_spawns_occur() {
            let fish = Lanternfish(6);
            let expected: Vec<i64> = vec![];
            assert_eq!(fish.days_spawns_occur(0).collect::<Vec<i64>>(), expected);
            assert_eq!(fish.days_spawns_occur(6).collect::<Vec<i64>>(), expected);

            // fish has exactly seven days to spawn
            let fish = Lanternfish(6);
            let expected = vec![7, 14, 21, 28];
            assert_eq!(fish.days_spawns_occur(30).collect::<Vec<i64>>(), expected);

            // fish has exactly seven days to spawn
            let fish = Lanternfish(3);
            let expected = vec![4, 11, 18, 25];
            assert_eq!(fish.days_spawns_occur(30).collect::<Vec<i64>>(), expected);

            let fish = Lanternfish(8);
            let expected = vec![9, 16, 23, 30];
            assert_eq!(fish.days_spawns_occur(30).collect::<Vec<i64>>(), expected);
        }
    }

    mod sim {
        use super::super::*;

        #[test]
        fn simulating() {
            let sim = Sim::from_str("3,4,3,1,2").expect("Could not create sim");
            assert_eq!(sim.population_after(1), 5);
            assert_eq!(sim.population_after(2), 6);
            assert_eq!(sim.population_after(3), 7);
            assert_eq!(sim.population_after(4), 9);
            assert_eq!(sim.population_after(18), 26);
            assert_eq!(sim.population_after(80), 5934);
            assert_eq!(sim.population_after(256), 26984457539);
        }
    }
}

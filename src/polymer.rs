use std::{convert::TryFrom, str::FromStr};

use anyhow::{anyhow, bail, Result};
use itertools::{Itertools, MinMaxResult};
use rustc_hash::FxHashMap;

type Cache = FxHashMap<(usize, [char; 2]), [usize; 26]>;

#[derive(Debug, Clone, Copy)]
pub struct Rule {
    key: [char; 2],
    insertion: char,
    insertion_value: usize,
}

impl Rule {
    pub fn iterations(&self, num: usize, rules: &Rules, cache: &mut Cache) -> [usize; 26] {
        self.recur(num, rules, cache)
    }

    pub fn recur(&self, depth: usize, rules: &Rules, cache: &mut Cache) -> [usize; 26] {
        if let Some(cached) = cache.get(&(depth, self.key)) {
            return *cached;
        }

        let mut counts = [0; 26];
        counts[self.insertion_value] += 1;

        if depth < 2 {
            return counts;
        }

        if let Some(left) = rules.get(&[self.key[0], self.insertion]) {
            for (i, v) in left.recur(depth - 1, rules, cache).iter().enumerate() {
                counts[i] += v;
            }
        }

        if let Some(right) = rules.get(&[self.insertion, self.key[1]]) {
            for (i, v) in right.recur(depth - 1, rules, cache).iter().enumerate() {
                counts[i] += v;
            }
        }

        cache.insert((depth, self.key), counts);

        counts
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(" -> ");
        let key: Vec<char> = parts
            .next()
            .ok_or_else(|| anyhow!("Missing key: {}", s))?
            .chars()
            .collect();

        if key.len() != 2 {
            bail!("Key is invalid length: {}", s);
        }

        let insertion = parts
            .next()
            .and_then(|p| p.chars().next())
            .ok_or_else(|| anyhow!("Missing insertion: {}", s))?;
        let insertion_value = insertion as usize - 'A' as usize;

        Ok(Rule {
            key: [key[0], key[1]],
            insertion,
            insertion_value,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Rules {
    rules: FxHashMap<[char; 2], Rule>,
}

impl Rules {
    pub fn get(&self, key: &[char; 2]) -> Option<&Rule> {
        self.rules.get(key)
    }
}

impl TryFrom<Vec<String>> for Rules {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut rules = FxHashMap::default();
        for s in &value {
            let r = Rule::from_str(s)?;
            rules.insert(r.key, r);
        }

        Ok(Rules { rules })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Formula(String);

impl From<String> for Formula {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Polymerizer {
    formula: Formula,
    rules: Rules,
}

impl Polymerizer {
    pub fn iterations(&self, num: usize) -> usize {
        let mut final_rules: FxHashMap<[char; 2], [usize; 26]> = FxHashMap::default();
        let mut counts = [0_usize; 26];

        for ch in self.formula.0.chars() {
            counts[ch as usize - 'A' as usize] += 1;
        }

        let mut cache: Cache = FxHashMap::default();

        for (key, rule) in self.rules.rules.iter() {
            final_rules.insert(*key, rule.iterations(num, &self.rules, &mut cache));
        }

        for (begin, end) in self.formula.0.chars().tuple_windows() {
            let search = [begin, end];
            if let Some(map) = final_rules.get(&search) {
                for (i, v) in map.iter().enumerate() {
                    counts[i] += v;
                }
            }
        }

        match counts.iter().filter(|v| **v > 0).minmax() {
            MinMaxResult::MinMax(a, b) => b - a,
            _ => 0,
        }
    }
}

impl TryFrom<Vec<String>> for Polymerizer {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut parts = value.into_iter();
        let formula: Formula = parts
            .next()
            .ok_or_else(|| anyhow!("Missing formula"))?
            .into();

        // blank line
        parts.next();

        let rules = Rules::try_from(parts.collect::<Vec<String>>())?;

        Ok(Self { formula, rules })
    }
}

#[cfg(test)]
mod tests {
    mod polymerizer {
        use crate::util::test_input;

        use super::super::*;

        #[test]
        fn process() {
            let input = test_input(
                "
                NNCB

                CH -> B
                HH -> N
                CB -> H
                NH -> C
                HB -> C
                HC -> B
                HN -> C
                NN -> C
                BH -> H
                NC -> B
                NB -> B
                BN -> B
                BB -> N
                BC -> B
                CC -> N
                CN -> C
                ",
            );

            let p = Polymerizer::try_from(input).expect("could not parse input");
            assert_eq!(p.iterations(10), 1588);
        }
    }
}

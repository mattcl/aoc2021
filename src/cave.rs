use std::convert::TryFrom;

use anyhow::{anyhow, Result};
use aoc_helpers::Solver;
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum CaveType {
    Big,
    Small,
    Start,
    End,
}

impl From<&str> for CaveType {
    fn from(value: &str) -> Self {
        if value == "start" {
            return Self::Start;
        }

        if value == "end" {
            return Self::End;
        }

        if value.chars().all(|ch| ch.is_uppercase()) {
            Self::Big
        } else {
            Self::Small
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cave {
    kind: CaveType,
    id: String,
    links: FxHashSet<usize>,
}

impl Cave {
    pub fn add_link(&mut self, other: usize) {
        self.links.insert(other);
    }
}

impl From<String> for Cave {
    fn from(value: String) -> Self {
        Self {
            kind: CaveType::from(value.as_str()),
            id: value,
            links: FxHashSet::default(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct CaveSystem {
    caves: Vec<Cave>,
}

impl CaveSystem {
    pub fn link(&mut self, a: usize, b: usize) -> Result<()> {
        self.caves
            .get_mut(a)
            .ok_or_else(|| anyhow!("cannot find cave {} for link", a))?
            .add_link(b);

        self.caves
            .get_mut(b)
            .ok_or_else(|| anyhow!("cannot find cave {} for link", b))?
            .add_link(a);

        Ok(())
    }

    /// So the problem, as written, doesn't actually need you to know what the
    /// paths are. We only really need to know *how many* there are to answer
    /// the question.
    pub fn paths_fast(&self, allow_multi_visit: bool) -> Result<usize> {
        // find the index of the start cave
        let start = self
            .caves
            .iter()
            .enumerate()
            .find_map(|cave| {
                if cave.1.kind == CaveType::Start {
                    Some(cave.0)
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow!("cave system does not have a start"))?;

        // find the index of the end cave
        let end = self
            .caves
            .iter()
            .enumerate()
            .find_map(|cave| {
                if cave.1.kind == CaveType::End {
                    Some(cave.0)
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow!("cave system does not have an end"))?;

        let mut seen = vec![0; self.caves.len()];
        self.recur_fast(start, end, !allow_multi_visit, &mut seen)
    }

    pub fn paths_semi_par(&self, allow_multi_visit: bool) -> Result<usize> {
        // find the index of the start cave
        let start = self
            .caves
            .iter()
            .find_map(|cave| {
                if cave.kind == CaveType::Start {
                    Some(cave)
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow!("cave system does not have a start"))?;

        // find the index of the end cave
        let end = self
            .caves
            .iter()
            .enumerate()
            .find_map(|cave| {
                if cave.1.kind == CaveType::End {
                    Some(cave.0)
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow!("cave system does not have an end"))?;

        let count = start
            .links
            .par_iter()
            .map(|ns| {
                let mut seen = vec![0; self.caves.len()];
                seen[*ns] = 1;
                self.recur_fast(*ns, end, !allow_multi_visit, &mut seen)
            })
            .collect::<Result<Vec<usize>>>()?
            .iter()
            .sum();
        Ok(count)
    }

    pub fn recur_fast(
        &self,
        start: usize,
        end: usize,
        allowance_used: bool,
        seen: &mut Vec<usize>,
    ) -> Result<usize> {
        if start == end {
            return Ok(1);
        }

        let cave = self.lookup(start)?;

        let mut count = 0;

        for i in cave.links.iter() {
            let i = *i;
            // otherwise
            let next = self.lookup(i)?;
            if next.kind == CaveType::Big || next.kind == CaveType::End {
                count += self.recur_fast(i, end, allowance_used, seen)?;
            } else if next.kind == CaveType::Small {
                if seen[i] > 0 {
                    // simulate allowing this or not
                    if !allowance_used {
                        count += self.recur_fast(i, end, true, seen)?;
                    }
                } else {
                    seen[i] += 1;
                    count += self.recur_fast(i, end, allowance_used, seen)?;
                    seen[i] -= 1;
                }
            }
        }

        Ok(count)
    }

    fn lookup(&self, idx: usize) -> Result<&Cave> {
        self.caves
            .get(idx)
            .ok_or_else(|| anyhow!("Unknown cave index: {}", idx))
    }
}

impl TryFrom<Vec<String>> for CaveSystem {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut seen: FxHashMap<String, usize> = FxHashMap::default();
        let mut cs = CaveSystem::default();

        for s in value {
            let mut parts = s.split('-');
            let a = Cave::from(
                parts
                    .next()
                    .ok_or_else(|| anyhow!("Invalid input, missing first part: {}", s))?
                    .to_string(),
            );
            let b = Cave::from(
                parts
                    .next()
                    .ok_or_else(|| anyhow!("Invalid input, missing second part: {}", s))?
                    .to_string(),
            );

            let a_idx = *seen.entry(a.id.clone()).or_insert_with(|| {
                cs.caves.push(a);
                cs.caves.len() - 1
            });

            let b_idx = *seen.entry(b.id.clone()).or_insert_with(|| {
                cs.caves.push(b);
                cs.caves.len() - 1
            });

            cs.link(a_idx, b_idx)?;
        }

        Ok(cs)
    }
}

impl Solver for CaveSystem {
    const ID: &'static str = "passage pathing";
    const DAY: usize = 12;

    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Self::P1 {
        self.paths_fast(false).expect("could not find paths")
    }

    fn part_two(&mut self) -> Self::P2 {
        self.paths_semi_par(true).expect("could not find paths")
    }
}

#[cfg(test)]
mod tests {
    mod cave_system {
        use aoc_helpers::util::test_input;

        use super::super::*;

        #[test]
        fn paths_that_visit_small_caves() {
            let input = test_input(
                "
                start-A
                start-b
                A-c
                A-b
                b-d
                A-end
                b-end
                ",
            );
            let cs = CaveSystem::try_from(input).expect("could not parse input");
            let paths = cs.paths_fast(false).expect("could not find paths");
            assert_eq!(paths, 10);

            let input = test_input(
                "
                dc-end
                HN-start
                start-kj
                dc-start
                dc-HN
                LN-dc
                HN-end
                kj-sa
                kj-HN
                kj-dc
                ",
            );
            let cs = CaveSystem::try_from(input).expect("could not parse input");
            let paths = cs.paths_fast(false).expect("could not find paths");
            assert_eq!(paths, 19);

            let input = test_input(
                "
                fs-end
                he-DX
                fs-he
                start-DX
                pj-DX
                end-zg
                zg-sl
                zg-pj
                pj-he
                RW-he
                fs-DX
                pj-RW
                zg-RW
                start-pj
                he-WI
                zg-he
                pj-fs
                start-RW
                ",
            );
            let cs = CaveSystem::try_from(input).expect("could not parse input");
            let paths = cs.paths_fast(false).expect("could not find paths");
            assert_eq!(paths, 226);
        }

        #[test]
        fn allowing_visiting_a_single_small_twice() {
            let input = test_input(
                "
                dc-end
                HN-start
                start-kj
                dc-start
                dc-HN
                LN-dc
                HN-end
                kj-sa
                kj-HN
                kj-dc
                ",
            );
            let cs = CaveSystem::try_from(input).expect("could not parse input");
            let paths = cs.paths_fast(true).expect("could not find paths");
            assert_eq!(paths, 103);

            let paths = cs.paths_semi_par(true).expect("could not find paths");
            assert_eq!(paths, 103);
        }
    }
}

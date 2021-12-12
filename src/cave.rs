use std::{convert::TryFrom, fmt};

use anyhow::{anyhow, Result};
use itertools::Itertools;
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
pub struct Path {
    caves: Vec<Cave>,
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.caves.iter().map(|c| c.id.clone()).join(","))
    }
}

impl From<Vec<&Cave>> for Path {
    fn from(value: Vec<&Cave>) -> Self {
        Path {
            caves: value.into_iter().cloned().collect(),
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

    pub fn paths(&self, allow_multi_visit: bool) -> Result<Vec<Path>> {
        // find the index of the start cave
        let (idx, start) = self
            .caves
            .iter()
            .enumerate()
            .find_map(|cave| {
                if cave.1.kind == CaveType::Start {
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
            .find_map(|cave| {
                if cave.kind == CaveType::End {
                    Some(cave)
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow!("cave system does not have an end"))?;

        let mut p = Vec::new();
        let mut seen = FxHashMap::default();
        seen.insert(idx, 3); // set to something higher so we don't repeat start
        let mut cur = vec![start];
        self.recur(start, end, !allow_multi_visit, &mut cur, &mut seen, &mut p)?;
        Ok(p)
    }

    pub fn recur<'a>(
        &'a self,
        start: &Cave,
        end: &Cave,
        allowance_used: bool,
        cur: &mut Vec<&'a Cave>,
        seen: &mut FxHashMap<usize, u8>,
        paths: &mut Vec<Path>,
    ) -> Result<()> {
        if start == end {
            paths.push(cur.clone().into());
            return Ok(());
        }

        for i in start.links.iter() {
            // since we won't ever insert big ones in here, this is fine
            if matches!(seen.get(i), Some(v) if (allowance_used && *v > 0) || *v > 1) {
                continue;
            }

            // otherwise
            let next = self.lookup(*i)?;
            let mut next_allowance = allowance_used;
            if next.kind != CaveType::Big {
                let e = seen.entry(*i).or_default();
                *e += 1;
                next_allowance = next_allowance || *e > 1;
            }
            cur.push(next);
            self.recur(next, end, next_allowance, cur, seen, paths)?;
            cur.pop();

            // This is safe, since the only way the and_modify triggers is if
            // we previously inserted
            seen.entry(*i).and_modify(|e| *e -= 1);
        }

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

        let mut seen = FxHashSet::default();
        let mut cache = FxHashMap::default();
        self.recur_fast(start, end, !allow_multi_visit, &mut seen, &mut cache)
    }

    pub fn recur_fast<'a>(
        &'a self,
        start: usize,
        end: usize,
        allowance_used: bool,
        seen: &mut FxHashSet<usize>,
        cache: &mut FxHashMap<(usize, usize), usize>,
    ) -> Result<usize> {
        if start == end {
            return Ok(1);
        }

        let cave = self.lookup(start)?;

        let mut count = 0;

        for i in cave.links.iter() {
            // otherwise
            let next = self.lookup(*i)?;
            if next.kind == CaveType::Big || next.kind == CaveType::End {
                count += self.recur_fast(*i, end, allowance_used, seen, cache)?;
            } else if next.kind == CaveType::Small {
                if seen.contains(i) {
                    // simulate allowing this or not
                    if allowance_used {
                        continue;
                    } else {
                        count += self.recur_fast(*i, end, true, seen, cache)?;
                    }
                } else {
                    if cave.links.len() < 3 {
                        if let Some(c) = cache.get(&(start, *i)) {
                            count += *c;
                            continue;
                        }
                    }

                    seen.insert(*i);
                    let c = self.recur_fast(*i, end, allowance_used, seen, cache)?;
                    count += c;
                    seen.remove(i);

                    if cave.links.len() < 3 {
                        cache.insert((start, *i), c);
                    }
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

#[cfg(test)]
mod tests {
    mod cave_system {
        use crate::util::test_input;

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
            let paths = cs.paths(false).expect("could not find paths");
            assert_eq!(paths.len(), 10);

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
            let paths = cs.paths(true).expect("could not find paths");
            assert_eq!(paths.len(), 103);

            let paths = cs.paths_fast(true).expect("could not find paths");
            assert_eq!(paths, 103);
        }
    }
}

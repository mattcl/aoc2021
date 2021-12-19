use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{convert::TryFrom, fmt, hash::Hash, iter::FromIterator, str::FromStr};

// I'm not smart enough to write something to generate this
pub const ROTATIONS: [([i64; 3], [usize; 3]); 24] = [
    ([1, 1, 1], [0, 1, 2]),
    ([1, -1, 1], [1, 0, 2]),
    ([-1, -1, 1], [0, 1, 2]),
    ([-1, 1, 1], [1, 0, 2]),
    ([1, 1, -1], [2, 1, 0]),
    ([1, -1, -1], [1, 2, 0]),
    ([-1, -1, -1], [2, 1, 0]),
    ([-1, 1, -1], [1, 2, 0]),
    ([1, -1, -1], [2, 0, 1]),
    ([-1, -1, -1], [0, 2, 1]),
    ([-1, 1, -1], [2, 0, 1]),
    ([1, 1, -1], [0, 2, 1]),
    ([1, -1, 1], [2, 1, 0]),
    ([-1, -1, 1], [1, 2, 0]),
    ([-1, 1, 1], [2, 1, 0]),
    ([1, 1, 1], [1, 2, 0]),
    ([1, 1, 1], [2, 0, 1]),
    ([1, -1, 1], [0, 2, 1]),
    ([-1, -1, 1], [2, 0, 1]),
    ([-1, 1, 1], [0, 2, 1]),
    ([-1, 1, -1], [0, 1, 2]),
    ([1, 1, -1], [1, 0, 2]),
    ([1, -1, -1], [0, 1, 2]),
    ([-1, -1, -1], [1, 0, 2]),
];

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub struct Beacon {
    coords: [i64; 3],
}

impl Beacon {
    pub fn dist_squared(&self, other: &Self) -> i64 {
        let dx = self.x() - other.x();
        let dy = self.y() - other.y();
        let dz = self.z() - other.z();

        dx * dx + dy * dy + dz * dz
    }

    pub fn manhattan(&self, other: &Self) -> i64 {
        (self.x() - other.x()).abs() + (self.y() - other.y()).abs() + (self.z() - other.z()).abs()
    }

    pub fn x(&self) -> i64 {
        self.coords[0]
    }

    pub fn y(&self) -> i64 {
        self.coords[1]
    }

    pub fn z(&self) -> i64 {
        self.coords[2]
    }

    pub fn offset(&self, other: &Self) -> Self {
        [
            self.coords[0] - other.coords[0],
            self.coords[1] - other.coords[1],
            self.coords[2] - other.coords[2],
        ]
        .into()
    }

    pub fn translate(&mut self, by: &[i64; 3]) {
        self.coords[0] += by[0];
        self.coords[1] += by[1];
        self.coords[2] += by[2];
    }

    pub fn rotate(&mut self, idx: usize) {
        self.coords = self.rotation(idx).coords;
    }

    pub fn rotation(&self, idx: usize) -> Self {
        let (signs, pos) = ROTATIONS[idx];
        [
            signs[0] * self.coords[pos[0]],
            signs[1] * self.coords[pos[1]],
            signs[2] * self.coords[pos[2]],
        ]
        .into()
    }

    pub fn rotations(&self) -> impl Iterator<Item = (usize, Self)> + '_ {
        ROTATIONS
            .iter()
            .enumerate()
            .map(move |(idx, (signs, pos))| {
                (
                    idx,
                    [
                        signs[0] * self.coords[pos[0]],
                        signs[1] * self.coords[pos[1]],
                        signs[2] * self.coords[pos[2]],
                    ]
                    .into(),
                )
            })
    }
}

impl fmt::Display for Beacon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.x(), self.y(), self.z())
    }
}

impl From<[i64; 3]> for Beacon {
    fn from(value: [i64; 3]) -> Self {
        Self { coords: value }
    }
}

impl FromStr for Beacon {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut iter = s.split(',').map(i64::from_str);

        let x = iter
            .next()
            .ok_or_else(|| anyhow!("cannot make beacon, missing x: {}", s))??;

        let y = iter
            .next()
            .ok_or_else(|| anyhow!("cannot make beacon, missing y: {}", s))??;

        let z = iter
            .next()
            .ok_or_else(|| anyhow!("cannot make beacon, missing z: {}", s))??;

        Ok([x, y, z].into())
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Measurement {
    dist: i64,
    instance: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Scanner {
    index: usize,
    beacons: Vec<Beacon>,
    /// A mapping between a beacon and its distances to other beacons in the
    /// scanner. So the idea is that the distances between any two beacons is
    /// constant regardless of what scanner reads them. Hopefully this lets me
    /// more quickly find the matching beacons. In this case, we're storing the
    /// square of the distance to avoid issues with representing these as ints
    /// instead of floats.
    dist_map: Vec<FxHashSet<Measurement>>,
    offset: Option<Beacon>,
}

impl Scanner {
    // So apparently, a threashold of 3 instead of 12 is good enough because of
    // the dataset, but let's just meet in the middle for whatever reason
    pub const THRESHOLD: usize = 6;

    pub fn new(index: usize, beacons: Vec<Beacon>) -> Self {
        let mut dist_map: Vec<FxHashSet<Measurement>> = vec![FxHashSet::default(); beacons.len()];

        let mut dist_pre_map: Vec<FxHashMap<i64, usize>> =
            vec![FxHashMap::default(); beacons.len()];

        for comb in beacons.iter().enumerate().combinations(2) {
            let a = comb[0];
            let b = comb[1];

            let dist = a.1.dist_squared(b.1);

            let e = dist_pre_map[a.0].entry(dist).or_default();
            *e += 1;

            let e = dist_pre_map[b.0].entry(dist).or_default();
            *e += 1;
        }

        for (idx, _) in beacons.iter().enumerate() {
            for (k, v) in dist_pre_map[idx].iter() {
                for i in 0..*v {
                    dist_map[idx].insert(Measurement {
                        dist: *k,
                        instance: i,
                    });
                }
            }
        }

        Self {
            index,
            beacons,
            dist_map,
            offset: None,
        }
    }

    pub fn transform(&mut self, rot: usize, trans: &[i64; 3]) {
        self.beacons.iter_mut().for_each(|b| {
            b.rotate(rot);
            b.translate(trans);
        });

        self.offset = Some(Beacon::from(*trans));
    }

    /// Returns a vector of a mapping between the index of a beacon in this
    /// scanner with the index of a beacon in the other scanner
    pub fn intersection<'a>(&self, other: &'a Self) -> Option<Vec<(&Beacon, &'a Beacon)>> {
        let mut candidates = Vec::new();
        let mut seen: FxHashSet<usize> = FxHashSet::default();

        for (idx, dists) in self.dist_map.iter().enumerate() {
            if let Some(found) = other.find_by_distances(dists) {
                if seen.contains(&found) {
                    // So I'm guessing the input has to ensure that this is
                    // unique, otherwise it'd be possible to incorrectly match
                    // something. That *could* be part of the problem, but let's
                    // just assume it isn't for now.
                    panic!("Well, shit..., there are multiple overlaps");
                }

                candidates.push((&self.beacons[idx], &other.beacons[found]));
                seen.insert(found);
            }

            // we can stop after we find enough
            if candidates.len() >= Self::THRESHOLD {
                return Some(candidates);
            }

            if candidates.len() + (self.beacons.len() - idx - 1) < Self::THRESHOLD {
                // we can't possibly satisfy this intersection, so break early
                return None;
            }
        }

        None
    }

    pub fn par_intersection<'a>(&self, other: &'a Self) -> Option<Vec<(&Beacon, &'a Beacon)>> {
        let res: Vec<_> = self
            .dist_map
            .par_iter()
            .enumerate()
            .filter_map(|(idx, dists)| {
                other
                    .find_by_distances(dists)
                    .map(|found| (&self.beacons[idx], &other.beacons[found]))
            })
            .collect();

        if res.len() < Self::THRESHOLD {
            return None;
        }

        Some(res)
    }

    pub fn find_by_distances(&self, distances: &FxHashSet<Measurement>) -> Option<usize> {
        for (idx, dists) in self.dist_map.iter().enumerate() {
            if distances.intersection(dists).count() >= Self::THRESHOLD - 1 {
                return Some(idx);
            }
        }

        None
    }

    pub fn par_find_by_distances(&self, distances: &FxHashSet<Measurement>) -> Option<usize> {
        self.dist_map
            .par_iter()
            .enumerate()
            .find_any(|(_, dists)| distances.intersection(dists).count() >= Self::THRESHOLD - 1)
            .map(|(idx, _)| idx)
    }

    pub fn get(&self, index: usize) -> Option<&Beacon> {
        self.beacons.get(index)
    }
}

impl TryFrom<&[String]> for Scanner {
    type Error = anyhow::Error;

    fn try_from(value: &[String]) -> Result<Self> {
        let mut parts = value.iter();
        let name_components = parts
            .next()
            .ok_or_else(|| anyhow!("missing scanner header"))?
            .split_whitespace()
            .collect::<Vec<&str>>();

        if name_components.len() < 4 {
            bail!("invalid scanner header: {}", value[0]);
        }

        let index = usize::from_str(name_components[2])?;

        let beacons = parts
            .map(|s| Beacon::from_str(s))
            .collect::<Result<Vec<Beacon>>>()?;

        Ok(Self::new(index, beacons))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Mapper {
    scanners: Vec<Scanner>,
}

impl Mapper {
    pub fn largest_distance(&self) -> Option<i64> {
        self.scanners
            .iter()
            .combinations(2)
            .map(|comb| {
                comb[0]
                    .offset
                    .unwrap_or_default()
                    .manhattan(&comb[1].offset.unwrap_or_default())
            })
            .max()
    }

    pub fn correlate(&mut self, beacons: &mut FxHashSet<Beacon>) {
        if self.scanners.is_empty() {
            return;
        }

        let mut solved: FxHashSet<usize> = FxHashSet::default();
        // we consider scanner 0 as the reference
        solved.insert(0);

        let mut pending: FxHashSet<usize> = FxHashSet::from_iter(1..self.scanners.len());

        // we can just go ahead and set these now
        for b in &self.scanners[0].beacons {
            beacons.insert(*b);
        }

        let mut already_checked: FxHashSet<(usize, usize)> = FxHashSet::default();

        loop {
            for r_idx in solved.clone().iter() {
                for p_idx in pending.clone().iter() {
                    let cache_key = (*r_idx.min(p_idx), *r_idx.max(p_idx));
                    if already_checked.contains(&cache_key) {
                        continue;
                    }

                    if let Some(intersection) =
                        self.scanners[*r_idx].par_intersection(&self.scanners[*p_idx])
                    {
                        if let Some((rot, offset)) = self.find_offset(&intersection) {
                            if let Some(s) = self.scanners.get_mut(*p_idx) {
                                s.transform(rot, &offset.coords);
                                for b in &s.beacons {
                                    beacons.insert(*b);
                                }
                                pending.remove(p_idx);
                                solved.insert(*p_idx);
                                break;
                            }
                        }
                    } else {
                        already_checked.insert(cache_key);
                    }
                }
            }

            if pending.is_empty() {
                break;
            }
        }
    }

    fn find_offset(&self, intersection: &[(&Beacon, &Beacon)]) -> Option<(usize, Beacon)> {
        for rot in 0..ROTATIONS.len() {
            if let Some(offset) = self.check_rotation(rot, intersection) {
                return Some((rot, offset));
            }
        }

        None
    }

    fn check_rotation(&self, rot: usize, intersection: &[(&Beacon, &Beacon)]) -> Option<Beacon> {
        let mut prev: Option<Beacon> = None;
        for (a, b) in intersection.iter().take(Scanner::THRESHOLD) {
            let delta = a.offset(&b.rotation(rot));
            if let Some(p) = prev {
                if delta != p {
                    // this rotation is invalid
                    return None;
                }
            } else {
                prev = Some(delta);
            }
        }

        prev
    }
}

impl TryFrom<Vec<String>> for Mapper {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let scanners = value
            .split(|s| s.is_empty())
            .map(Scanner::try_from)
            .collect::<Result<Vec<Scanner>>>()?;
        Ok(Self { scanners })
    }
}

#[cfg(test)]
mod tests {
    mod scanner {
        use crate::util::test_input;

        use super::super::*;

        #[test]
        fn construction() {
            let input = test_input(
                "
                --- scanner 0 ---
                -1,-1,1
                -2,-2,2
                -3,-3,3
                -2,-3,1
                5,6,-4
                8,0,7
                ",
            );
            Scanner::try_from(input.as_ref()).expect("could not parse scanner");
        }
    }

    mod mapping {
        use crate::util::test_input;

        use super::super::*;

        #[test]
        fn solution() {
            let input = test_input(
                "
                --- scanner 0 ---
                404,-588,-901
                528,-643,409
                -838,591,734
                390,-675,-793
                -537,-823,-458
                -485,-357,347
                -345,-311,381
                -661,-816,-575
                -876,649,763
                -618,-824,-621
                553,345,-567
                474,580,667
                -447,-329,318
                -584,868,-557
                544,-627,-890
                564,392,-477
                455,729,728
                -892,524,684
                -689,845,-530
                423,-701,434
                7,-33,-71
                630,319,-379
                443,580,662
                -789,900,-551
                459,-707,401

                --- scanner 1 ---
                686,422,578
                605,423,415
                515,917,-361
                -336,658,858
                95,138,22
                -476,619,847
                -340,-569,-846
                567,-361,727
                -460,603,-452
                669,-402,600
                729,430,532
                -500,-761,534
                -322,571,750
                -466,-666,-811
                -429,-592,574
                -355,545,-477
                703,-491,-529
                -328,-685,520
                413,935,-424
                -391,539,-444
                586,-435,557
                -364,-763,-893
                807,-499,-711
                755,-354,-619
                553,889,-390

                --- scanner 2 ---
                649,640,665
                682,-795,504
                -784,533,-524
                -644,584,-595
                -588,-843,648
                -30,6,44
                -674,560,763
                500,723,-460
                609,671,-379
                -555,-800,653
                -675,-892,-343
                697,-426,-610
                578,704,681
                493,664,-388
                -671,-858,530
                -667,343,800
                571,-461,-707
                -138,-166,112
                -889,563,-600
                646,-828,498
                640,759,510
                -630,509,768
                -681,-892,-333
                673,-379,-804
                -742,-814,-386
                577,-820,562

                --- scanner 3 ---
                -589,542,597
                605,-692,669
                -500,565,-823
                -660,373,557
                -458,-679,-417
                -488,449,543
                -626,468,-788
                338,-750,-386
                528,-832,-391
                562,-778,733
                -938,-730,414
                543,643,-506
                -524,371,-870
                407,773,750
                -104,29,83
                378,-903,-323
                -778,-728,485
                426,699,580
                -438,-605,-362
                -469,-447,-387
                509,732,623
                647,635,-688
                -868,-804,481
                614,-800,639
                595,780,-596

                --- scanner 4 ---
                727,592,562
                -293,-554,779
                441,611,-461
                -714,465,-776
                -743,427,-804
                -660,-479,-426
                832,-632,460
                927,-485,-438
                408,393,-506
                466,436,-512
                110,16,151
                -258,-428,682
                -393,719,612
                -211,-452,876
                808,-476,-593
                -575,615,604
                -485,667,467
                -680,325,-822
                -627,-443,-432
                872,-547,-609
                833,512,582
                807,604,487
                839,-516,451
                891,-625,532
                -652,-548,-490
                30,-46,-14
                ",
            );
            let mut m = Mapper::try_from(input).expect("could not parse input");
            let mut beacons = FxHashSet::default();
            m.correlate(&mut beacons);
            assert_eq!(beacons.len(), 79);
            assert_eq!(m.largest_distance(), Some(3621));
        }
    }
}

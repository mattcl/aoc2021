use anyhow::{anyhow, bail, Result};
use aoc_helpers::Solver;
use rustc_hash::FxHashMap;
use std::{
    collections::BinaryHeap,
    convert::TryFrom,
    fmt,
    hash::{Hash, Hasher},
    // iter::FromIterator,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AmphipodType {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl AmphipodType {
    pub fn energy_per_step(&self) -> usize {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }

    pub fn desired_room(&self) -> usize {
        match self {
            Self::Amber => 0,
            Self::Bronze => 1,
            Self::Copper => 2,
            Self::Desert => 3,
        }
    }

    pub fn desired_room_entrance(&self) -> usize {
        match self {
            Self::Amber => 2,
            Self::Bronze => 4,
            Self::Copper => 6,
            Self::Desert => 8,
        }
    }
}

impl TryFrom<char> for AmphipodType {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        Ok(match value {
            'A' => Self::Amber,
            'B' => Self::Bronze,
            'C' => Self::Copper,
            'D' => Self::Desert,
            _ => bail!("cannot create amphipod type from '{}'", value),
        })
    }
}

impl fmt::Display for AmphipodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Self::Amber => 'A',
            Self::Bronze => 'B',
            Self::Copper => 'C',
            Self::Desert => 'D',
        };
        write!(f, "{}", ch)
    }
}

pub const EMPTY: char = ' ';

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Room<const N: usize> {
    desired: char,
    capacity: usize,
    state: [char; N],
}

impl<const N: usize> Hash for Room<N> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.state.hash(state);
    }
}

impl<const N: usize> Room<N> {
    pub fn new(desired: char) -> Self {
        Self {
            desired,
            capacity: N,
            state: [' '; N],
        }
    }

    pub fn empty(&self) -> bool {
        self.capacity == N
    }

    pub fn full(&self) -> bool {
        self.capacity == 0
    }

    pub fn accepting_desired(&self) -> bool {
        !self.full()
            && self
                .state
                .iter()
                .all(|ch| *ch == EMPTY || *ch == self.desired)
    }

    pub fn complete(&self) -> bool {
        self.state.iter().all(|v| *v == self.desired)
    }

    pub fn push_distance(&self) -> usize {
        self.capacity
    }

    pub fn push(&mut self, v: char) -> bool {
        if self.full() {
            return false;
        }

        self.capacity -= 1;

        self.state[self.capacity] = v;
        true
    }

    pub fn pop(&mut self) -> char {
        let v = self.state[self.capacity];
        self.state[self.capacity] = EMPTY;
        self.capacity += 1;
        v
    }

    pub fn peek(&self) -> char {
        if self.empty() {
            return 'X';
        }
        self.state[self.capacity]
    }

    pub fn valid_hall_moves<'a>(&self, hall: &'a Hall) -> impl Iterator<Item = (char, usize)> + 'a {
        let ch = self.peek();
        let empty = self.empty();
        let complete = self.complete();
        let accepting_desired = self.accepting_desired();
        // this unwrap is "safe" in the sense that rooms should not be made with
        // incorrect desired values
        let kind = AmphipodType::try_from(self.desired).unwrap();
        let desired_room_entrance = kind.desired_room_entrance();
        Hall::VALID_WAITING_POSITIONS
            .iter()
            .filter(move |p| hall.state[**p] == EMPTY)
            .filter_map(move |hall_pos| {
                if !empty && !complete && !accepting_desired {
                    hall.can_move_between(desired_room_entrance, *hall_pos)
                        .then(|| (ch, *hall_pos))
                } else {
                    None
                }
            })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Hall {
    state: [char; 11],
}

impl Default for Hall {
    fn default() -> Self {
        Self { state: [EMPTY; 11] }
    }
}

impl Hall {
    pub const VALID_WAITING_POSITIONS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

    pub fn set(&mut self, pos: usize, val: char) {
        self.state[pos] = val;
    }

    pub fn unset(&mut self, pos: usize) {
        self.state[pos] = EMPTY;
    }

    pub fn can_move_between(&self, start: usize, end: usize) -> bool {
        let s = start.min(end);
        let e = start.max(end);

        // is there no one in our way between here and there?
        (s..=e).all(|spot| self.state[spot] == EMPTY)
    }

    pub fn occupants(&self) -> impl Iterator<Item = (usize, &char)> {
        self.state.iter().enumerate().filter(|(_, s)| **s != EMPTY)
    }

    pub fn moveable<'a, const N: usize>(
        &'a self,
        rooms: &'a [Room<N>],
    ) -> impl Iterator<Item = (usize, &'a char, AmphipodType, usize)> + 'a {
        self.occupants().filter_map(move |(pos, ch)| {
            if let Ok(kind) = AmphipodType::try_from(*ch) {
                // can we even move to the room?
                let room = rooms[kind.desired_room()];
                if room.accepting_desired() {
                    let desired_room_entrance = kind.desired_room_entrance();
                    let (start, end) = if desired_room_entrance < pos {
                        (desired_room_entrance, pos - 1)
                    } else {
                        (pos + 1, desired_room_entrance)
                    };

                    if self.can_move_between(start, end) {
                        let dist = end - start + 1;
                        return Some((pos, ch, kind, dist + room.push_distance()));
                    }
                }
            }

            None
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node<const N: usize> {
    state: Burrow<N>,
    cost: usize,
    f: usize,
}

impl<const N: usize> Node<N> {
    pub fn new(state: Burrow<N>, cost: usize, f: usize) -> Self {
        Self { state, cost, f }
    }
}

impl<const N: usize> Ord for Node<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f.cmp(&self.f)
    }
}

impl<const N: usize> PartialOrd for Node<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Burrow<const N: usize> {
    hall: Hall,
    rooms: [Room<N>; 4],
}

impl<const N: usize> Burrow<N> {
    // pub fn key(&self) -> String {
    //     String::from_iter(
    //         self.hall.state.
    //             iter()
    //             .chain(self.rooms[0].state.iter())
    //             .chain(self.rooms[1].state.iter())
    //             .chain(self.rooms[2].state.iter())
    //             .chain(self.rooms[3].state.iter()))
    // }

    pub fn key(&self) -> u64 {
        self.hall.state.
            iter()
            .chain(self.rooms[0].state.iter())
            .chain(self.rooms[1].state.iter())
            .chain(self.rooms[2].state.iter())
            .chain(self.rooms[3].state.iter())
            .fold(0, |acc, ch| {
                acc * 10 + (ch.to_digit(16).unwrap_or_default() as u64)
            })
    }

    pub fn complete(&self) -> bool {
        self.rooms.iter().all(|r| r.complete())
    }

    pub fn minimize(&self) -> Option<usize> {
        let mut lowest: FxHashMap<u64, usize> = FxHashMap::default();
        lowest.insert(self.key(), 0);
        let mut heap = BinaryHeap::new();
        heap.push(Node::new(*self, 0, 0));

        while let Some(cur) = heap.pop() {
            if cur.state.complete() {
                return Some(cur.cost);
            }

            // while this seems fine, the cache lookup performance is just way
            // too slow because it has to be hashed instead of direct index
            // if cur.cost > *lowest.get(&cur.state.key()).unwrap_or(&usize::MAX) {
            //     continue;
            // }

            // if we can move directly, this is the thing with the lowest cost
            let mut any_direct = false;
            for (room_idx, room) in cur.state.rooms.iter().enumerate() {
                if !room.empty() && !room.accepting_desired() {
                    let ch = room.peek();
                    let kind = AmphipodType::try_from(ch).unwrap();
                    let desired = cur.state.rooms[kind.desired_room()];

                    if desired.accepting_desired() {
                        let origin_kind = AmphipodType::try_from(room.desired).unwrap();
                        let origin_entrance = origin_kind.desired_room_entrance();
                        let desired_room_entrance = kind.desired_room_entrance();

                        if cur.state.hall.can_move_between(origin_entrance, desired_room_entrance) {
                            any_direct = true;
                            let mut new_state = cur.state;
                            new_state.rooms[room_idx].pop();
                            new_state.rooms[kind.desired_room()].push(ch);
                            let entrance_dist = (origin_entrance as i64 - desired_room_entrance as i64).abs() + 1;
                            let dist = room.push_distance() + desired.push_distance() +  entrance_dist as usize;
                            let cost = cur.cost + dist * kind.energy_per_step();
                            let new_node = Node::new(new_state, cost, cost);

                            lowest
                                .entry(new_node.state.key())
                                .and_modify(|e| {
                                    if new_node.cost < *e {
                                        *e = new_node.cost;
                                        heap.push(new_node.clone());
                                    }
                                })
                                .or_insert_with(|| {
                                    let cost = new_node.cost;
                                    heap.push(new_node);
                                    cost
                                });
                        }
                    }
                }
            }

            // these are optimal, so don't bother checking anything else (they
            // would seem sub-optimal compared to the halway movements or some
            // of the room -> hallway moves
            if any_direct {
                continue;
            }

            // find a list of all the new game states
            // for all items in the hall, attempt to move them to accepting rooms
            for (pos, ch, kind, dist) in cur.state.hall.moveable(&cur.state.rooms) {
                // copies
                let mut new_state = cur.state;
                new_state.rooms[kind.desired_room()].push(*ch);
                new_state.hall.unset(pos);
                let cost = cur.cost + dist * kind.energy_per_step();
                let new_node = Node::new(new_state, cost, cost);

                lowest
                    .entry(new_node.state.key())
                    .and_modify(|e| {
                        if new_node.cost < *e {
                            *e = new_node.cost;
                            heap.push(new_node.clone());
                        }
                    })
                    .or_insert_with(|| {
                        let cost = new_node.cost;
                        heap.push(new_node);
                        cost
                    });
            }

            // for all items in rooms where they don't belong
            for (room_idx, room) in cur.state.rooms.iter().enumerate() {
                let room_kind = AmphipodType::try_from(room.desired).unwrap();
                if room.complete() {
                    continue;
                }

                for (ch, pos) in room.valid_hall_moves(&cur.state.hall) {
                    let mut new_state = cur.state;
                    let kind = AmphipodType::try_from(ch).unwrap();
                    let dist = room.push_distance()
                        + 1
                        + (room_kind.desired_room_entrance() as i32 - pos as i32).abs() as usize;
                    new_state.rooms[room_idx].pop();
                    new_state.hall.set(pos, ch);
                    let cost = cur.cost + dist * kind.energy_per_step();
                    // let h = (pos as i32 - kind.desired_room_entrance() as i32).abs() as usize
                    //     + new_state.rooms[kind.desired_room()].push_distance();
                    let new_node =
                        // Node::new(new_state, cost, cost + (dist + h) * kind.energy_per_step());
                        Node::new(new_state, cost, cost);

                    lowest
                        .entry(new_node.state.key())
                        .and_modify(|e| {
                            if new_node.cost < *e {
                                *e = new_node.cost;
                                heap.push(new_node.clone());
                            }
                        })
                        .or_insert_with(|| {
                            let cost = new_node.cost;
                            heap.push(new_node);
                            cost
                        });
                }
            }
        }

        None
    }
}

impl<const N: usize> Default for Burrow<N> {
    fn default() -> Self {
        Self {
            hall: Hall::default(),
            rooms: [
                Room::<N>::new('A'),
                Room::<N>::new('B'),
                Room::<N>::new('C'),
                Room::<N>::new('D'),
            ],
        }
    }
}

pub type SmallBurrow = Burrow<2>;

impl TryFrom<&Vec<String>> for SmallBurrow {
    type Error = anyhow::Error;

    fn try_from(value: &Vec<String>) -> Result<Self> {
        // so the parsing is dumb
        let mut burrow = SmallBurrow::default();
        let chars = value
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let c_offset = 1;
        let rows = [3_usize, 2];

        for room in burrow.rooms.iter_mut() {
            let room_kind = AmphipodType::try_from(room.desired).unwrap();
            let c_idx = c_offset + room_kind.desired_room_entrance();
            for row in rows.iter() {
                room.push(
                    *chars.get(*row).and_then(|r| r.get(c_idx)).ok_or_else(|| {
                        anyhow!("invalid input, could not find {}, {}", row, c_idx)
                    })?,
                );
            }
        }

        Ok(burrow)
    }
}

impl fmt::Display for SmallBurrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "#############
#{}#
###{}#{}#{}#{}###
  #{}#{}#{}#{}#
  #########",
            self.hall.state.iter().collect::<String>(),
            self.rooms[0].state[0],
            self.rooms[1].state[0],
            self.rooms[2].state[0],
            self.rooms[3].state[0],
            self.rooms[0].state[1],
            self.rooms[1].state[1],
            self.rooms[2].state[1],
            self.rooms[3].state[1],
        )
    }
}

pub type LargeBurrow = Burrow<4>;

impl TryFrom<&Vec<String>> for LargeBurrow {
    type Error = anyhow::Error;

    fn try_from(value: &Vec<String>) -> Result<Self> {
        // so the parsing is dumb
        let mut burrow = LargeBurrow::default();
        let chars = value
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let c_offset = 1;
        let rows = [3_usize, 2];
        let padding = [['D', 'D'], ['B', 'C'], ['A', 'B'], ['C', 'A']];

        for (room_idx, room) in burrow.rooms.iter_mut().enumerate() {
            let room_kind = AmphipodType::try_from(room.desired).unwrap();
            let c_idx = c_offset + room_kind.desired_room_entrance();
            for (idx, row) in rows.iter().enumerate() {
                room.push(
                    *chars.get(*row).and_then(|r| r.get(c_idx)).ok_or_else(|| {
                        anyhow!("invalid input, could not find {}, {}", row, c_idx)
                    })?,
                );
                if idx == 0 {
                    for p in padding[room_idx].iter() {
                        room.push(*p);
                    }
                }
            }
        }

        Ok(burrow)
    }
}

pub struct Amphipod {
    small: SmallBurrow,
    large: LargeBurrow,
}

impl TryFrom<Vec<String>> for Amphipod {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let small = SmallBurrow::try_from(&value)?;
        let large = LargeBurrow::try_from(&value)?;

        Ok(Self {small, large})
    }
}

impl Solver for Amphipod {
    const ID: &'static str = "amphipod";
    const DAY: usize = 23;

    type P1 = usize;
    type P2 = usize;


    fn part_one(&mut self) -> <Self as aoc_helpers::Solver>::P1 {
        self.small.minimize().expect("could not solve part 1")
    }

    fn part_two(&mut self) -> <Self as aoc_helpers::Solver>::P1 {
        self.large.minimize().expect("could not solve part 1")
    }
}

#[cfg(test)]
mod tests {
    use aoc_helpers::util::test_input;

    use super::*;

    #[test]
    fn rooms() {
        let mut room = Room::<2>::new('A');
        assert!(room.empty());
        assert!(room.accepting_desired());
        assert!(!room.complete());
        assert_eq!(room.push_distance(), 2);

        room.push('A');
        assert!(!room.empty());
        assert!(room.accepting_desired());
        assert!(!room.complete());
        assert_eq!(room.push_distance(), 1);
        assert_eq!(room.state, [' ', 'A']);

        room.push('A');
        assert!(!room.empty());
        assert!(!room.accepting_desired());
        assert!(room.complete());
        assert_eq!(room.push_distance(), 0);
        assert_eq!(room.state, ['A', 'A']);

        let r = room.pop();
        assert_eq!(r, 'A');
        assert!(!room.empty());
        assert!(room.accepting_desired());
        assert!(!room.complete());
        assert_eq!(room.push_distance(), 1);
        assert_eq!(room.state, [' ', 'A']);

        room.push('B');
        assert!(!room.empty());
        assert!(!room.accepting_desired());
        assert!(!room.complete());
        assert_eq!(room.push_distance(), 0);
        assert_eq!(room.state, ['B', 'A']);
    }

    #[test]
    fn halls() {
        let mut hall = Hall::default();
        assert!(hall.can_move_between(0, 1));
        assert!(hall.can_move_between(1, 0));

        hall.set(1, 'A');
        assert!(!hall.can_move_between(0, 1));
        assert!(!hall.can_move_between(1, 0));
    }

    #[test]
    fn small_example() {
        // i have to pad a little since my load input function strips lines
        let input = test_input(
            "
            #############
            #...........#
            ###B#C#B#D###
            ###A#D#C#A#
            ###########
            ",
        );
        let burrow = SmallBurrow::try_from(&input).expect("could not parse input");
        let cost = burrow.minimize().expect("could not solve");
        assert_eq!(cost, 12521)
    }

    #[test]
    #[ignore]
    fn large_example() {
        // i have to pad a little since my load input function strips lines
        let input = test_input(
            "
            #############
            #...........#
            ###B#C#B#D###
            ###A#D#C#A#
            ###########
            ",
        );
        let burrow = LargeBurrow::try_from(&input).expect("could not parse input");
        let cost = burrow.minimize().expect("could not solve");
        assert_eq!(cost, 44169)
    }
}

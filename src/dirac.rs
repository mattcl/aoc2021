use anyhow::{anyhow, bail, Result};
use aoc_helpers::Solver;
use rustc_hash::FxHashMap;
use std::{convert::TryFrom, str::FromStr};

pub const BOARD_MAX: usize = 10;
// [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
// [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Player {
    score: usize,
    pos: usize,
}

impl Player {
    pub fn turn(&mut self, move_dist: usize) -> usize {
        self.pos = (self.pos + move_dist) % BOARD_MAX;
        self.score += self.pos + 1;
        self.score
    }

    pub fn pretend(&self, move_dist: usize) -> Self {
        let mut new = *self;
        new.turn(move_dist);
        new
    }
}

impl FromStr for Player {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let v = s
            .split(": ")
            .last()
            .ok_or_else(|| anyhow!("cannot parse player from: {}", s))?;

        Ok(Player {
            pos: usize::from_str(v)? - 1_usize,
            score: 0,
        })
    }
}

pub trait Die: Iterator<Item = usize> + Default {
    fn rolls(&self) -> usize;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeterministicDie {
    cur: usize,
    max: usize,
    rolls: usize,
}

impl Default for DeterministicDie {
    fn default() -> Self {
        Self {
            cur: 1,
            max: 100,
            rolls: 0,
        }
    }
}

impl Iterator for DeterministicDie {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.rolls += 1;

        if self.cur + 2 > self.max {
            let c = self.cur - 1;
            let d = self.cur + ((c + 1) % self.max) + ((c + 2) % self.max) + 2;
            self.cur = (self.cur + 3) % self.max;
            return Some(d);
        }

        let d = self.cur * 3 + 3;
        self.cur += 3;

        if self.cur > self.max {
            self.cur = 1;
        }

        Some(d)
    }
}

impl Die for DeterministicDie {
    fn rolls(&self) -> usize {
        self.rolls
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Game<T>
where
    T: Die,
{
    die: T,
    players: Vec<Player>,
}

impl<T> Game<T>
where
    T: Die,
{
    pub fn play(&mut self) -> Result<usize> {
        for player in (0..self.players.len()).cycle() {
            let roll = self
                .die
                .next()
                .ok_or_else(|| anyhow!("Die did not produce a value!"))?;
            let score = self.players[player].turn(roll);

            if score >= 1000 {
                return Ok(self.players[(player + 1) % self.players.len()].score
                    * self.die.rolls()
                    * 3);
            }
        }

        unreachable!("The cycle should prevent ever getting here");
    }
}

impl<T> TryFrom<&[String]> for Game<T>
where
    T: Die,
{
    type Error = anyhow::Error;

    fn try_from(value: &[String]) -> Result<Self> {
        let players = value
            .iter()
            .map(|s| Player::from_str(s))
            .collect::<Result<Vec<Player>>>()?;
        Ok(Game {
            players,
            ..Game::default()
        })
    }
}

/// So I'm really bummed my part 1 gamble didn't pay off here and I have to
/// implement this struct
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct QuantumGame {
    turn: usize,
    players: [Player; 2],
}

// 1                  2
// 1     2     3
// 1 2 3 1 2 3 1 2 3

// 1,3
// 2,4
// 3,5,
// 2,6,
// 1,7
impl QuantumGame {
    pub const TARGET: usize = 21;
    // (frequncy of value, value)
    pub const ROLL_VALUES: [(usize, usize); 7] =
        [(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)];

    pub fn play(&self) -> usize {
        let mut cache = FxHashMap::default();
        let wins = self.take_turn(&mut cache);
        wins[0].max(wins[1])
    }

    pub fn take_turn(&self, cache: &mut FxHashMap<Self, [usize; 2]>) -> [usize; 2] {
        if let Some(wins) = cache.get(self) {
            return *wins;
        }

        let idx = self.turn % 2;

        let mut wins = [0_usize, 0_usize];
        for (freq, value) in QuantumGame::ROLL_VALUES.iter() {
            let mut new_game = *self;
            let score = new_game.players[idx].turn(*value);
            if score >= QuantumGame::TARGET {
                wins[idx] += freq;
            } else {
                new_game.turn = (new_game.turn + 1) % 2;
                let res = new_game.take_turn(cache);
                wins[0] += res[0] * freq;
                wins[1] += res[1] * freq;
            }
        }

        cache.insert(*self, wins);

        wins
    }
}

impl TryFrom<&[String]> for QuantumGame {
    type Error = anyhow::Error;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        let players = value
            .iter()
            .map(|s| Player::from_str(s))
            .collect::<Result<Vec<Player>>>()?;
        if players.len() != 2 {
            bail!("Wrong number of players: {}", players.len());
        }

        Ok(Self {
            players: [players[0], players[1]],
            ..QuantumGame::default()
        })
    }
}

#[derive(Debug, Clone)]
pub struct Games {
    deterministic: Game<DeterministicDie>,
    quantum: QuantumGame,
}

impl TryFrom<Vec<String>> for Games {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        Ok(Self {
            deterministic: Game::try_from(value.as_ref())?,
            quantum: QuantumGame::try_from(value.as_ref())?,
        })
    }
}

impl Solver for Games {
    const ID: &'static str = "dirac dice";
    const DAY: usize = 21;

    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Self::P1 {
        let mut g = self.deterministic.clone();
        g.play().expect("unable to play game")
    }

    fn part_two(&mut self) -> Self::P2 {
        self.quantum.play()
    }
}

#[cfg(test)]
mod tests {
    use aoc_helpers::util::test_input;

    use super::*;

    #[test]
    fn deterministic() {
        let input = test_input(
            "
            Player 1 starting position: 4
            Player 2 starting position: 8
            ",
        );
        let mut game: Game<DeterministicDie> =
            Game::try_from(input.as_ref()).expect("could not parse game");
        assert_eq!(game.play().expect("unexpected failure"), 739785);
    }

    #[test]
    fn quantum() {
        let input = test_input(
            "
            Player 1 starting position: 4
            Player 2 starting position: 8
            ",
        );
        let game = QuantumGame::try_from(input.as_ref()).expect("could not parse game");
        assert_eq!(game.play(), 444356092776315);
    }
}

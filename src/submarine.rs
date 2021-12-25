use anyhow::{anyhow, bail, Result};
use aoc_helpers::{parse_input, Solver};
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(' ');
        let name = parts
            .next()
            .ok_or_else(|| anyhow!("Missing command name in '{}'", s))?;
        let value: i64 = parts
            .next()
            .ok_or_else(|| anyhow!("Missing command value in '{}'", s))?
            .parse()?;

        match name {
            "forward" => Ok(Command::Forward(value)),
            "down" => Ok(Command::Down(value)),
            "up" => Ok(Command::Up(value)),
            _ => bail!("Unknown command {}", name),
        }
    }
}

// This ended up being unnecessary as of day 2, but I was thinking that maybe
// they'd introduce another dimension
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Position(i64);

impl_op_ex!(+=|a: &mut Position, b: &i64| {
    a.0 += b
});

impl_op_ex_commutative!(+|a: &Position, b: &i64| -> Position {
    Position(a.0 + b)
});

impl_op_ex!(+|a: &Position, b: &Position| -> Position {
    Position(a.0 + b.0)
});

pub trait Moveable {
    fn execute(&mut self, cmd: &Command);
    fn location_hash(&self) -> i64;
}

#[derive(Debug, Clone, Default)]
pub struct Submarine {
    pos: Position,
    depth: i64,
}

impl Submarine {
    pub fn new() -> Self {
        Submarine::default()
    }
}

impl Moveable for Submarine {
    fn execute(&mut self, cmd: &Command) {
        match cmd {
            Command::Forward(dist) => self.pos += dist,
            Command::Down(dist) => self.depth += dist,
            Command::Up(dist) => self.depth -= dist,
        }
    }

    fn location_hash(&self) -> i64 {
        self.depth * self.pos.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct AimableSubmarine {
    pos: Position,
    aim: i64,
    depth: i64,
}

impl AimableSubmarine {
    pub fn new() -> Self {
        AimableSubmarine::default()
    }
}

impl Moveable for AimableSubmarine {
    fn execute(&mut self, cmd: &Command) {
        match cmd {
            Command::Forward(dist) => {
                self.pos += dist;
                self.depth += self.aim * dist;
            }
            Command::Down(dist) => self.aim += dist,
            Command::Up(dist) => self.aim -= dist,
        }
    }

    fn location_hash(&self) -> i64 {
        self.depth * self.pos.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct Subs {
    normal: Submarine,
    aimable: AimableSubmarine,
    commands: Vec<Command>,
}

impl TryFrom<Vec<String>> for Subs {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let commands = parse_input(&value)?;

        Ok(Self {
            commands,
            ..Self::default()
        })
    }
}

impl Solver for Subs {
    const ID: &'static str = "dive";
    const DAY: usize = 2;

    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Self::P1 {
        for command in self.commands.iter() {
            self.normal.execute(command);
        }

        self.normal.location_hash()
    }

    fn part_two(&mut self) -> Self::P2 {
        for command in self.commands.iter() {
            self.aimable.execute(command);
        }

        self.aimable.location_hash()
    }
}

#[cfg(test)]
mod tests {
    mod pos {
        use super::super::*;

        #[test]
        fn basic_addition() {
            let p = Position(10);
            assert_eq!(p + 11, Position(21));
            assert_eq!(p + &11, Position(21));
            assert_eq!(p + Position(11), Position(21));
        }
    }

    mod submarine {
        use super::super::*;
        use aoc_helpers::util::{parse_input, test_input};

        #[test]
        fn movement() {
            let input = test_input(
                "
                forward 5
                down 5
                forward 8
                up 3
                down 8
                forward 2
            ",
            );
            let commands: Vec<Command> = parse_input(&input).expect("Could not parse input");
            let mut sub = Submarine::new();

            for command in &commands {
                sub.execute(command);
            }

            assert_eq!(sub.location_hash(), 150);
        }
    }

    mod aimable_submarine {
        use super::super::*;
        use aoc_helpers::util::{parse_input, test_input};

        #[test]
        fn movement() {
            let input = test_input(
                "
                forward 5
                down 5
                forward 8
                up 3
                down 8
                forward 2
            ",
            );
            let commands: Vec<Command> = parse_input(&input).expect("Could not parse input");
            let mut sub = AimableSubmarine::new();

            for command in &commands {
                sub.execute(command);
            }

            assert_eq!(sub.location_hash(), 900);
        }
    }
}

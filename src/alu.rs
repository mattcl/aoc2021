use std::str::FromStr;
use std::{convert::TryFrom, ops::Deref};

use anyhow::{anyhow, bail, Result};
use aoc_helpers::Solver;
use itertools::Itertools;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Val {
    VarW,
    VarX,
    VarY,
    VarZ,
    Raw(i64),
}

impl Val {
    pub fn var_index(&self) -> Result<usize> {
        Ok(match self {
            Self::VarW => 2,
            Self::VarX => 0,
            Self::VarY => 1,
            Self::VarZ => 3,
            _ => bail!("cannot get a vari index for a raw value: {:?}", self),
        })
    }
}

impl FromStr for Val {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "w" => Val::VarW,
            "x" => Val::VarX,
            "y" => Val::VarY,
            "z" => Val::VarZ,
            _ => Val::Raw(i64::from_str(s)?),
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum OpCode {
    RW(Val),
    Add(Val, Val),
    Mul(Val, Val),
    Div(Val, Val),
    Rem(Val, Val),
    Eq(Val, Val),
}

impl OpCode {
    pub fn execute(&self, input: i64, output: &mut Output) -> Result<()> {
        match self {
            Self::RW(val) => output.set(val, input),
            Self::Add(v1, v2) => output.set(v1, output.get(v1) + output.get(v2)),
            Self::Mul(v1, v2) => output.set(v1, output.get(v1) * output.get(v2)),
            Self::Div(v1, v2) => output.set(v1, output.get(v1) / output.get(v2)),
            Self::Rem(v1, v2) => output.set(v1, output.get(v1) % output.get(v2)),
            Self::Eq(v1, v2) => output.set(
                v1,
                if output.get(v1) == output.get(v2) {
                    1
                } else {
                    0
                },
            ),
        }
    }
}

impl FromStr for OpCode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split_whitespace().collect::<Vec<_>>();

        Ok(match parts.as_slice() {
            ["inp", x] => Self::RW(Val::from_str(x)?),
            ["add", x, y] => Self::Add(Val::from_str(x)?, Val::from_str(y)?),
            ["mul", x, y] => Self::Mul(Val::from_str(x)?, Val::from_str(y)?),
            ["div", x, y] => Self::Div(Val::from_str(x)?, Val::from_str(y)?),
            ["mod", x, y] => Self::Rem(Val::from_str(x)?, Val::from_str(y)?),
            ["eql", x, y] => Self::Eq(Val::from_str(x)?, Val::from_str(y)?),
            _ => bail!("unknown operation: {}", s),
        })
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Program(pub Vec<OpCode>);

impl Deref for Program {
    type Target = Vec<OpCode>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&Vec<String>> for Program {
    type Error = anyhow::Error;

    fn try_from(value: &Vec<String>) -> Result<Self> {
        let instructions = value
            .iter()
            .map(|v| OpCode::from_str(v))
            .collect::<Result<Vec<OpCode>>>()?;

        Ok(Self(instructions))
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct Input {
    values: Vec<i64>,
    pos: usize,
}

impl Input {
    pub fn new(value: i64) -> Self {
        let mut values = Vec::with_capacity(14);
        let mut start = value;

        for _ in 0..14 {
            values.push(start % 10);
            start /= 10;

            if start == 0 {
                break;
            }
        }

        values.reverse();

        Self { values, pos: 0 }
    }
    pub fn next(&mut self) -> Option<i64> {
        let out = self.values.get(self.pos).cloned();
        self.pos += 1;
        out
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Output {
    variables: [i64; 4],
}

impl Output {
    pub fn set(&mut self, val: &Val, value: i64) -> Result<()> {
        self.variables[val.var_index()?] = value;
        Ok(())
    }

    pub fn get(&self, val: &Val) -> i64 {
        if let Val::Raw(v) = val {
            return *v;
        }
        // we know this is safe now
        self.variables[val.var_index().unwrap()]
    }

    pub fn x(&self) -> i64 {
        self.get(&Val::VarX)
    }

    pub fn y(&self) -> i64 {
        self.get(&Val::VarY)
    }

    pub fn z(&self) -> i64 {
        self.get(&Val::VarZ)
    }

    pub fn w(&self) -> i64 {
        self.get(&Val::VarW)
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Computer {
    program: Program,
}

impl Computer {
    pub fn run(&self, input: &mut Input, program: &Program) -> Result<Output> {
        let mut out = Output::default();

        let mut cur_input = 0;
        for op in program.iter() {
            if let OpCode::RW(_) = op {
                cur_input = input
                    .next()
                    .ok_or_else(|| anyhow!("unexpected end of input"))?;
            }
            op.execute(cur_input, &mut out)?;
        }

        Ok(out)
    }

    pub fn explore(&self, program: &Program, largest: bool) -> Result<i64> {
        let output = Output::default();
        let mut cache = FxHashMap::default();
        let digits = if largest {
            [9, 8, 7, 6, 5, 4, 3, 2, 1]
        } else {
            [1, 2, 3, 4, 5, 6, 7, 8, 9]
        };

        let res = self.recur(1, program, &output, &mut cache, &digits)?;
        let mut backward = res.ok_or_else(|| anyhow!("did not find a solution"))?;
        let mut ans = 0;
        loop {
            ans = ans * 10 + backward % 10;
            backward /= 10;
            if backward == 0 {
                break;
            }
        }

        Ok(ans)
    }

    fn recur(
        &self,
        inst_pointer: usize,
        program: &Program,
        output: &Output,
        cache: &mut FxHashMap<(i64, usize), Option<i64>>,
        digits: &[i64; 9],
    ) -> Result<Option<i64>> {
        if let Some(v) = cache.get(&(output.z(), inst_pointer)) {
            return Ok(*v);
        }

        'digits: for digit in digits.iter() {
            // let mut working = output.clone();
            let mut working = *output;
            working.set(&Val::VarW, *digit)?;

            let mut new_pointer = inst_pointer;
            loop {
                // if we're at the end of the program, we want to check the value
                // of z
                if new_pointer >= program.len() {
                    if working.z() == 0 {
                        cache.insert((0, inst_pointer), Some(*digit));
                        return Ok(Some(*digit));
                    }
                    continue 'digits;
                }

                if let OpCode::RW(_) = program[new_pointer] {
                    break;
                }

                program[new_pointer].execute(0, &mut working)?;
                new_pointer += 1;
            }

            if let Some(val) = self.recur(new_pointer + 1, program, &working, cache, digits)? {
                let cur = Some(val * 10 + digit);
                cache.insert((working.z(), inst_pointer), cur);
                return Ok(cur);
            }
        }

        // if we have exhausted all the search digits and we did not find a
        // solution, there is no solution for this set
        cache.insert((output.z(), inst_pointer), None);
        Ok(None)
    }
}

impl TryFrom<Vec<String>> for Computer {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        Ok(Self {
            program: Program::try_from(&value)?,
        })
    }
}

impl Solver for Computer {
    const ID: &'static str = "arithmetic logic unit";
    const DAY: usize = 24;

    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Self::P1 {
        self.explore(&self.program, true)
            .expect("could not solve program")
    }

    fn part_two(&mut self) -> Self::P1 {
        self.explore(&self.program, false)
            .expect("could not solve program")
    }
}

#[derive(Debug, Clone, Default)]
pub struct PrecompiledSolver {
    blocks: Vec<Vec<OpCode>>,
}

impl PrecompiledSolver {
    pub fn solve_digits(&self, digits: &mut [i64]) -> Result<u64> {
        if digits.len() != self.blocks.len() {
            bail!("there must be the same number of digits as blocks");
        }

        let mut stack = Vec::with_capacity(14);

        for i in 0..digits.len() {
            let (a, b, c) = self.extract_vars(i)?;

            if a == 1 {
                stack.push((i, c));
            } else {
                let (j, c) = stack.pop()
                    .ok_or_else(|| anyhow!("attempted to pop empty stack!"))?;

                digits[i] = digits[j] + b + c;

                if digits[i] > 9 {
                    digits[j] -= digits[i] % 9;
                    digits[i] = 9;
                } else if digits[i] < 1 {
                    digits[j] += 1 - digits[i];
                    digits[i] = 1;
                }
            }
        }

        Ok(digits.iter().fold(0, |acc, d| acc * 10 + *d as u64))
    }

    pub fn extract_vars(&self, block_idx: usize) -> Result<(i64, i64, i64)> {
        let block = self.blocks
            .get(block_idx)
            .ok_or_else(|| anyhow!("no block {}", block_idx))?;
        let mut vars = (0, 0, 0);

        if let Some(OpCode::Div(_, Val::Raw(a))) = block.get(4) {
            if *a != 26 && *a != 1 {
                bail!("block {} contains invalid 'A' value", block_idx);
            }

            vars.0 = *a;
        } else {
            bail!("block {} does not contain 'A' value", block_idx);
        }

        if let Some(OpCode::Add(_, Val::Raw(b))) = block.get(5) {
            vars.1 = *b;
        } else {
            bail!("block {} does not contain 'B' value", block_idx);
        }

        if let Some(OpCode::Add(_, Val::Raw(c))) = block.get(15) {
            vars.2 = *c;
        } else {
            bail!("block {} does not contain 'C' value", block_idx);
        }

        Ok(vars)
    }
}

impl TryFrom<Vec<String>> for PrecompiledSolver {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let lines = value
            .iter()
            .map(|v| OpCode::from_str(v))
            .collect::<Result<Vec<OpCode>>>()?;

        let mut blocks = Vec::with_capacity(14);

        for chunk in &lines.into_iter().chunks(18) {
            let block = chunk.collect::<Vec<_>>();
            if block.len() != 18 {
                bail!("input contains invalid or not enough blocks");
            }

            blocks.push(block);
        }

        if blocks.len() != 14 {
            bail!("incorrect number of blocks from input {}", blocks.len());
        }

        Ok(Self {blocks})
    }
}

impl Solver for PrecompiledSolver {
    const ID: &'static str = "arithmetic logic unit";
    const DAY: usize = 24;

    type P1 = u64;
    type P2 = u64;

    fn part_one(&mut self) -> Self::P1 {
        let mut digits = [9_i64; 14];
        self.solve_digits(&mut digits).expect("could not solve program")
    }

    fn part_two(&mut self) -> Self::P1 {
        let mut digits = [1_i64; 14];
        self.solve_digits(&mut digits).expect("could not solve program")
    }
}

#[cfg(test)]
mod tests {
    use aoc_helpers::util::test_input;

    use super::*;

    #[test]
    fn system_verification() {
        let lines = test_input(
            "
            inp w
            add z w
            mod z 2
            div w 2
            add y w
            mod y 2
            div w 2
            add x w
            mod x 2
            div w 2
            mod w 2
            ",
        );
        let mut input = Input::new(0b110);
        let program = Program::try_from(&lines).expect("could not load program");
        let c = Computer { program };

        let output = c
            .run(&mut input, &c.program)
            .expect("program did not exit correctly");

        assert_eq!(output.z(), 0);
        assert_eq!(output.y(), 1);
        assert_eq!(output.x(), 1);
        assert_eq!(output.w(), 0);
    }
}

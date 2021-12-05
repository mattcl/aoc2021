use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::{convert::TryFrom, iter::FromIterator, num::ParseIntError, str::FromStr};

use anyhow::{anyhow, bail, Result};
use rayon::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Sequence {
    values: Vec<i64>,
}

impl FromStr for Sequence {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parsed: Vec<i64> = s
            .split(',')
            .map(|sub| sub.parse())
            .collect::<Result<Vec<i64>, ParseIntError>>()?;

        Ok(Sequence { values: parsed })
    }
}

#[derive(Debug, Clone, Default)]
struct Cell {
    pub marked: bool,
    pub row: usize,
    pub col: usize,
}

impl Cell {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            marked: false,
            row,
            col,
        }
    }

    pub fn mark(&mut self) {
        self.marked = true;
    }

    pub fn marked(&self) -> bool {
        self.marked
    }
}

pub trait BingoLike {
    fn attempt_to_mark(&mut self, num: i64);
    fn marked(&self, num: i64) -> bool;
    fn won(&self) -> bool;
    fn unmarked_sum(&self) -> i64;
}

#[derive(Debug, Clone, Default)]
pub struct Board {
    side: usize,
    values: HashMap<i64, Cell>,
    ordering: Vec<i64>,
    won: bool,
}

impl Board {
    fn internal_marked(&self, value: i64) -> bool {
        self.values
            .get(&value)
            .map(|cell| cell.marked())
            .unwrap_or(false)
    }

    fn check_win(&self, row: usize, col: usize) -> bool {
        self.check_row(row) || self.check_col(col)
    }

    fn check_row(&self, row: usize) -> bool {
        let count = (0..self.side)
            .filter_map(|i| {
                self.get(row, i)
                    .and_then(|v| if self.internal_marked(*v) { Some(()) } else { None })
            })
            .count();

        count == self.side
    }

    fn check_col(&self, col: usize) -> bool {
        let count = (0..self.side)
            .filter_map(|i| {
                self.get(i, col)
                    .and_then(|v| if self.internal_marked(*v) { Some(()) } else { None })
            })
            .count();

        count == self.side
    }

    fn get(&self, row: usize, col: usize) -> Option<&i64> {
        self.ordering.get(row * self.side + col)
    }
}

impl BingoLike for Board {
    fn attempt_to_mark(&mut self, num: i64) {
        if let Entry::Occupied(entry) = self.values.entry(num).and_modify(|e| e.mark()) {
            // This is to avoid the second mutable borrow
            let cell = entry.get();
            let row = cell.row;
            let col = cell.col;
            if self.check_win(row, col) {
                self.won = true;
            }
        }
    }

    fn marked(&self, num: i64) -> bool {
        self.internal_marked(num)
    }

    fn unmarked_sum(&self) -> i64 {
        self.values
            .iter()
            .map(|(v, cell)| if !cell.marked() { *v } else { 0 })
            .sum()
    }

    fn won(&self) -> bool {
        self.won
    }
}

impl TryFrom<&[String]> for Board {
    type Error = anyhow::Error;

    fn try_from(value: &[String]) -> Result<Self> {
        if value.is_empty() {
            bail!("Cannot construct a board from empty value");
        }

        let ordering: Vec<i64> = value
            .iter()
            .map(|v| {
                v.split_whitespace()
                    .map(|s| s.parse())
                    .collect::<Vec<std::result::Result<i64, ParseIntError>>>()
            })
            .flatten()
            .collect::<std::result::Result<Vec<i64>, ParseIntError>>()?;

        let side = (ordering.len() as f64).sqrt() as usize;

        let values = HashMap::from_iter(
            ordering
                .iter()
                .enumerate()
                .map(|(i, v)| (*v, Cell::new(i / side, i % side))),
        );

        Ok(Board {
            side,
            values,
            ordering,
            won: false,
        })
    }
}

#[derive(Debug, Clone)]
pub struct FastBoard {
    cells: HashMap<i64, Cell>,
    score: i64,
    won: bool,
    rows: Vec<usize>,
    cols: Vec<usize>,
}

impl FastBoard {
    pub fn new(values: &[i64]) -> Self {
        let side = (values.len() as f64).sqrt() as usize;
        let score = values.iter().sum();
        let cells = HashMap::from_iter(
            values
                .iter()
                .enumerate()
                .map(|(i, v)| (*v, Cell::new(i / side, i % side))),
        );

        Self {
            cells,
            score,
            won: false,
            rows: vec![0; side],
            cols: vec![0; side]
        }
    }
}

impl BingoLike for FastBoard {
    fn attempt_to_mark(&mut self, num: i64) {
        if self.won() {
            return;
        }

        if let Some(cell) = self.cells.get_mut(&num) {
            if cell.marked() {
                return;
            }
            cell.mark();
            let row = cell.row;
            let col = cell.col;

            self.rows[row] += 1;
            self.cols[col] += 1;

            if self.rows[row] == 5 || self.cols[col] == 5 {
                self.won = true;
            }

            self.score -= num;
        }
    }

    fn marked(&self, num: i64) -> bool {
        self.cells
            .get(&num)
            .map(|cell| cell.marked())
            .unwrap_or(false)
    }

    fn unmarked_sum(&self) -> i64 {
        self.score
    }

    fn won(&self) -> bool {
        self.won
    }
}

impl TryFrom<&[String]> for FastBoard {
    type Error = anyhow::Error;

    fn try_from(value: &[String]) -> Result<Self> {
        if value.is_empty() {
            bail!("Cannot construct a board from empty value");
        }

        let values: Vec<i64> = value
            .iter()
            .map(|v| {
                v.split_whitespace()
                    .map(|s| s.parse())
                    .collect::<Vec<std::result::Result<i64, ParseIntError>>>()
            })
            .flatten()
            .collect::<std::result::Result<Vec<i64>, ParseIntError>>()?;

        Ok(FastBoard::new(&values))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Runner<T> where T: BingoLike + Send + Sync {
    sequence: Sequence,
    boards: Vec<T>,
}

impl<T> Runner<T> where T: BingoLike + Send + Sync {
    pub fn play(&mut self) -> Result<i64> {
        for v in &self.sequence.values {
            for board in self.boards.iter_mut() {
                board.attempt_to_mark(*v);
                if board.won() {
                    return Ok(board.unmarked_sum() * v);
                }
            }
        }
        bail!("No winner could be determined")
    }

    pub fn play_all(&mut self) -> Vec<i64> {
        let mut scores = Vec::new();

        for v in &self.sequence.values {
            for board in self.boards.iter_mut() {
                if !board.won() {
                    board.attempt_to_mark(*v);
                    if board.won() {
                        scores.push(board.unmarked_sum() * v);
                    }
                }
            }
        }

        scores
    }

    pub fn par_find_last_scoring(&mut self) -> Result<i64> {
        let seq = self.sequence.values.clone();
        let mut res = self
            .boards
            .par_iter_mut()
            .enumerate()
            .filter_map(|(b_idx, board)| {
                for (i, v) in seq.iter().enumerate() {
                    board.attempt_to_mark(*v);
                    if board.won() {
                        return Some((i, b_idx));
                    }
                }
                None
            })
            .collect::<Vec<(usize, usize)>>();
        res.sort_by(|a, b| a.0.cmp(&b.0));

        res.last()
            .map(|(s_idx, b_idx)| self.sequence.values[*s_idx] * self.boards[*b_idx].unmarked_sum())
            .ok_or_else(|| anyhow!("Could not determine last winner because list is empty"))
    }
}

impl TryFrom<Vec<String>> for Runner<Board> {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut chunks = value.split(|elem| elem.is_empty());
        let first = chunks
            .next()
            .ok_or_else(|| anyhow!("Invalid input missing sequence"))?;
        if first.is_empty() {
            bail!("Invalid input, missing sequence despite chunk present");
        }

        let sequence = Sequence::from_str(&first[0])?;

        // the remaining chunks should all be boards
        let boards = chunks
            .map(Board::try_from)
            .collect::<Result<Vec<Board>>>()?;

        Ok(Runner { sequence, boards })
    }
}

impl TryFrom<Vec<String>> for Runner<FastBoard> {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut chunks = value.split(|elem| elem.is_empty());
        let first = chunks
            .next()
            .ok_or_else(|| anyhow!("Invalid input missing sequence"))?;
        if first.is_empty() {
            bail!("Invalid input, missing sequence despite chunk present");
        }

        let sequence = Sequence::from_str(&first[0])?;

        // the remaining chunks should all be boards
        let boards = chunks
            .map(FastBoard::try_from)
            .collect::<Result<Vec<FastBoard>>>()?;

        Ok(Runner { sequence, boards })
    }
}

#[cfg(test)]
mod tests {
    mod sequence {
        use super::super::*;

        #[test]
        fn creation() {
            let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1";
            let expected: Vec<i64> = vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ];

            let s = Sequence::from_str(input).expect("Could not make board");

            assert_eq!(s.values, expected);
        }
    }

    mod board {
        use crate::util::test_input;

        use super::super::*;

        use std::convert::TryFrom;

        #[test]
        fn marked() {
            let input = test_input(
                "
                14 21 17 24  4
                10 16 15  9 19
                18  8 23 26 20
                22 11 13  6  5
                 2  0 12  3  7
                ",
            );
            let slice = input.as_slice();
            let mut board = Board::try_from(slice).expect("Could not make board");
            assert_eq!(board.marked(9), false);
            assert_eq!(board.marked(1000), false); // missing values are treated as false

            board.attempt_to_mark(9);
            assert_eq!(board.marked(9), true);
        }

        #[test]
        fn unmarked_sum() {
            let input = test_input(
                "
                14 21 17 24  4
                10 16 15  9 19
                18  8 23 26 20
                22 11 13  6  5
                 2  0 12  3  7
                ",
            );
            let slice = input.as_slice();
            let mut board = Board::try_from(slice).expect("Could not make board");
            for v in vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24] {
                board.attempt_to_mark(v);
            }

            assert!(board.won);
            assert_eq!(board.unmarked_sum(), 188);
        }
    }

    mod fast_board {
        use crate::util::test_input;

        use super::super::*;

        use std::convert::TryFrom;

        #[test]
        fn marked() {
            let input = test_input(
                "
                14 21 17 24  4
                10 16 15  9 19
                18  8 23 26 20
                22 11 13  6  5
                 2  0 12  3  7
                ",
            );
            let slice = input.as_slice();
            let mut board = FastBoard::try_from(slice).expect("Could not make board");
            assert_eq!(board.marked(9), false);
            assert_eq!(board.marked(1000), false); // missing values are treated as false

            board.attempt_to_mark(9);
            assert_eq!(board.marked(9), true);
        }

        #[test]
        fn unmarked_sum() {
            let input = test_input(
                "
                14 21 17 24  4
                10 16 15  9 19
                18  8 23 26 20
                22 11 13  6  5
                 2  0 12  3  7
                ",
            );
            let slice = input.as_slice();
            let mut board = FastBoard::try_from(slice).expect("Could not make board");
            for v in vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24] {
                board.attempt_to_mark(v);
            }

            assert!(board.won());
            assert_eq!(board.unmarked_sum(), 188);
        }
    }

    mod runner {
        use crate::util::test_input;

        use super::super::*;

        use std::convert::TryFrom;

        fn input() -> Vec<String> {
            test_input(
                "
                7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

                22 13 17 11  0
                 8  2 23  4 24
                21  9 14 16  7
                 6 10  3 18  5
                 1 12 20 15 19

                 3 15  0  2 22
                 9 18 13 17  5
                19  8  7 25 23
                20 11 10 24  4
                14 21 16 12  6

                14 21 17 24  4
                10 16 15  9 19
                18  8 23 26 20
                22 11 13  6  5
                 2  0 12  3  7
                ",
            )
        }

        #[test]
        fn finding_first_win() {
            let input = input();

            let mut runner: Runner<Board> = Runner::try_from(input.clone()).expect("Could not construct runner");
            let score = runner.play().expect("Did not find a winner");
            assert_eq!(score, 4512);

            let mut runner: Runner<FastBoard> = Runner::try_from(input).expect("Could not construct runner");
            let score = runner.play().expect("Did not find a winner");
            assert_eq!(score, 4512);
        }

        #[test]
        fn finding_all_wins() {
            let input = input();

            let mut runner: Runner<Board> = Runner::try_from(input.clone()).expect("Could not construct runner");
            let scores = runner.play_all();
            assert_eq!(scores.last().cloned(), Some(1924));

            let mut runner: Runner<FastBoard> = Runner::try_from(input).expect("Could not construct runner");
            let scores = runner.play_all();
            assert_eq!(scores.last().cloned(), Some(1924));
        }

        #[test]
        fn finding_all_wins_in_parallel() {
            let input = input();

            let mut runner: Runner<Board> = Runner::try_from(input.clone()).expect("Could not construct runner");
            let score = runner
                .par_find_last_scoring()
                .expect("Could not find last scoring");
            assert_eq!(score, 1924);

            let mut runner: Runner<FastBoard> = Runner::try_from(input).expect("Could not construct runner");
            let score = runner
                .par_find_last_scoring()
                .expect("Could not find last scoring");
            assert_eq!(score, 1924);
        }
    }
}

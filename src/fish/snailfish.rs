use std::{
    convert::TryFrom,
    fmt,
    ops::{Add, AddAssign},
    str::FromStr,
};

use anyhow::anyhow;
use aoc_helpers::Solver;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    sequence::{delimited, separated_pair},
    IResult,
};
use rayon::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Element {
    Num(i64),
    Pair(Box<Pair>),
}

impl Element {
    pub fn split(&self) -> Option<Self> {
        match self {
            Self::Num(v) if *v > 9 => Some(Self::Pair(Box::new(Pair::new(
                (v / 2).into(),
                ((*v as f64 / 2.0).ceil() as i64).into(),
            )))),
            _ => None,
        }
    }

    pub fn right_add_assign(&mut self, other: i64) {
        match self {
            Self::Num(ref mut v) => *v += other,
            Self::Pair(ref mut p) => p.right.right_add_assign(other),
        }
    }

    pub fn magnitude(&self) -> i64 {
        match self {
            Self::Num(v) => *v,
            Self::Pair(p) => p.magnitude(),
        }
    }
}

impl AddAssign<i64> for Element {
    fn add_assign(&mut self, other: i64) {
        match self {
            Self::Num(ref mut v) => *v += other,
            Self::Pair(ref mut p) => p.left += other,
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(v) => write!(f, "{}", v),
            Self::Pair(p) => write!(f, "{}", p),
        }
    }
}

impl From<i64> for Element {
    fn from(v: i64) -> Self {
        Self::Num(v)
    }
}

impl From<Pair> for Element {
    fn from(v: Pair) -> Self {
        Self::Pair(Box::new(v))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pair {
    left: Element,
    right: Element,
}

impl Pair {
    pub fn new(left: Element, right: Element) -> Self {
        Self { left, right }
    }

    pub fn magnitude(&self) -> i64 {
        self.left.magnitude() * 3 + self.right.magnitude() * 2
    }

    pub fn reduce(&mut self) {
        let mut action_taken = false;
        loop {
            // explode first
            self.recur_explode(0, &mut action_taken);

            // if we didn't explode something, check for splits
            if !action_taken {
                // if we didn't explode or split something, we're done
                if !self.recur_split() {
                    break;
                }
            }

            // reset this to false for the next loop
            action_taken = false;
        }
    }

    fn recur_explode(&mut self, depth: usize, action_taken: &mut bool) -> Option<(i64, i64)> {
        if *action_taken {
            return None;
        }

        if depth >= 4 {
            *action_taken = true;
            // So I don't know if I want to deal with the case where the element
            // here nests deeper. Just return None in these cases for now
            let l_val = match self.left {
                Element::Num(v) => v,
                _ => return None,
            };

            let r_val = match self.right {
                Element::Num(v) => v,
                _ => return None,
            };

            return Some((l_val, r_val));
        }

        match self.left {
            Element::Num(_) => {}
            Element::Pair(ref mut p) => {
                if let Some((l, r)) = p.recur_explode(depth + 1, action_taken) {
                    if depth == 3 {
                        self.left = Element::Num(0);
                    }

                    if l == 0 && r == 0 {
                        return None;
                    }

                    if r > 0 {
                        self.right += r;
                    }

                    // if we're left, we always have a right. So we know we've
                    // already appied the addition for the right value
                    return Some((l, 0));
                }
            }
        }

        match self.right {
            Element::Num(_) => {}
            Element::Pair(ref mut p) => {
                if let Some((l, r)) = p.recur_explode(depth + 1, action_taken) {
                    if depth == 3 {
                        self.right = Element::Num(0);
                    }

                    if l == 0 && r == 0 {
                        return None;
                    }

                    if l > 0 {
                        // okay, so what happens if the left value is a pair?
                        // if so, we want to add to the rightmost value in that pair
                        self.left.right_add_assign(l);
                    }

                    // if we're right, we always have a left. So we know we've
                    // already appied the addition for the left value
                    return Some((0, r));
                }
            }
        }

        None
    }

    fn recur_split(&mut self) -> bool {
        match self.left {
            Element::Num(_) => {
                if let Some(s) = self.left.split() {
                    self.left = s;
                    return true;
                }
            }
            Element::Pair(ref mut p) => {
                if p.recur_split() {
                    return true;
                }
            }
        }

        match self.right {
            Element::Num(_) => {
                if let Some(s) = self.right.split() {
                    self.right = s;
                    return true;
                }
            }
            Element::Pair(ref mut p) => {
                if p.recur_split() {
                    return true;
                }
            }
        }

        false
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

impl Add<&Pair> for Pair {
    type Output = Pair;

    fn add(self, rhs: &Pair) -> Self::Output {
        let mut sum = Self::new(
            Element::Pair(Box::new(self)),
            Element::Pair(Box::new(rhs.clone())),
        );
        sum.reduce();
        sum
    }
}

impl Add<&Pair> for &Pair {
    type Output = Pair;

    fn add(self, rhs: &Pair) -> Self::Output {
        let mut sum = Pair::new(
            Element::Pair(Box::new(self.clone())),
            Element::Pair(Box::new(rhs.clone())),
        );
        sum.reduce();
        sum
    }
}

impl Add<Pair> for Pair {
    type Output = Pair;

    fn add(self, rhs: Pair) -> Self::Output {
        let mut sum = Self::new(Element::Pair(Box::new(self)), Element::Pair(Box::new(rhs)));
        sum.reduce();
        sum
    }
}

impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (_, p) = parse_pair(s).map_err(|_| anyhow!("Unable to parse pair from: {}", s))?;
        Ok(p)
    }
}

// nom parsers
fn parse_num(input: &str) -> IResult<&str, Element> {
    let (input, v) = map_res(digit1, i64::from_str)(input)?;
    Ok((input, Element::Num(v)))
}

fn parse_elem_pair(input: &str) -> IResult<&str, Element> {
    let (input, v) = parse_pair(input)?;
    Ok((input, Element::Pair(v.into())))
}

fn parse_elem(input: &str) -> IResult<&str, Element> {
    alt((parse_num, parse_elem_pair))(input)
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    let (input, (left, right)) = delimited(
        tag("["),
        separated_pair(parse_elem, tag(","), parse_elem),
        tag("]"),
    )(input)?;

    Ok((input, Pair::new(left, right)))
}

#[derive(Debug, Clone)]
pub struct Homework {
    pairs: Vec<Pair>,
}

impl Homework {
    pub fn sum(&self) -> Option<Pair> {
        let mut iter = self.pairs.iter();
        let first = iter.next()?;
        Some(iter.fold(first.clone(), |acc, p| acc + p))
    }

    pub fn largest_magnitude_of_pairs(&self) -> Option<i64> {
        if self.pairs.is_empty() {
            return None;
        }

        self.pairs
            .iter()
            .permutations(2)
            .par_bridge()
            .map(|pair| (pair[0] + pair[1]).magnitude())
            .max()
    }
}

impl TryFrom<Vec<String>> for Homework {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> anyhow::Result<Self> {
        let pairs = value
            .iter()
            .map(|s| Pair::from_str(s))
            .collect::<anyhow::Result<Vec<Pair>>>()?;
        Ok(Self { pairs })
    }
}

impl Solver for Homework {
    const ID: &'static str = "snailfish";
    const DAY: usize = 18;

    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Self::P1 {
        self.sum().expect("could not find sum").magnitude()
    }

    fn part_two(&mut self) -> Self::P2 {
        self.largest_magnitude_of_pairs()
            .expect("could not largest magnitude")
    }
}

#[cfg(test)]
mod tests {
    mod element {
        use super::super::*;

        #[test]
        fn split() {
            let e = Element::Num(10);
            let s = e.split().unwrap();
            assert_eq!(s, Element::Pair(Box::new(Pair::new(5.into(), 5.into()))));

            let e = Element::Num(11);
            let s = e.split().unwrap();
            assert_eq!(s, Element::Pair(Box::new(Pair::new(5.into(), 6.into()))));
        }
    }

    mod pair {
        use super::super::*;

        #[test]
        fn parsing() {
            let input = "[[[[[9,8],1],2],3],4]";
            let p = Pair::from_str(input).expect("could not parse pair");
            assert_eq!(p.to_string(), input);
        }

        #[test]
        fn addition() {
            let p1 = Pair::new(1.into(), 2.into());
            let p2 = Pair::new(Pair::new(3.into(), 4.into()).into(), 5.into());
            let expected = Pair::new(p1.clone().into(), p2.clone().into());
            let r = p1 + p2;
            assert_eq!(r, expected);

            // addition reduces
            let p1 = Pair::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]").expect("could not parse pair");
            let p2 = Pair::from_str("[1,1]").expect("could not parse pair");
            let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
            let r = p1 + p2;
            assert_eq!(r.to_string(), expected);
        }

        #[test]
        fn magnitude() {
            let input = "[[1,2],[[3,4],5]]";
            let p = Pair::from_str(input).expect("could not parse pair");
            assert_eq!(p.magnitude(), 143);

            let input = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
            let p = Pair::from_str(input).expect("could not parse pair");
            assert_eq!(p.magnitude(), 3488);
        }

        #[test]
        fn reduce() {
            let input = "[[[[[9,8],1],2],3],4]";
            let mut p = Pair::from_str(input).expect("could not parse pair");
            let expected = "[[[[0,9],2],3],4]";

            p.reduce();
            assert_eq!(p.to_string(), expected);

            let input = "[7,[6,[5,[4,[3,2]]]]]";
            let mut p = Pair::from_str(input).expect("could not parse pair");
            let expected = "[7,[6,[5,[7,0]]]]";

            p.reduce();
            assert_eq!(p.to_string(), expected);

            let input = "[[6,[5,[4,[3,2]]]],1]";
            let mut p = Pair::from_str(input).expect("could not parse pair");
            let expected = "[[6,[5,[7,0]]],3]";

            p.reduce();
            assert_eq!(p.to_string(), expected);

            let input = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
            let mut p = Pair::from_str(input).expect("could not parse pair");
            let expected = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]";

            p.reduce();
            assert_eq!(p.to_string(), expected);

            // has explodes and splits
            let input = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
            let mut p = Pair::from_str(input).expect("could not parse pair");
            let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";

            p.reduce();
            assert_eq!(p.to_string(), expected);
        }
    }

    mod homework {
        use aoc_helpers::util::test_input;

        use super::super::*;

        #[test]
        fn sum() {
            let input = test_input(
                "
                [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
                [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
                [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
                [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
                [7,[5,[[3,8],[1,4]]]]
                [[2,[2,2]],[8,[8,1]]]
                [2,9]
                [1,[[[9,3],9],[[9,0],[0,7]]]]
                [[[5,[7,4]],7],1]
                [[[[4,2],2],6],[8,7]]
                ",
            );
            let homework = Homework::try_from(input).expect("could not parse input");
            let expected = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";

            let s = homework.sum().expect("No sum calculated");

            assert_eq!(s.to_string(), expected);
        }

        #[test]
        fn largest_magnitude_of_pairs() {
            let input = test_input(
                "
                [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
                [[[5,[2,8]],4],[5,[[9,9],0]]]
                [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
                [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
                [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
                [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
                [[[[5,4],[7,7]],8],[[8,3],8]]
                [[9,3],[[9,9],[6,[4,9]]]]
                [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
                [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
                ",
            );
            let homework = Homework::try_from(input).expect("could not parse input");
            let m = homework
                .largest_magnitude_of_pairs()
                .expect("No magnitude calculated");

            assert_eq!(m, 3993);
        }
    }
}

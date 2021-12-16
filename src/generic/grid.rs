use std::{convert::TryFrom, fmt::Debug};

use anyhow::bail;

use super::Location;

pub trait GridLike {
    type Item;

    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn get(&self, location: &Location) -> Option<&Self::Item>;

    fn top_left(&self) -> Location {
        Location::default()
    }

    fn bottom_right(&self) -> Location {
        Location::new(self.rows() - 1, self.cols() - 1)
    }

    fn size(&self) -> usize {
        self.rows() * self.cols()
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

pub trait Scalable: GridLike {
    fn scaled_bottom_right(&self, scale: usize) -> Location {
        Location::new(self.rows() * scale - 1, self.cols() * scale - 1)
    }

    fn get_scaled<F>(
        &self,
        location: &Location,
        scale: usize,
        scale_fn: F,
    ) -> Option<<Self as GridLike>::Item>
    where
        F: Fn(&<Self as GridLike>::Item, usize, usize) -> <Self as GridLike>::Item,
    {
        // we're out of bounds here
        let r_fac = location.row / self.rows();
        let c_fac = location.col / self.cols();
        if r_fac >= scale || c_fac >= scale {
            return None;
        }

        let row = location.row % self.rows();
        let col = location.col % self.cols();
        self.get(&Location::new(row, col))
            .map(|v| scale_fn(v, r_fac, c_fac))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Grid<T> {
    pub locations: Vec<Vec<T>>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> GridLike for Grid<T>
where
    T: Debug + Clone,
{
    type Item = T;

    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn get(&self, location: &Location) -> Option<&Self::Item> {
        self.locations
            .get(location.row)
            .and_then(|r| r.get(location.col))
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T>
where
    T: Debug + Clone,
{
    type Error = anyhow::Error;

    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let rows = value.len();
        let cols = value.get(0).map(|c| c.len()).unwrap_or_default();

        if value.iter().any(|c| c.len() != cols) {
            bail!("Not all rows are the same length");
        }

        Ok(Self {
            locations: value,
            rows,
            cols,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type GTest = Grid<usize>;
    impl Scalable for GTest {}

    #[test]
    fn general() {
        let empty = GTest::default();
        assert!(empty.is_empty());

        let values: Vec<Vec<usize>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let grid = GTest::try_from(values).expect("could not construct grid");
        assert!(!grid.is_empty());
        assert_eq!(grid.size(), 12);
        assert_eq!(grid.rows(), 3);
        assert_eq!(grid.cols(), 4);

        assert_eq!(grid.get(&Location::new(0, 0)), Some(&1));
        assert_eq!(grid.get(&Location::new(2, 3)), Some(&12));
        assert_eq!(grid.get(&Location::new(1, 2)), Some(&7));

        assert_eq!(grid.top_left(), Location::new(0, 0));
        assert_eq!(grid.bottom_right(), Location::new(2, 3));
    }

    #[test]
    fn scale() {
        let values: Vec<Vec<usize>> = vec![vec![8]];
        let grid = GTest::try_from(values).expect("could not construct grid");
        let scale = 5;
        let scale_fn = |num: &usize, r_fac, c_fac| {
            let mut v = num + r_fac + c_fac;
            if v > 9 {
                v = v % 10 + 1;
            }
            v
        };

        assert_eq!(
            grid.get_scaled(&Location::new(0, 0), scale, scale_fn),
            Some(8)
        );
        assert_eq!(
            grid.get_scaled(&Location::new(1, 1), scale, scale_fn),
            Some(1)
        );
        assert_eq!(
            grid.get_scaled(&Location::new(1, 4), scale, scale_fn),
            Some(4)
        );
        assert_eq!(
            grid.get_scaled(&Location::new(2, 2), scale, scale_fn),
            Some(3)
        );
        assert_eq!(
            grid.get_scaled(&Location::new(3, 3), scale, scale_fn),
            Some(5)
        );
        assert_eq!(
            grid.get_scaled(&Location::new(4, 4), scale, scale_fn),
            Some(7)
        );
    }
}

use std::{collections::HashMap, hash::Hash};

use array2d::Array2D;

pub trait CountOccurences<T> {
    fn count_occurences(self) -> HashMap<T, usize>;
}

impl<T, U> CountOccurences<T> for U
where
    U: IntoIterator<Item = T>,
    T: Hash + Eq,
{
    fn count_occurences(self) -> HashMap<T, usize> {
        self.into_iter()
            .fold(HashMap::new(), |mut counts, occurence| {
                *counts.entry(occurence).or_default() += 1;
                counts
            })
    }
}

pub struct Neighbours<'a, T> {
    top_left: Option<&'a T>,
    up: Option<&'a T>,
    top_right: Option<&'a T>,
    left: Option<&'a T>,
    centre: &'a T,
    right: Option<&'a T>,
    bottom_left: Option<&'a T>,
    down: Option<&'a T>,
    bottom_right: Option<&'a T>,
}

pub struct NeighboursMut<'a, T> {
    top_left: Option<&'a mut T>,
    up: Option<&'a mut T>,
    top_right: Option<&'a mut T>,
    left: Option<&'a mut T>,
    centre: &'a mut T,
    right: Option<&'a mut T>,
    bottom_left: Option<&'a mut T>,
    down: Option<&'a mut T>,
    bottom_right: Option<&'a mut T>,
}

// TODO update smoke_basin to use this
pub trait Neighbouring<T> {
    fn neighbours(&self, row: usize, column: usize) -> Option<Neighbours<T>>;
    fn neighbours_mut(&mut self, row: usize, column: usize) -> Option<NeighboursMut<T>>;
}

impl<T: Clone> Neighbouring<T> for Array2D<T> {
    fn neighbours(&self, row: usize, column: usize) -> Option<Neighbours<T>> {
        let mut top_left = None;
        let mut up = None;
        let mut top_right = None;
        let mut left = None;
        let centre = self.get(row, column)?;
        let right = self.get(row, column + 1);
        let mut bottom_left = None;
        let down = self.get(row + 1, column);
        let bottom_right = self.get(row + 1, column + 1);

        if row > 0 && column > 0 {
            top_left = self.get(row - 1, column - 1)
        }
        if row > 0 {
            up = self.get(row - 1, column);
            top_right = self.get(row - 1, column + 1);
        }
        if column > 0 {
            left = self.get(row, column - 1);
            bottom_left = self.get(row + 1, column - 1);
        }

        Some(Neighbours {
            top_left,
            up,
            top_right,
            left,
            centre,
            right,
            bottom_left,
            down,
            bottom_right,
        })
    }

    fn neighbours_mut(&mut self, row: usize, column: usize) -> Option<NeighboursMut<T>> {
        let mut top_left = None;
        let mut up = None;
        let mut top_right = None;
        let mut left = None;
        let centre = self.get_mut(row, column)?;
        let right = self.get_mut(row, column + 1);
        let mut bottom_left = None;
        let down = self.get_mut(row + 1, column);
        let bottom_right = self.get_mut(row + 1, column + 1);

        if row > 0 && column > 0 {
            top_left = self.get_mut(row - 1, column - 1)
        }
        if row > 0 {
            up = self.get_mut(row - 1, column);
            top_right = self.get_mut(row - 1, column + 1);
        }
        if column > 0 {
            left = self.get_mut(row, column - 1);
            bottom_left = self.get_mut(row + 1, column - 1);
        }

        Some(NeighboursMut {
            top_left,
            up,
            top_right,
            left,
            centre,
            right,
            bottom_left,
            down,
            bottom_right,
        })
    }
}
